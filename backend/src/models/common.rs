use uuid::Uuid;

pub type ID = i32;

pub type BigId = i64;

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[derive(serde::Deserialize)]
pub struct ReadGamesDetailedParams {
    pub page: i64,
    pub page_size: i64,
    pub pairing_code: String,
}

impl PaginationParams {
    pub const MAX_PAGE_SIZE: u16 = 1000;
}

#[derive(serde::Deserialize)]
pub struct SearchDateParams {
    pub from_date: i64,
    pub to_date: i64,
}

#[derive(serde::Deserialize)]
pub struct TournamentParam {
    pub tid: Uuid,
}
