use backend::models::tournament_admin::NewTournamentAdmin;
use uuid::Uuid;

pub fn get_tour_admin_payload_singular(tid: Uuid, user_id: Uuid) -> NewTournamentAdmin {
    NewTournamentAdmin {
        tournamentid: tid,
        adminid: user_id,       
        role_description: "default role (test id 334)".to_string(),            
        access_lvl: 0
    }
}
