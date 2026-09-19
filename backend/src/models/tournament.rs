
use crate::database;
use crate::models::tournament_admin::TournamentAdmin;
use crate::models::tournamentgroup_tournament::TournamentGroupTournament;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable,Queryable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use utoipa::{ToSchema};
use uuid::Uuid;

#[derive(Clone)]
pub struct TournamentBuilder {
    organization: Option<String>,
    tname: String,
    breadcrumb: Option<String>,
    fromdate: Option<chrono::naive::NaiveDate>,
    todate: Option<chrono::naive::NaiveDate>,
    venue: Option<String>,
    city: Option<String>,
    country: Option<String>,
    contact: Option<String>,
    contactemail: Option<String>,
    shortinfo : Option<String>,
    info: Option<String>,
    owner_id: Option<Uuid>,
    pairing_code: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    is_public: bool,
    registration_open_date: Option<chrono::naive::NaiveDate>,
    registration_close_date: Option<chrono::naive::NaiveDate>,
    last_modified_user: Option<Uuid>,
    use_team_registration: bool,
    use_gear_registration: bool,
    use_volunteer_registration: bool,
    default_round_duration: i32,
}

impl TournamentBuilder {
    pub fn new(tname: &str) -> Self {
        Self {
            organization: None,
            tname: tname.to_string(),
            breadcrumb: None,
            fromdate: None,
            todate: None,
            venue: None,
            city: None,
            country: None,
            contact: None,
            contactemail: None,
            shortinfo: None,
            info: None,
            owner_id: None,
            pairing_code: None,
            address_line_1: None,
            address_line_2: None,
            state: None,
            zip_code: None,
            is_public: false,
            registration_open_date: None,
            registration_close_date: None,
            last_modified_user: None,
            use_team_registration: true,
            use_gear_registration: true,
            use_volunteer_registration: true,
            default_round_duration: 30,
        }
    }
    pub fn new_default(tname: &str) -> Self {
        // this mostly intended to be used by tests, not production
        Self {
            organization: Some("Nazarene".to_string()),
            tname: tname.to_string(),
            breadcrumb: Some("/test/post".to_string()),
            fromdate: Some(NaiveDate::from_ymd_opt(2025, 5, 23).unwrap()),
            todate: Some(NaiveDate::from_ymd_opt(2025, 5, 27).unwrap()),
            venue: Some("Vancouver University".to_string()),
            city: Some("Vancouver".to_string()),
            country: Some("Canada".to_string()),
            contact: Some("primemin".to_string()),
            contactemail: Some("primemin@fakeemail.com".to_string()),
            shortinfo: Some("Winter Olympics".to_string()),
            info: Some("Shawn White did excellent in the halfpipe.".to_string()),
            owner_id: None,
            pairing_code: None,
            address_line_1: Some("100 Convention Way".to_string()),
            address_line_2: Some("".to_string()),
            state: Some("BC".to_string()),
            zip_code: Some("V6B 1A1".to_string()),
            is_public: false,
            registration_open_date: None,
            registration_close_date: None,
            last_modified_user: None,
            use_team_registration: true,
            use_gear_registration: true,
            use_volunteer_registration: true,
            default_round_duration: 30,
        }
    }

    pub fn set_default_round_duration(mut self, minutes: i32) -> Self {
        self.default_round_duration = minutes;
        self
    }

    pub fn set_use_team_registration(mut self, v: bool) -> Self {
        self.use_team_registration = v;
        self
    }
    pub fn set_use_gear_registration(mut self, v: bool) -> Self {
        self.use_gear_registration = v;
        self
    }
    pub fn set_use_volunteer_registration(mut self, v: bool) -> Self {
        self.use_volunteer_registration = v;
        self
    }

    pub fn set_organization(mut self, org: &str) -> Self {
        self.organization = Some(org.to_string());
        self
    }
    pub fn set_tname(mut self, tname: &str) -> Self {
        self.tname = tname.to_string();
        self
    }
    pub fn set_breadcrumb(mut self, breadcrumb: &str) -> Self {
        self.breadcrumb = Some(breadcrumb.to_string());
        self
    }
    pub fn set_fromdate(mut self, fromdate: NaiveDate) -> Self {
        self.fromdate = Some(fromdate);
        self
    }
    pub fn set_todate(mut self, todate: NaiveDate) -> Self {
        self.todate = Some(todate);
        self
    }
    pub fn set_venue(mut self, venue: &str) -> Self {
        self.venue = Some(venue.to_string());
        self
    }
    pub fn set_city(mut self, city: &str) -> Self {
        self.city = Some(city.to_string());
        self
    }
    pub fn set_country(mut self, country: &str) -> Self {
        self.country = Some(country.to_string());
        self
    }
    pub fn set_address_line_1(mut self, address_line_1: &str) -> Self {
        self.address_line_1 = Some(address_line_1.to_string());
        self
    }
    pub fn set_address_line_2(mut self, address_line_2: &str) -> Self {
        self.address_line_2 = Some(address_line_2.to_string());
        self
    }
    pub fn set_state(mut self, state: &str) -> Self {
        self.state = Some(state.to_string());
        self
    }
    pub fn set_zip_code(mut self, zip_code: &str) -> Self {
        self.zip_code = Some(zip_code.to_string());
        self
    }
    pub fn set_contact(mut self, contact: &str) -> Self {
        self.contact = Some(contact.to_string());
        self
    }
    pub fn set_contactemail(mut self, contactemail: &str) -> Self {
        self.contactemail = Some(contactemail.to_string());
        self
    }
    pub fn set_shortinfo(mut self, shortinfo: &str) -> Self {
        self.shortinfo = Some(shortinfo.to_string());
        self
    }
    pub fn set_info(mut self, info: &str) -> Self {
        self.info = Some(info.to_string());
        self
    }
    pub fn set_owner_id(mut self, owner_id: Uuid) -> Self {
        self.owner_id = Some(owner_id);
        self
    }
    pub fn set_pairing_code(mut self, pairing_code: &str) -> Self {
        self.pairing_code = Some(pairing_code.to_string());
        self
    }
    pub fn set_is_public(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }
    pub fn set_registration_open_date(mut self, registration_open_date: NaiveDate) -> Self {
        self.registration_open_date = Some(registration_open_date);
        self
    }
    pub fn set_registration_close_date(mut self, registration_close_date: NaiveDate) -> Self {
        self.registration_close_date = Some(registration_close_date);
        self
    }
    pub fn set_last_modified_user(mut self, user_id: Uuid) -> Self {
        self.last_modified_user = Some(user_id);
        self
    }
    fn validate_all_are_some(&self) -> Result<bool, Vec<String>> {

        let mut errors = Vec::new();
    
        if self.organization.is_none() {
            errors.push("organization is required".to_string());
        }
        if self.breadcrumb.is_none() {
            errors.push("breadcrumb is required".to_string());
        }
        if self.fromdate.is_none() {
            errors.push("fromdate is required".to_string());
        }
        if self.todate.is_none() {
            errors.push("todate is required".to_string());
        }
        if self.venue.is_none() {
            errors.push("venue is required".to_string());
        }
        if self.city.is_none() {
            errors.push("city is required".to_string());
        }
        if self.country.is_none() {
            errors.push("country is required".to_string());
        }
        if self.contact.is_none() {
            errors.push("contact is required".to_string());
        }
        if self.contactemail.is_none() {
            errors.push("contactemail is required".to_string());
        }
        if self.shortinfo.is_none() {
            errors.push("shortinfo is required".to_string());
        }
        if self.info.is_none() {
            errors.push("info is required".to_string());
        }
        if self.owner_id.is_none() {
            errors.push("owner_id is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(true)
    }
    pub fn build(self) -> Result<NewTournament, Vec<String>> {

        let pairing_code = format!("{:06}", rand::random_range(0..=999_999u32));

        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(NewTournament {
                    organization: self.organization.unwrap(),
                    tname: self.tname,
                    breadcrumb: self.breadcrumb.unwrap(),
                    fromdate: self.fromdate.unwrap(),
                    todate: self.todate.unwrap(),
                    venue: self.venue.unwrap(),
                    city: self.city.unwrap(),
                    country: self.country.unwrap(),
                    contact: self.contact.unwrap(),
                    contactemail: self.contactemail.unwrap(),
                    shortinfo: self.shortinfo.unwrap(),
                    info: self.info.unwrap(),
                    owner_id: self.owner_id.unwrap(),
                    creator_id: self.owner_id.unwrap(),
                    pairing_code: self.pairing_code.unwrap_or(pairing_code),
                    address_line_1: self.address_line_1.unwrap_or_default(),
                    address_line_2: self.address_line_2.unwrap_or_default(),
                    state: self.state.unwrap_or_default(),
                    zip_code: self.zip_code.unwrap_or_default(),
                    is_public: self.is_public,
                    registration_open_date: self.registration_open_date,
                    registration_close_date: self.registration_close_date,
                    last_modified_user: self.last_modified_user.unwrap_or(self.owner_id.unwrap()),
                    use_team_registration: self.use_team_registration,
                    use_gear_registration: self.use_gear_registration,
                    use_volunteer_registration: self.use_volunteer_registration,
                    default_round_duration: self.default_round_duration,
                })
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> Result<Tournament, Vec<String>> {
        let new_tournament_result = self.build();

        if new_tournament_result.is_err() {
            return Err(new_tournament_result.err().unwrap());
        }

        let new_tournament = new_tournament_result.unwrap();
        match create(db, &new_tournament) {
            Err(e) => Err(vec![format!("Database insertion error: {}", e)]),
            Ok(tournament) => Ok(tournament)
        }
    }
}

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Identifiable,
    Selectable,
    ToSchema
)]
#[diesel(table_name = crate::schema::tournaments)]
#[diesel(primary_key(tid))]
pub struct Tournament {
    pub tid: Uuid, 
    pub organization: String,
    pub tname: String,             // name of this tournament (humans)
    pub breadcrumb: String,
    #[schema(value_type = String, format = DateTime)]
    pub fromdate: chrono::naive::NaiveDate,
    #[schema(value_type = String, format = DateTime)]
    pub todate: chrono::naive::NaiveDate,
    pub venue: String,
    pub city: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub is_public: bool,
    pub shortinfo : String,
    pub info: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
    pub creator_id: Uuid,
    pub pairing_code: String,
    pub address_line_1: String,
    pub address_line_2: String,
    pub state: String,
    pub zip_code: String,
    #[schema(value_type = Option<String>, format = Date)]
    pub registration_open_date: Option<chrono::naive::NaiveDate>,
    #[schema(value_type = Option<String>, format = Date)]
    pub registration_close_date: Option<chrono::naive::NaiveDate>,
    pub last_modified_user: Uuid,
    pub del_fl: bool,
    pub use_team_registration: bool,
    pub use_gear_registration: bool,
    pub use_volunteer_registration: bool,
    /// Default length of a round, in minutes; used by the schedule Auto-Schedule features.
    pub default_round_duration: i32,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Insertable,
    ToSchema
)]
#[diesel(table_name = crate::schema::tournaments)]
pub struct NewTournament {
    pub organization: String,
    pub tname: String,             // name of this tournament (humans)
    pub breadcrumb: String,
    pub fromdate: chrono::naive::NaiveDate,
    pub todate: chrono::naive::NaiveDate,
    pub venue: String,
    pub city: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub shortinfo : String,
    pub info: String,
    pub owner_id: Uuid,
    pub creator_id: Uuid,
    pub pairing_code: String,
    pub address_line_1: String,
    pub address_line_2: String,
    pub state: String,
    pub zip_code: String,
    pub is_public: bool,
    pub registration_open_date: Option<chrono::naive::NaiveDate>,
    pub registration_close_date: Option<chrono::naive::NaiveDate>,
    pub last_modified_user: Uuid,
    pub use_team_registration: bool,
    pub use_gear_registration: bool,
    pub use_volunteer_registration: bool,
    pub default_round_duration: i32,
}

/// Payload accepted from the frontend for tournament creation (no owner_id — that is
/// populated server-side from the authenticated user's context).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct NewTournamentPayload {
    pub organization: String,
    pub tname: String,
    pub breadcrumb: String,
    pub fromdate: chrono::naive::NaiveDate,
    pub todate: chrono::naive::NaiveDate,
    pub venue: String,
    pub city: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub shortinfo: String,
    pub info: String,
    pub is_public: bool,
    #[serde(default)]
    pub address_line_1: String,
    #[serde(default)]
    pub address_line_2: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub zip_code: String,
    #[serde(default)]
    pub registration_open_date: Option<chrono::naive::NaiveDate>,
    #[serde(default)]
    pub registration_close_date: Option<chrono::naive::NaiveDate>,
    // Which registration types the tournament offers. Default to true when omitted.
    #[serde(default = "default_true")]
    pub use_team_registration: bool,
    #[serde(default = "default_true")]
    pub use_gear_registration: bool,
    #[serde(default = "default_true")]
    pub use_volunteer_registration: bool,
    // Default round length in minutes; defaults to 30 when omitted.
    #[serde(default = "default_round_duration_value")]
    pub default_round_duration: i32,
}

fn default_true() -> bool { true }
fn default_round_duration_value() -> i32 { 30 }

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::tournaments)]
#[diesel(primary_key(tid))]
pub struct TournamentChangeset {
    pub organization: Option<String>,
    pub tname: Option<String>,
    pub breadcrumb: Option<String>,
    pub fromdate: Option<chrono::naive::NaiveDate>,
    pub todate: Option<chrono::naive::NaiveDate>,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub contact: Option<String>,
    pub contactemail: Option<String>,
    pub is_public: Option<bool>,
    pub shortinfo: Option<String>,
    pub info: Option<String>,
    pub pairing_code: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub registration_open_date: Option<chrono::naive::NaiveDate>,
    pub registration_close_date: Option<chrono::naive::NaiveDate>,
    pub use_team_registration: Option<bool>,
    pub use_gear_registration: Option<bool>,
    pub use_volunteer_registration: Option<bool>,
    pub default_round_duration: Option<i32>,
}

pub fn create(db: &mut database::Connection, item: &NewTournament) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    diesel::insert_into(tournaments)
        .values(item)
        .get_result::<Tournament>(db)
}

pub fn exists(db: &mut database::Connection, tid: Uuid) -> bool {
    use crate::schema::tournaments::dsl::tournaments;
    tournaments
        .find(tid)
        .get_result::<Tournament>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    tournaments.filter(tid.eq(item_id)).filter(del_fl.eq(false)).first::<Tournament>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::tournaments::dsl::*;
    tournaments.filter(del_fl.eq(false)).count().get_result(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments::dsl::*;
    let values = tournaments
        .filter(del_fl.eq(false))
        .order(todate)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Tournament>(db);
    values
}

pub enum VisibilityFilter {
    Public,  // default
    Private,
    All,
}

impl VisibilityFilter {
    /// Parses the `visibility` query parameter: "all" and "private" select those; anything
    /// else — including "public", an empty value, or an absent parameter — defaults to public.
    pub fn from_param(param: Option<&str>) -> Self {
        match param.map(str::to_ascii_lowercase).as_deref() {
            Some("all") => VisibilityFilter::All,
            Some("private") => VisibilityFilter::Private,
            _ => VisibilityFilter::Public,
        }
    }
}

pub fn read_between_dates(db: &mut database::Connection, from_dt: i64, to_dt: i64, visibility: VisibilityFilter) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments::dsl::*;
    let dt_from = Utc.timestamp_millis_opt(from_dt ).unwrap().naive_utc().date();
    let dt_to = Utc.timestamp_millis_opt(to_dt).unwrap().naive_utc().date();

    let mut query = tournaments
        .order(todate)
        .filter(del_fl.eq(false))
        .filter(todate.ge(dt_from))
        .filter(fromdate.le(dt_to))
        .into_boxed();
    match visibility {
        VisibilityFilter::Public => query = query.filter(is_public.eq(true)),
        VisibilityFilter::Private => query = query.filter(is_public.eq(false)),
        VisibilityFilter::All => {}
    }
    query.load::<Tournament>(db)
}

pub fn read_all_tournaments_where_user_is_admin(db: &mut database::Connection, admin_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments_admins::dsl::*;
    use crate::schema::tournaments::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tour_ids: Vec<Uuid> = 
        tournaments_admins
            .filter(adminid.eq(admin_id))
            .load::<TournamentAdmin>(db)
            .unwrap()
            .iter()
            .map(|tour_admin| tour_admin.tournamentid)
            .collect();

    tournaments
        .filter(tid.eq_any(tour_ids))
        .filter(del_fl.eq(false))
        .order(todate)
        .limit(page_size)
        .offset(offset_val)
        .load::<Tournament>(db)
}

/// One fully-formed row of the user's "Managed Tournaments" data table: the tournament plus the
/// display names of its creator and last-modifier. Populated in a single API call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserManagedTournamentRow {
    pub tid: Uuid,
    pub tname: String,
    pub venue: String,
    pub city: String,
    pub state: String,
    pub country: String,
    #[schema(value_type = String, format = Date)]
    pub fromdate: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub todate: NaiveDate,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub creator_id: Uuid,
    pub creator_name: String,
    pub last_modified_user_id: Uuid,
    pub last_modified_user_name: String,
}

/// Returns one page of the tournaments owned by `user_id` (enriched with creator + last-modifier
/// display names) plus the total count — a single scoped, paginated call for the profile table.
pub fn read_managed_tournament_rows_of_user(
    db: &mut database::Connection,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<UserManagedTournamentRow>, i64)> {
    let total: i64 = {
        use crate::schema::tournaments::dsl::*;
        tournaments.filter(owner_id.eq(user_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let list: Vec<Tournament> = {
        use crate::schema::tournaments::dsl::*;
        tournaments
            .filter(owner_id.eq(user_id))
            .filter(del_fl.eq(false))
            .order(todate.desc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Tournament>(db)?
    };
    let mut ids: Vec<Uuid> = list.iter().map(|t| t.creator_id).collect();
    ids.extend(list.iter().map(|t| t.last_modified_user));
    let name_by_id = crate::models::user::read_display_names(db, &ids)?;
    let name_of = |id: Uuid| name_by_id.get(&id).cloned().unwrap_or_else(|| id.to_string());
    let rows = list
        .into_iter()
        .map(|t| UserManagedTournamentRow {
            creator_name: name_of(t.creator_id),
            last_modified_user_name: name_of(t.last_modified_user),
            tid: t.tid,
            tname: t.tname,
            venue: t.venue,
            city: t.city,
            state: t.state,
            country: t.country,
            fromdate: t.fromdate,
            todate: t.todate,
            created_at: t.created_at,
            updated_at: t.updated_at,
            creator_id: t.creator_id,
            last_modified_user_id: t.last_modified_user,
        })
        .collect();
    Ok((rows, total))
}

pub fn read_all_tournaments_where_user_is_admin_or_owner(db: &mut database::Connection, user_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Tournament>> {
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let admin_tour_ids: Vec<Uuid> = {
        use crate::schema::tournaments_admins::dsl as ta_dsl;
        ta_dsl::tournaments_admins
            .filter(ta_dsl::adminid.eq(user_id))
            .load::<TournamentAdmin>(db)
            .unwrap_or_default()
            .into_iter()
            .map(|ta| ta.tournamentid)
            .collect()
    };

    use crate::schema::tournaments::dsl::*;
    tournaments
        .filter(owner_id.eq(user_id).or(tid.eq_any(admin_tour_ids)))
        .filter(del_fl.eq(false))
        .order(todate.desc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Tournament>(db)
}

/// Enriched row for the user's "As Admin" table: the tournament fields the table shows plus the
/// display names of its creator and last-modifier.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AdminTournamentRow {
    pub tid: Uuid,
    pub tname: String,
    pub organization: String,
    #[schema(value_type = String, format = Date)]
    pub fromdate: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub todate: NaiveDate,
    pub venue: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub owner_id: Uuid,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub creator_id: Uuid,
    pub creator_name: String,
    pub last_modified_user_id: Uuid,
    pub last_modified_user_name: String,
}

/// Same set as `read_all_tournaments_where_user_is_admin_or_owner`, enriched with creator +
/// last-modifier display names for the profile table's audit columns.
pub fn read_admin_tournament_rows_of_user(
    db: &mut database::Connection,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<AdminTournamentRow>> {
    let list = read_all_tournaments_where_user_is_admin_or_owner(db, user_id, pagination)?;
    let mut ids: Vec<Uuid> = list.iter().map(|t| t.creator_id).collect();
    ids.extend(list.iter().map(|t| t.last_modified_user));
    let name_by_id = crate::models::user::read_display_names(db, &ids)?;
    let name_of = |id: Uuid| name_by_id.get(&id).cloned().unwrap_or_else(|| id.to_string());
    Ok(list
        .into_iter()
        .map(|t| AdminTournamentRow {
            creator_name: name_of(t.creator_id),
            last_modified_user_name: name_of(t.last_modified_user),
            tid: t.tid,
            tname: t.tname,
            organization: t.organization,
            fromdate: t.fromdate,
            todate: t.todate,
            venue: t.venue,
            city: t.city,
            state: t.state,
            country: t.country,
            owner_id: t.owner_id,
            created_at: t.created_at,
            updated_at: t.updated_at,
            creator_id: t.creator_id,
            last_modified_user_id: t.last_modified_user,
        })
        .collect())
}

pub fn read_all_tournaments_of_tournamentgroup(db: &mut database::Connection, tg_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournamentgroups_tournaments::dsl::*;
    use crate::schema::tournaments::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tour_ids: Vec<Uuid> = 
        tournamentgroups_tournaments
            .filter(tournamentgroupid.eq(tg_id))
            .load::<TournamentGroupTournament>(db)
            .unwrap()
            .iter()
            .map(|tg_tour| tg_tour.tournamentid)
            .collect();

    tournaments
        .filter(tid.eq_any(tour_ids))
        .filter(del_fl.eq(false))
        .order(todate)
        .limit(page_size)
        .offset(offset_val)
        .load::<Tournament>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TournamentChangeset, modified_by: Uuid) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    diesel::update(tournaments.filter(tid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
            last_modified_user.eq(modified_by),
        ))
        .get_result(db)
}

/// Soft delete: mark the tournament deleted (excluded from reads) without removing the row.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments::dsl::*;
    diesel::update(tournaments.filter(tid.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the tournament row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments::dsl::*;
    diesel::delete(tournaments.filter(tid.eq(item_id))).execute(db)
}
