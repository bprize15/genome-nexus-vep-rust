use axum::{extract::Path, Json};

pub async fn annotate_hgvs_post(Json(payload): Json<Vec<String>>) -> String {
    "POST".into()
}

pub async fn annotate_hgvs_get(Path(variant): Path<String>) -> String {
    "GET".into()
}