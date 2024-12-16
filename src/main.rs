use axum::{routing::{get, post}, Router};
use genome_nexus_vep_rust::{annotate_hgvs_get, annotate_hgvs_post, get_configuration};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let application_settings = configuration.application;
    let vep_settings = configuration.vep;

    let app = Router::new()
        .route("/", get(|| async { "Hello world! "}))
        .route("/vep/human/hgvs/:variant", get(annotate_hgvs_get))
        .route("/vep/human/hgvs", post(annotate_hgvs_post))
        .with_state(vep_settings);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", application_settings.host, application_settings.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
