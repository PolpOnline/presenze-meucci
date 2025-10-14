use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use utoipa::ToSchema;

use crate::{app::openapi::DASHBOARD_TAG, types::AbsenceStatus, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct PatchAbsenceRequest {
    id: i32,
    #[schema(default = AbsenceStatus::default)]
    status: AbsenceStatus,
    substitute_teacher_availability_id: Option<i32>,
}

#[utoipa::path(
    patch,
    path = "/",
    summary = "Modify absence",
    request_body = PatchAbsenceRequest,
    responses(
        (status = OK, description = "Absence modified"),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
        (status = NOT_FOUND, description = "Absence not found or not accessible"),
    ),
    security(("session" = [])),
    tag = DASHBOARD_TAG,
)]
pub async fn patch(
    auth_session: AuthSession,
    Sonic(req): Sonic<PatchAbsenceRequest>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    if req.substitute_teacher_availability_id.is_some()
        && req.status != AbsenceStatus::SubstituteFound
    {
        return (
            StatusCode::BAD_REQUEST,
            "Status must be SubstituteFound if availability id is set",
        )
            .into_response();
    }

    if req.substitute_teacher_availability_id.is_none()
        && req.status == AbsenceStatus::SubstituteFound
    {
        return (
            StatusCode::BAD_REQUEST,
            "Availability id must be set if status is SubstituteFound",
        )
            .into_response();
    }

    let res = sqlx::query!(
        r#"
        UPDATE absence ab
        SET status = COALESCE($2, ab.status),
            substitute_teacher_availability = (
                SELECT av.id
                FROM availability av
                JOIN teacher t2 ON av.teacher_id = t2.id
                JOIN import i2 ON t2.import_id = i2.id
                WHERE av.id = $3
                    AND i2.user_id = $4
            )
        FROM lesson l, teacher t, import i
        WHERE ab.id = $1
          AND ab.absent_teacher_lesson = l.id
          AND l.teacher_id = t.id
          AND t.import_id = i.id
          AND i.user_id = $5
        "#,
        req.id,
        req.status as AbsenceStatus,
        req.substitute_teacher_availability_id,
        user.id,
        user.id
    )
    .execute(&auth_session.backend.db)
    .await;

    match res {
        Ok(done) if done.rows_affected() >= 1 => StatusCode::OK.into_response(),
        Ok(_) => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
        Err(e) => {
            error!("Failed to modify absence: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}
