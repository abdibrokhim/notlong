// src/routes/redirect.rs

use actix_web::{get, web, HttpResponse, http::header};
use crate::db::operations::find_by_short_code;
use crate::Pool;

#[get("/{short_code}")]
pub async fn redirect_short(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let code = path.into_inner();

    let mut conn = pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("DB pool error: {e}"))
    })?;

    let result = find_by_short_code(&mut conn, &code);
    match result {
        Ok(record) => {
            // 301/302 redirect to the original_url
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, record.original_url))
                .finish())
        }
        Err(_) => Ok(HttpResponse::NotFound().body("Short URL not found.")),
    }
}
