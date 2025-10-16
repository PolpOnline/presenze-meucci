use axum::{extract::Query, response::IntoResponse};
use axum_serde::Sonic;
use chrono::NaiveDate;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::DASHBOARD_TAG, users::AuthSession};

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetCanBeAbsentRequest {
    /// Date for which to get teachers who can be absent.
    /// Results in the teachers who have lessons on that day.
    /// If not provided, defaults to today.
    date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CanBeAbsentTeacher {
    id: i32,
    full_name: String,
}

#[utoipa::path(
    get,
    path = "/can_be_absent",
    summary = "Teachers who can be absent",
    params(GetCanBeAbsentRequest),
    responses(
        (status = OK, description = "Absences and their status", body = Vec<CanBeAbsentTeacher>),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
    ),
    security(
        ("session" = [])
    ),
    tag = DASHBOARD_TAG,
)]
pub async fn can_be_absent(
    Query(req): Query<GetCanBeAbsentRequest>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    let can_be_absent_teachers = match sqlx::query_as!(
        CanBeAbsentTeacher,
        r#"
        SELECT DISTINCT t.id, t.full_name
        FROM teacher t
                 JOIN import i ON t.import_id = i.id
                 JOIN lesson l ON t.id = l.teacher_id
        WHERE l.day = EXTRACT(DOW FROM COALESCE($1, CURRENT_DATE)::date)
          AND i.user_id = $2
        ORDER BY t.full_name
        "#,
        req.date,
        user.id,
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!(
                "Database error when fetching teachers who can be absent: {}",
                e
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    Sonic(can_be_absent_teachers).into_response()
}
