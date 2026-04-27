
use actix_web::{web, middleware::from_fn};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Next;
use utoipa_swagger_ui::SwaggerUi;
// use services::tournament::TournamentDoc;
// use services::division::DivisionDoc;
use crate::{services, middleware::add_user_context_to_extensions_from_access_token_middleware};

// For debug:
async fn log_headers_middleware(req: ServiceRequest, next: Next<impl actix_web::body::MessageBody>) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    log::debug!("--- Incoming request: {} {} ---", req.method(), req.uri());
    for (name, value) in req.headers() {
        log::debug!("  Header: {:?} = {:?}", name, value);
    }
    next.call(req).await
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services::pingmsg::endpoints(web::scope("/pingmsg")));
    cfg.service(services::namelist::endpoints(web::scope("/namelist")));
    cfg.service(services::gameevent::endpoints(web::scope("/scoreevent")));
    // cfg.service(services::file::endpoints(web::scope("/files")))
    // cfg.service(services::roominfo::endpoints(web::scope("/roominfo")))
    cfg.service(
        web::scope("/api")
            // For debug:
            .wrap(from_fn(log_headers_middleware))
            .wrap(from_fn(add_user_context_to_extensions_from_access_token_middleware))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    // When you add a new scoped service, add its docs path here (*some other configuration required):
                    // .url("/tournaments/docs", TournamentDoc::openapi())
                    // .url("/divisions/docs", DivisionDoc::openapi())
                    // .url("/roominfo/docs", RoomInfoDoc::openapi())
                    // .url("/quizevents/docs", QuizEventDoc::openapi())
                    // .url("/namelist/docs", NamelistDoc::openapi())
                    // .url("/pingmsg/docs", PingMsgDoc::openapi())
            )
            .service(services::auth::endpoints(web::scope("/auth")))
            .service(services::tournament::endpoints(web::scope("/tournaments")))
            .service(services::division::endpoints(web::scope("/divisions")))
            .service(services::user::endpoints(web::scope("/users")))
            .service(services::room::endpoints(web::scope("/rooms")))
            .service(services::round::endpoints(web::scope("/rounds")))
            .service(services::team::endpoints(web::scope("/teams")))
            .service(services::game::endpoints(web::scope("/games")))
            .service(services::role::endpoints(web::scope("/roles")))
            .service(services::permission::endpoints(web::scope("/permissions")))
            .service(services::users_roles::endpoints(web::scope("/usersroles")))
            .service(services::tournamentgroup::endpoints(web::scope("/tournamentgroups")))
            .service(services::statsgroup::endpoints(web::scope("/statsgroups")))
            .service(services::create_tournament_applicant::endpoints(web::scope("/createtournamentapplicants")))
            .service(services::roster::endpoints(web::scope("/rosters")))
            .service(services::equipmentset::endpoints(web::scope("/equipmentsets")))
            .service(services::equipmentregistration::endpoints(web::scope("/equipmentregistrations")))
            .service(
                web::scope("/equipment")
                    .service(services::computer::endpoints(web::scope("/computers")))
                    .service(services::jumppad::endpoints(web::scope("/jumppads")))
                    .service(services::interfacebox::endpoints(web::scope("/interfaceboxes")))
                    .service(services::monitor::endpoints(web::scope("/monitors")))
                    .service(services::microphonerecorder::endpoints(web::scope("/microphonerecorders")))
                    .service(services::projector::endpoints(web::scope("/projectors")))
                    .service(services::powerstrip::endpoints(web::scope("/powerstrips")))
                    .service(services::extensioncord::endpoints(web::scope("/extensioncords")))
                    .service(services::equipment::endpoints(web::scope("")))  // *must be after all other paths of parent service
            )
    );
}
