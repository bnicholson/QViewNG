//! The backend's integration tests, compiled as ONE test binary.
//!
//! Each `*_test.rs` file here is a module of this crate rather than a crate of its own. Cargo
//! links every integration-test crate separately against the whole backend, so 52 separate files
//! meant 52 links of ~38 MB each on every change; as modules of one crate they link once.
//! Run one file's tests by filtering on its module name, e.g. `cargo test --test it game_test::`.
//!
//! Add a new test file by creating `<service>_test.rs` here and declaring it below. See
//! ../README.txt for what goes in each file.

mod common;
mod fixtures;

mod apicalllog_test;
mod auth_test;
mod computer_test;
mod createtournamentapplicant_test;
mod division_rows_test;
mod division_test;
mod eligible_quizzers_test;
mod email_test;
mod equipment_test;
mod equipmentdbo_test;
mod equipmentregistration_test;
mod equipmentset_test;
mod extensioncord_test;
mod game_rows_test;
mod game_test;
mod gameevent_test;
mod helpdoc_test;
mod interfacebox_test;
mod live_test;
mod jumppad_test;
mod microphonerecorder_test;
mod monitor_test;
mod permission_test;
mod pingmsg_test;
mod pool_bracket_test;
mod poolbracketgroup_test;
mod powerstrip_test;
mod projector_test;
mod quizzer_rows_test;
mod role_test;
mod room_queue_test;
mod room_rows_test;
mod room_test;
mod roomgroup_test;
mod roster_test;
mod round_rows_test;
mod round_test;
mod roundgroup_test;
mod schedule_test;
mod seed_data_test;
mod statsgroup_test;
mod team_rows_test;
mod team_test;
mod teamgroup_test;
mod tournament_test;
mod tournamentgroup_rows_test;
mod tournamentgroup_test;
mod user_gear_rows_test;
mod user_managed_tournament_rows_test;
mod user_managed_tournamentgroup_rows_test;
mod user_role_test;
mod user_roster_quizzer_rows_test;
mod user_team_rows_test;
mod user_test;
