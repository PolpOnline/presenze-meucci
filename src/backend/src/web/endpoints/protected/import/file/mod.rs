mod importer;

use axum::{extract::Query, response::IntoResponse};
use axum_serde::Xml;
use chrono::NaiveDateTime;
use http::StatusCode;
use importer::import_file;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    app::openapi::IMPORT_TAG, users::AuthSession,
    web::endpoints::protected::import::file::importer::ScheduleFile,
};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ImportFileMeta {
    file_name: String,
    #[param(default = ImportMode::default)]
    mode: ImportMode,
    begin_ts: NaiveDateTime,
    end_ts: NaiveDateTime,
}

#[derive(Default, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ImportMode {
    DryRun,
    #[default]
    Write,
}

#[utoipa::path(
    post,
    path = "/file",
    summary = "Import File",
    description = "Import a schedule file. The file must be in XML format exported by OrarioFacile.",
    request_body(content = ScheduleFile, content_type = "application/xml"),
    params(ImportFileMeta),
    responses(
        (status = OK, description = "File imported successfully", example = ""),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
        (status = INTERNAL_SERVER_ERROR, description = "Error importing file", example = "Error importing file: ..."),
    ),
    security(
        ("session" = [])
    ),
    tag = IMPORT_TAG,
)]
pub async fn file(
    auth_session: AuthSession,
    Query(meta): Query<ImportFileMeta>,
    Xml(file): Xml<ScheduleFile>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    match import_file(&auth_session.backend.db, meta, file, user.id).await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Error importing file: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error importing file: {:?}", e),
            )
                .into_response();
        }
    }

    StatusCode::OK.into_response()
}
