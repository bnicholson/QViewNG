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

#[derive(serde::Deserialize)]
pub struct GameEventParams {
                            // GET /gameevent
    pub gid: String,        // ?gid=  // Game ID (UUID; in QuizMachine codebase it is referred to as a GUID)
    pub bldgroom: String,   // &bldgroom=  // "building name + room name"
    pub key: String,        // &key=0000000067ebfa14a12be944e218a9a876f915ca36f8a440  // "key4Server"
    pub tk: String,         // &tk=
    pub tn: String,         // &tn=midsouthoctobermeet  // "tournament name"
    pub dn: String,         // &dn=All  // "division name"
    pub rm: String,         // &rm=Room%206  // "room name"
    pub rd: i32,            // &rd=1  // "round number" - must be a number only
    pub qn: i32,            // &qn=6  // "question number"
    pub e: i32,             // &e=1  // "event number"
    pub n: String,          // &n=Clara%20Hoffmann
    pub t: i32,             // &t=0  // "team number"
    pub q: i32,             // &q=1  // "quizzer number/seat" (zero-indexed)
    pub ec: String,         // &ec=TC  // "event code"
    pub p1: String,         // &p1=  // "parameter 1"
    pub p2: String,         // &p2=  // "parameter 2"
    pub ts: String,         // &ts=1783464615  // "timestamp"
    pub md5: String,        // &md5=804d745c4c2b61b7b3901a7502c10d43  // "md5digest"
    pub nonce: String,      // &nonce=2567764725361350931648290373801246491543638377087318691787425923628932249
    pub s3s: String,        // &s3s=zC1sd1w7%2bkJYyr4bBThttP7ujkI%3d  // "SHA3-512 checksum"
}
