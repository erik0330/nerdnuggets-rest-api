use crate::state::AppState;
use aws_sdk_s3::primitives::ByteStream;
use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use types::{
    dto::{
        GetCategoryOption, GetCategoryResponse, GetCityOption, GetCityResponse, GetCountryOption,
        GetCountryResponse, GetInstitutionDetailItem, GetInstitutionDetailOption,
        GetInstitutionsItem, GetInstitutionsOption, RemoveFileFromS3Request,
    },
    error::{ApiError, UploadError, ValidatedRequest},
    models::Category,
};
use url::Url;
use uuid::Uuid;

pub async fn get_country(
    opts: Option<Query<GetCountryOption>>,
    State(state): State<AppState>,
) -> Result<Json<GetCountryResponse>, ApiError> {
    let Query(opts) = opts.unwrap_or_default();
    let result = state
        .service
        .util
        .get_country(opts.country.unwrap_or_default(), opts.limit.unwrap_or(500))
        .await?;
    Ok(Json(result))
}

pub async fn get_city(
    opts: Option<Query<GetCityOption>>,
    State(state): State<AppState>,
) -> Result<Json<GetCityResponse>, ApiError> {
    let Query(opts) = opts.unwrap_or_default();
    let result = state
        .service
        .util
        .get_city(
            opts.country.unwrap_or_default(),
            opts.city.unwrap_or_default(),
        )
        .await?;
    Ok(Json(result))
}

pub async fn get_institutions(
    Query(opts): Query<GetInstitutionsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<GetInstitutionsItem>>, ApiError> {
    let result = state
        .service
        .util
        .get_institutions(state.env.google_map_api_key, opts.search_type, opts.query)
        .await?;
    Ok(Json(result))
}

pub async fn get_institution_detail(
    Query(opts): Query<GetInstitutionDetailOption>,
    State(state): State<AppState>,
) -> Result<Json<GetInstitutionDetailItem>, ApiError> {
    let result = state
        .service
        .util
        .get_institution_detail(state.env.google_map_api_key, opts.place_id)
        .await?;
    Ok(Json(result))
}

pub async fn get_categories(
    Query(opts): Query<GetCategoryOption>,
    State(state): State<AppState>,
) -> Result<Json<GetCategoryResponse>, ApiError> {
    let result = state
        .service
        .util
        .get_categories(
            opts.name.unwrap_or_default(),
            opts.is_available,
            opts.start,
            opts.limit,
        )
        .await?;
    Ok(Json(result))
}

pub async fn get_category_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Category>, ApiError> {
    let result = state.service.util.get_category_by_id(&id).await?;
    Ok(Json(result))
}

pub async fn upload_file_to_s3(
    Path(folder): Path<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<String>, ApiError> {
    let bucket_name = state.env.aws_bucket_name;
    let mut file_bytes = None;
    let mut name_field = None;
    let mut content_type = None;
    let mut file_name = String::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::UploadError(UploadError::NoFileProvided))?
    {
        if let Some("name") = field.name() {
            name_field = Some(
                field
                    .text()
                    .await
                    .map_err(|_| ApiError::UploadError(UploadError::FailedProcessData))?,
            );
        } else {
            content_type = field.content_type().map(|ct| ct.to_owned());
            file_name = field.file_name().map(|s| s.to_string()).unwrap_or_default();
            file_bytes = Some(
                field
                    .bytes()
                    .await
                    .map_err(|_| {
                        ApiError::UploadError(UploadError::SomethingWentWrong(
                            "Failed to read file data".to_string(),
                        ))
                    })?
                    .to_vec(),
            );
        }
    }
    let extension = file_name.rsplit('.').next().map(|f| format!(".{f}"));
    let name = if let Some(name) = name_field {
        format!("{}{}", name, extension.unwrap_or_default())
    } else {
        file_name
    };
    let bytes = file_bytes.ok_or(ApiError::UploadError(UploadError::NoFileProvided))?;
    let key = format!("{folder}/{}-{}", Uuid::new_v4().to_string(), name);
    let url = format!("https://{bucket_name}/{key}");
    let body = ByteStream::from(bytes.clone());
    state
        .s3_client
        .put_object()
        .bucket(&bucket_name)
        .content_type(content_type.unwrap_or_default())
        .content_length(bytes.len() as i64)
        .key(&key)
        .body(body)
        .send()
        .await
        .map_err(|_| {
            ApiError::UploadError(UploadError::SomethingWentWrong(
                "Failed to upload file to S3".to_string(),
            ))
        })?;
    Ok(Json(url))
}

pub async fn remove_file_from_s3(
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<RemoveFileFromS3Request>,
) -> Result<Json<bool>, ApiError> {
    if let Ok(url) = Url::parse(&payload.link) {
        let bucket_name = state.env.aws_bucket_name;
        let key = url.path().trim_start_matches('/').to_string();
        let result = state
            .s3_client
            .delete_object()
            .bucket(&bucket_name)
            .key(key)
            .send()
            .await
            .ok()
            .is_some();
        return Ok(Json(result));
    }
    Err(UploadError::SomethingWentWrong(
        "Url is invalid".to_string(),
    ))?
}
