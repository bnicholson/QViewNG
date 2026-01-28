
use crate::database;
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
    region: Option<String>,
    country: Option<String>,
    contact: Option<String>,
    contactemail: Option<String>,
    shortinfo : Option<String>,
    info: Option<String>
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
            region: None,
            country: None,
            contact: None,
            contactemail: None,
            shortinfo: None,
            info: None
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
            region: Some("North America".to_string()),
            country: Some("Canada".to_string()),
            contact: Some("primemin".to_string()),
            contactemail: Some("primemin@fakeemail.com".to_string()),
            shortinfo: Some("Winter Olympics".to_string()),
            info: Some("Shawn White did excellent in the halfpipe.".to_string())
        }
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
    pub fn set_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string());
        self
    }
    pub fn set_country(mut self, country: &str) -> Self {
        self.country = Some(country.to_string());
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
        if self.region.is_none() {
            errors.push("region is required".to_string());
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
        
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(true)
    }
    pub fn build(self) -> Result<NewTournament, Vec<String>> {
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
                    region: self.region.unwrap(),
                    country: self.country.unwrap(),
                    contact: self.contact.unwrap(),
                    contactemail: self.contactemail.unwrap(),
                    shortinfo: self.shortinfo.unwrap(),
                    info: self.info.unwrap()
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
    pub region: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub is_public: bool,
    pub shortinfo : String,
    pub info: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
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
    pub region: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub shortinfo : String,
    pub info: String
}

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
    pub region: Option<String>,
    pub country: Option<String>,
    pub contact: Option<String>,
    pub contactemail: Option<String>,
    pub is_public: Option<bool>,
    pub shortinfo: Option<String>,
    pub info: Option<String>
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
    tournaments.filter(tid.eq(item_id)).first::<Tournament>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments::dsl::*;
    let values = tournaments
        .order(todate)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Tournament>(db);
    values
}

pub fn read_between_dates(db: &mut database::Connection, from_dt: i64, to_dt: i64) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments::dsl::*;
    let dt_from = Utc.timestamp_millis_opt(from_dt ).unwrap().naive_utc().date();
    let dt_to = Utc.timestamp_millis_opt(to_dt).unwrap().naive_utc().date();

    let values = tournaments
        .order(todate)
        .filter(todate.ge(dt_from))
        .filter(fromdate.le(dt_to))
        .load::<Tournament>(db);
    values
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TournamentChangeset) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    diesel::update(tournaments.filter(tid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments::dsl::*;
    diesel::delete(tournaments.filter(tid.eq(item_id))).execute(db)
}
