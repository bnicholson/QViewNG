
use actix_web::web;
use utoipa_swagger_ui::SwaggerUi;
// use services::tournament::TournamentDoc;
// use services::division::DivisionDoc;
use crate::services;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
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
            .service(services::tournament::endpoints(web::scope("/tournaments")))
            .service(services::division::endpoints(web::scope("/divisions")))
            .service(services::user::endpoints(web::scope("/users")))
            .service(services::room::endpoints(web::scope("/rooms")))
            .service(services::round::endpoints(web::scope("/rounds")))
            // .service(services::file::endpoints(web::scope("/files")))
            // .service(services::namelist::endpoints(web::scope("/namelist")))
            // .service(services::pingmsg::endpoints(web::scope("/pingmsg")))
            // .service(services::roominfo::endpoints(web::scope("/roominfo")))
            // .service(services::quizevent::endpoints(web::scope("/quizevents")))
    );
}
