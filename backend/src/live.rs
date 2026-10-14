//! Live updates pushed to open pages over Server-Sent Events.
//!
//! A page that shows live tournament state (Games in Progress, the Server Monitor) opens one
//! long-lived `GET .../stream` request. Each open stream is a subscription to a [`Topic`], so the
//! hub always knows which pages are being watched, and only those are ever recalculated.
//!
//! # How an update travels
//!
//! 1. Something that changes what those pages show (a game event, a ping, an operator action)
//!    calls [`tournament_changed`]. That only queues a message and returns: it never waits on
//!    a recalculation, so it cannot slow down or fail the request that triggered it.
//! 2. One background worker collects triggers for [`DEBOUNCE`], so a burst of events becomes one
//!    recalculation, and recalculates each topic at most once per [`MIN_GAP`]. A busy tournament
//!    has every room pinging every few seconds; without the gap a watched topic would recalculate
//!    nonstop.
//! 3. The fresh snapshot goes into the topic's `watch` channel, and every open stream on that
//!    topic sends it.
//!
//! Every update is a whole snapshot, never a delta, so a viewer only ever needs the latest one.
//! That is why the channels are `tokio::sync::watch`: it keeps just the newest value, so a slow
//! viewer can never fall behind or miss an update that matters.
//!
//! # The same rule as the cache
//!
//! Live updates are an improvement on polling, never a dependency. Nothing here can fail a
//! request, and a page whose stream cannot connect falls back to polling the REST endpoint.
//! The hub is in-process: it is correct while the app runs as a single process (one container).
//! If the app is ever scaled out, [`LiveHub::tournament_changed`] is the one place to swap for a
//! Valkey `PUBLISH`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{HttpRequest, HttpResponse, web::{Bytes, Data}};
use chrono::{DateTime, Utc};
use tokio::sync::{mpsc, watch};
use tokio::time::Instant;
use uuid::Uuid;

/// How long the worker waits after a trigger for more to arrive before recalculating. A question
/// produces several events in quick succession and a resend produces dozens.
pub const DEBOUNCE: Duration = Duration::from_millis(250);

/// The minimum time between two recalculations of the same topic.
pub const MIN_GAP: Duration = Duration::from_secs(1);

/// How often every watched topic is recalculated even with no trigger. Some of what the pages
/// show changes only as time passes: a room drops off Games in Progress after 30 quiet minutes,
/// and a resend the room picked up but did not complete becomes "failed" on the Server Monitor.
pub const SWEEP: Duration = Duration::from_secs(30);

/// How often an open stream sends a comment line when nothing has changed. nginx closes a proxied
/// connection after 60 s of silence (`proxy_read_timeout`), and the Cloudflare tunnel has its own
/// idle limit.
pub const KEEPALIVE: Duration = Duration::from_secs(15);

/// A page that can be watched live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Topic {
    /// The public Games in Progress tickertape, for a tournament.
    GamesInProgress(Uuid),
    /// The restricted Server Monitor (room monitor), for a tournament.
    RoomMonitor(Uuid),
    /// The public team standings of a division (its own stats group).
    ///
    /// Marked by [`LiveHub::division_changed`], not by tournament triggers: standings change only
    /// when a game event lands (or the tie-breaker rules are edited), and every room's pings would
    /// otherwise rebuild them every second for nothing.
    TeamStats(Uuid),
}

impl Topic {
    /// Every topic of a tournament. A trigger marks all of them, so a trigger never has to know
    /// which page shows which data, and no page can fall behind because a trigger forgot it.
    pub fn all_of(tid: Uuid) -> [Topic; 2] {
        [Topic::GamesInProgress(tid), Topic::RoomMonitor(tid)]
    }
}

/// Builds a topic's snapshot as JSON. `None` means it could not be built this time; viewers keep
/// the last snapshot and the next trigger or sweep tries again.
///
/// Called from a blocking thread, so it may run synchronous Diesel queries.
pub type Compute = dyn Fn(Topic) -> Option<String> + Send + Sync + 'static;

/// A snapshot, or `None` before the first one for a topic has been built.
type Snapshot = Option<Arc<str>>;

/// Every topic's channel. Shared by the hub's handles and its worker.
type Channels = Arc<Mutex<HashMap<Topic, watch::Sender<Snapshot>>>>;

/// The registry of watched topics. Cheap to clone; every clone shares the same state.
///
/// The worker holds only the channels, never a `LiveHub`, so it stops once the last handle (and
/// with it the trigger sender) is dropped.
#[derive(Clone)]
pub struct LiveHub {
    channels: Channels,
    dirty_tx: mpsc::UnboundedSender<Topic>,
}

impl LiveHub {
    /// Creates the hub and spawns its worker on the current Tokio runtime.
    pub fn start(compute: Arc<Compute>) -> LiveHub {
        Self::start_with(compute, DEBOUNCE, MIN_GAP, SWEEP)
    }

    /// [`LiveHub::start`] with the timings as parameters, so tests can run them fast.
    pub fn start_with(compute: Arc<Compute>, debounce: Duration, min_gap: Duration, sweep: Duration) -> LiveHub {
        let (dirty_tx, dirty_rx) = mpsc::unbounded_channel();
        let hub = LiveHub { channels: Arc::new(Mutex::new(HashMap::new())), dirty_tx };
        tokio::spawn(run_worker(hub.channels.clone(), dirty_rx, compute, debounce, min_gap, sweep));
        hub
    }

    /// Marks every topic of the tournament as needing a recalculation. Returns immediately.
    pub fn tournament_changed(&self, tid: Uuid) {
        if !self.is_watching_anything() {
            return;
        }
        for topic in Topic::all_of(tid) {
            // Fails only if the worker has stopped, which happens only at shutdown.
            let _ = self.dirty_tx.send(topic);
        }
    }

    /// Marks a division's standings as needing a recalculation. Returns immediately.
    pub fn division_changed(&self, did: Uuid) {
        if self.is_watching_anything() {
            let _ = self.dirty_tx.send(Topic::TeamStats(did));
        }
    }

    /// Opens a subscription. The receiver's current value is the latest snapshot, or `None` until
    /// the first one is built; a recalculation is queued either way.
    pub fn subscribe(&self, topic: Topic) -> watch::Receiver<Snapshot> {
        let rx = {
            let mut channels = self.channels.lock().unwrap();
            let tx = channels.entry(topic).or_insert_with(|| watch::channel(None).0);
            // Nobody has been watching, so no one has kept the snapshot current: drop it rather
            // than show a new viewer something hours old.
            if tx.receiver_count() == 0 {
                tx.send_replace(None);
            }
            tx.subscribe()
        };
        let _ = self.dirty_tx.send(topic);
        rx
    }

    /// How many open streams are watching `topic`.
    pub fn viewers(&self, topic: Topic) -> usize {
        viewers(&self.channels, topic)
    }

    /// Whether any stream is open at all. Lets a trigger skip the work of finding its tournament
    /// when no one is watching anything.
    pub fn is_watching_anything(&self) -> bool {
        self.channels.lock().unwrap().values().any(|tx| tx.receiver_count() > 0)
    }
}

fn viewers(channels: &Channels, topic: Topic) -> usize {
    channels.lock().unwrap().get(&topic).map_or(0, |tx| tx.receiver_count())
}

fn watched_topics(channels: &Channels) -> Vec<Topic> {
    channels.lock().unwrap().iter()
        .filter(|(_, tx)| tx.receiver_count() > 0)
        .map(|(topic, _)| *topic)
        .collect()
}

fn publish(channels: &Channels, topic: Topic, json: String) {
    if let Some(tx) = channels.lock().unwrap().get(&topic) {
        tx.send_replace(Some(Arc::from(json)));
    }
}

/// Forgets topics no one is watching, so a tournament that is over stops costing anything.
fn drop_unwatched(channels: &Channels) {
    channels.lock().unwrap().retain(|_, tx| tx.receiver_count() > 0);
}

/// The hub's single worker: combines triggers and recalculates watched topics.
async fn run_worker(
    channels: Channels,
    mut dirty_rx: mpsc::UnboundedReceiver<Topic>,
    compute: Arc<Compute>,
    debounce: Duration,
    min_gap: Duration,
    sweep: Duration,
) {
    // When each dirty topic was first marked, and when each topic was last recalculated.
    let mut dirty_since: HashMap<Topic, Instant> = HashMap::new();
    let mut last_run: HashMap<Topic, Instant> = HashMap::new();
    let mut sweep_tick = tokio::time::interval_at(Instant::now() + sweep, sweep);

    loop {
        // The soonest moment a dirty topic is allowed to run.
        let next_due = dirty_since.iter()
            .map(|(topic, since)| {
                let after_debounce = *since + debounce;
                match last_run.get(topic) {
                    Some(ran) => after_debounce.max(*ran + min_gap),
                    None => after_debounce,
                }
            })
            .min();

        tokio::select! {
            msg = dirty_rx.recv() => match msg {
                Some(topic) => { dirty_since.entry(topic).or_insert_with(Instant::now); }
                None => return, // every hub handle is gone: the server is shutting down
            },
            _ = sweep_tick.tick() => {
                drop_unwatched(&channels);
                let now = Instant::now();
                for topic in watched_topics(&channels) {
                    dirty_since.entry(topic).or_insert(now);
                }
                last_run.retain(|topic, _| viewers(&channels, *topic) > 0 || dirty_since.contains_key(topic));
            },
            _ = sleep_until_or_forever(next_due) => {
                let now = Instant::now();
                let due: Vec<Topic> = dirty_since.iter()
                    .filter(|(topic, since)| {
                        now >= **since + debounce
                            && last_run.get(*topic).is_none_or(|ran| now >= *ran + min_gap)
                    })
                    .map(|(topic, _)| *topic)
                    .collect();

                for topic in due {
                    dirty_since.remove(&topic);
                    // A trigger marks every topic of its tournament; only watched ones cost anything.
                    if viewers(&channels, topic) == 0 {
                        continue;
                    }
                    last_run.insert(topic, Instant::now());
                    let compute = compute.clone();
                    match tokio::task::spawn_blocking(move || compute(topic)).await {
                        Ok(Some(json)) => publish(&channels, topic, json),
                        Ok(None) => log::warn!("Live update for {topic:?} could not be built; viewers keep the last one."),
                        Err(e) => log::error!("Live update for {topic:?} panicked: {e}"),
                    }
                }
            },
        }
    }
}

async fn sleep_until_or_forever(deadline: Option<Instant>) {
    match deadline {
        Some(at) => tokio::time::sleep_until(at).await,
        None => std::future::pending().await,
    }
}

/// The production [`Compute`]: builds each topic from the same model function its REST endpoint
/// serves, so a stream and a poll can never disagree.
pub fn snapshot_builder(db: crate::database::Database, cache: crate::cache::Cache) -> Arc<Compute> {
    Arc::new(move |topic| {
        let mut conn = match db.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::warn!("No database connection for the live update of {topic:?}: {e}");
                return None;
            }
        };
        let json = match topic {
            // Runs on a blocking thread, which may block on the cache's async reads.
            Topic::GamesInProgress(tid) => tokio::runtime::Handle::current()
                .block_on(crate::models::room::read_games_in_progress_of_tournament(&cache, &mut conn, tid))
                .map(|rows| serde_json::to_string(&rows)),
            Topic::RoomMonitor(tid) => crate::models::room::read_room_monitor_of_tournament(&mut conn, tid)
                .map(|rows| serde_json::to_string(&rows)),
            Topic::TeamStats(did) => tokio::runtime::Handle::current()
                .block_on(crate::models::statsgroup::read_team_stats_of_division(&cache, &mut conn, did))
                .map(|rows| serde_json::to_string(&rows)),
        };
        match json {
            Ok(Ok(json)) => Some(json),
            Ok(Err(e)) => {
                log::error!("Could not serialize the live update of {topic:?}: {e}");
                None
            }
            Err(e) => {
                log::warn!("Could not build the live update of {topic:?}: {e:?}");
                None
            }
        }
    })
}

/// Marks every live topic of the tournament for a recalculation, if this app has a hub.
///
/// Takes the request rather than a `Data<LiveHub>` extractor on purpose: an app built without a
/// hub (every integration test that does not exercise live updates) must still serve the handler.
pub fn tournament_changed(req: &HttpRequest, tid: Uuid) {
    if let Some(hub) = req.app_data::<Data<LiveHub>>() {
        hub.tournament_changed(tid);
    }
}

/// [`tournament_changed`] for a trigger that knows only the tournament's name, as the
/// QuizMachine clients send it. Skips the lookup entirely when no one is watching anything.
pub fn tournament_named_changed(req: &HttpRequest, conn: &mut crate::database::Connection, tournament_name: &str) {
    let Some(hub) = req.app_data::<Data<LiveHub>>() else { return };
    if !hub.is_watching_anything() {
        return;
    }
    use crate::schema::tournaments::dsl as t;
    use diesel::prelude::*;
    if let Ok(tid) = t::tournaments.filter(t::tname.eq(tournament_name)).select(t::tid).first::<Uuid>(conn) {
        hub.tournament_changed(tid);
    }
}

/// Marks a division's live standings for a recalculation, if this app has a hub.
pub fn division_changed(req: &HttpRequest, did: Uuid) {
    if let Some(hub) = req.app_data::<Data<LiveHub>>() {
        hub.division_changed(did);
    }
}

/// Marks the live standings of the division a game belongs to, from the game id a QuizMachine
/// client sends. Skips the lookup entirely when no one is watching anything.
pub fn game_division_changed(req: &HttpRequest, conn: &mut crate::database::Connection, game_id: &str) {
    let Some(hub) = req.app_data::<Data<LiveHub>>() else { return };
    if !hub.is_watching_anything() {
        return;
    }
    let Ok(gid) = Uuid::parse_str(game_id.trim()) else { return };
    if let Ok(division) = crate::models::game::read(conn, gid)
        .and_then(|game| crate::models::game::read_division_of_game(conn, &game))
    {
        hub.division_changed(division.did);
    }
}

// ─── The stream ────────────────────────────────────────────────────────────────

/// One Server-Sent Events frame carrying a snapshot.
fn snapshot_frame(json: &str) -> Bytes {
    Bytes::from(format!("event: snapshot\ndata: {json}\n\n"))
}

const KEEPALIVE_FRAME: &[u8] = b": keepalive\n\n";

/// Sent just before the server closes a stream whose access token has expired. The page then
/// refreshes its token and reconnects.
const EXPIRED_FRAME: &[u8] = b"event: expired\ndata: {}\n\n";

/// The response for an open stream on `topic`.
///
/// Sends the current snapshot at once (if there is one yet), then one frame per new snapshot,
/// with a keepalive comment through quiet stretches. When `expires_at` is given the stream ends
/// then, so a viewer whose access has lapsed or been revoked cannot keep watching on an old token.
pub fn stream_response(hub: &LiveHub, topic: Topic, expires_at: Option<DateTime<Utc>>) -> HttpResponse {
    struct State {
        rx: watch::Receiver<Snapshot>,
        sent_first: bool,
        expires_at: Option<Instant>,
        done: bool,
    }

    let mut rx = hub.subscribe(topic);
    // The current value is sent as the first frame, so it must not also count as a change.
    rx.mark_unchanged();

    let expires_at = expires_at.map(|at| {
        let remaining = (at - Utc::now()).to_std().unwrap_or(Duration::ZERO);
        Instant::now() + remaining
    });
    let state = State { rx, sent_first: false, expires_at, done: false };

    let stream = futures_util::stream::unfold(state, |mut st| async move {
        if st.done {
            return None;
        }
        if !st.sent_first {
            st.sent_first = true;
            let current = st.rx.borrow().clone();
            if let Some(json) = current {
                return Some((Ok::<Bytes, actix_web::Error>(snapshot_frame(&json)), st));
            }
        }
        let frame = tokio::select! {
            changed = st.rx.changed() => match changed {
                Ok(()) => match st.rx.borrow_and_update().clone() {
                    Some(json) => snapshot_frame(&json),
                    None => Bytes::from_static(KEEPALIVE_FRAME),
                },
                Err(_) => return None, // the hub dropped the topic: end the stream
            },
            _ = tokio::time::sleep(KEEPALIVE) => Bytes::from_static(KEEPALIVE_FRAME),
            _ = sleep_until_or_forever(st.expires_at) => {
                st.done = true;
                Bytes::from_static(EXPIRED_FRAME)
            }
        };
        Some((Ok(frame), st))
    });

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        // Opts out of the app-wide Compress middleware, which would otherwise hold frames back.
        .insert_header(actix_web::http::header::ContentEncoding::Identity)
        // Tells nginx not to buffer this response.
        .insert_header(("X-Accel-Buffering", "no"))
        .streaming(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const FAST_DEBOUNCE: Duration = Duration::from_millis(20);
    const FAST_GAP: Duration = Duration::from_millis(200);
    const LONG_SWEEP: Duration = Duration::from_secs(3600);

    /// A compute that counts its calls per topic and returns the call number as the snapshot.
    fn counting() -> (Arc<Compute>, Arc<Mutex<HashMap<Topic, usize>>>) {
        let calls: Arc<Mutex<HashMap<Topic, usize>>> = Arc::new(Mutex::new(HashMap::new()));
        let seen = calls.clone();
        let compute: Arc<Compute> = Arc::new(move |topic| {
            let mut calls = seen.lock().unwrap();
            let n = calls.entry(topic).or_insert(0);
            *n += 1;
            Some(n.to_string())
        });
        (compute, calls)
    }

    fn calls_of(calls: &Arc<Mutex<HashMap<Topic, usize>>>, topic: Topic) -> usize {
        calls.lock().unwrap().get(&topic).copied().unwrap_or(0)
    }

    async fn next_snapshot(rx: &mut watch::Receiver<Snapshot>) -> String {
        tokio::time::timeout(Duration::from_secs(2), rx.changed()).await
            .expect("timed out waiting for a snapshot")
            .expect("the hub dropped the topic");
        rx.borrow_and_update().as_deref().expect("a published snapshot").to_string()
    }

    #[actix_web::test]
    async fn a_new_subscriber_gets_a_first_snapshot() {
        let (compute, _) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, FAST_GAP, LONG_SWEEP);
        let mut rx = hub.subscribe(Topic::GamesInProgress(Uuid::new_v4()));
        assert_eq![next_snapshot(&mut rx).await, "1"];
    }

    #[actix_web::test]
    async fn a_burst_of_triggers_causes_one_recalculation() {
        let (compute, calls) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, FAST_GAP, LONG_SWEEP);
        let tid = Uuid::new_v4();
        let topic = Topic::GamesInProgress(tid);
        let mut rx = hub.subscribe(topic);
        next_snapshot(&mut rx).await;

        // Wait out the minimum gap so the burst is not held back by the first run.
        tokio::time::sleep(FAST_GAP).await;
        for _ in 0..25 {
            hub.tournament_changed(tid);
        }
        assert_eq![next_snapshot(&mut rx).await, "2"];
        tokio::time::sleep(FAST_GAP * 2).await;
        assert_eq![calls_of(&calls, topic), 2, "25 triggers in a burst should recalculate once"];
    }

    #[actix_web::test]
    async fn recalculations_respect_the_minimum_gap() {
        let (compute, calls) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, FAST_GAP, LONG_SWEEP);
        let tid = Uuid::new_v4();
        let topic = Topic::RoomMonitor(tid);
        let mut rx = hub.subscribe(topic);
        next_snapshot(&mut rx).await;
        let first = Instant::now();

        // A trigger right after a run waits for the gap rather than running at once.
        hub.tournament_changed(tid);
        next_snapshot(&mut rx).await;
        assert![first.elapsed() >= FAST_GAP - Duration::from_millis(30), "ran {:?} after the last run", first.elapsed()];
        assert_eq![calls_of(&calls, topic), 2];
    }

    #[actix_web::test]
    async fn a_trigger_recalculates_every_watched_topic_of_its_tournament() {
        let (compute, calls) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, Duration::ZERO, LONG_SWEEP);
        let tid = Uuid::new_v4();
        let mut games = hub.subscribe(Topic::GamesInProgress(tid));
        let mut monitor = hub.subscribe(Topic::RoomMonitor(tid));
        next_snapshot(&mut games).await;
        next_snapshot(&mut monitor).await;

        hub.tournament_changed(tid);
        assert_eq![next_snapshot(&mut games).await, "2"];
        assert_eq![next_snapshot(&mut monitor).await, "2"];
        let _ = calls;
    }

    #[actix_web::test]
    async fn unwatched_topics_are_never_recalculated() {
        let (compute, calls) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, Duration::ZERO, LONG_SWEEP);
        let watched_tid = Uuid::new_v4();
        let other_tid = Uuid::new_v4();
        let mut rx = hub.subscribe(Topic::GamesInProgress(watched_tid));
        next_snapshot(&mut rx).await;

        hub.tournament_changed(other_tid);
        hub.tournament_changed(watched_tid);
        next_snapshot(&mut rx).await;
        tokio::time::sleep(FAST_DEBOUNCE * 3).await;

        assert_eq![calls_of(&calls, Topic::GamesInProgress(other_tid)), 0];
        assert_eq![calls_of(&calls, Topic::RoomMonitor(other_tid)), 0];
        // The watched tournament's monitor topic was marked too, but no one watches it.
        assert_eq![calls_of(&calls, Topic::RoomMonitor(watched_tid)), 0];
    }

    #[actix_web::test]
    async fn closing_a_stream_lowers_the_viewer_count() {
        let (compute, _) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, FAST_GAP, LONG_SWEEP);
        let topic = Topic::RoomMonitor(Uuid::new_v4());
        assert![!hub.is_watching_anything()];

        let a = hub.subscribe(topic);
        let b = hub.subscribe(topic);
        assert_eq![hub.viewers(topic), 2];
        assert![hub.is_watching_anything()];

        drop(a);
        assert_eq![hub.viewers(topic), 1];
        drop(b);
        assert_eq![hub.viewers(topic), 0];
        assert![!hub.is_watching_anything()];
    }

    #[actix_web::test]
    async fn the_sweep_refreshes_watched_topics_without_a_trigger() {
        let (compute, _) = counting();
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, Duration::ZERO, Duration::from_millis(100));
        let mut rx = hub.subscribe(Topic::GamesInProgress(Uuid::new_v4()));
        assert_eq![next_snapshot(&mut rx).await, "1"];
        assert_eq![next_snapshot(&mut rx).await, "2"];
    }

    #[actix_web::test]
    async fn a_failed_build_keeps_the_last_snapshot() {
        let fail = Arc::new(AtomicUsize::new(0));
        let flag = fail.clone();
        let compute: Arc<Compute> = Arc::new(move |_| {
            if flag.load(Ordering::SeqCst) == 1 { None } else { Some("ok".to_string()) }
        });
        let hub = LiveHub::start_with(compute, FAST_DEBOUNCE, Duration::ZERO, LONG_SWEEP);
        let tid = Uuid::new_v4();
        let mut rx = hub.subscribe(Topic::GamesInProgress(tid));
        assert_eq![next_snapshot(&mut rx).await, "ok"];

        fail.store(1, Ordering::SeqCst);
        hub.tournament_changed(tid);
        tokio::time::sleep(FAST_DEBOUNCE * 5).await;
        assert![!rx.has_changed().unwrap(), "a failed build must not publish anything"];
        assert_eq![rx.borrow().as_deref(), Some("ok")];
    }
}
