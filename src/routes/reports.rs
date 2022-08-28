use crate::error::ApplicationError;
use crate::persistence;
use mongodb::Database;
use paperclip::actix::{
    api_v2_operation,
    web::{self},
};

pub fn reports_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/reports/all").route(web::get().to(report_all)));
}

/// Reports about whole smart house
#[api_v2_operation]
pub async fn report_all(db: web::Data<Database>) -> Result<String, ApplicationError> {
    let report = match persistence::reports::report_all(&db).await {
        Err(e) => {
            return Err(ApplicationError::InternalServer {
                message: e.to_string(),
            })
        }
        Ok(report) => report,
    };

    Ok(report)
}
