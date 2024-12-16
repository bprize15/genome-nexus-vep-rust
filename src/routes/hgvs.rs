use std::process::Command;

use axum::{body::Body, extract::{Path, State}, response::Response, Json};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use secrecy::ExposeSecret;

use crate::configuration::VepSettings;

pub async fn annotate_hgvs_post(State(vep_settings): State<VepSettings>, Json(payload): Json<Vec<String>>) -> Response {
    annotate_variants(payload, vep_settings)
}

pub async fn annotate_hgvs_get(State(vep_settings): State<VepSettings>, Path(variant): Path<String>) -> Response {
    annotate_variants(vec![variant], vep_settings)
}

fn annotate_variants(variants: Vec<String>, vep_settings: VepSettings) -> Response {
    let variant_chunks = split_variants(variants, 10);
    let processed_chunks: Vec<String> = variant_chunks.par_iter().map(|chunk| run_vep(chunk, &vep_settings)).collect();
    let json_formatted_response = format!("[{}]", processed_chunks.join("").replace("\n{", ",{"));
    Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_formatted_response))
        .expect("Failed to build reponse")
}

fn run_vep(variants: &[String], vep_settings: &VepSettings) -> String {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let vep_path= base_path.join("scripts/vep.sh").to_str().expect("Failed to parse VEP path").to_owned();

    let output = Command::new("sh")
        .arg(vep_path)
        .arg("--database")
        .arg(format!("--host={}", vep_settings.host))
        .arg(format!("--user={}", vep_settings.username))
        .arg(format!("--password={}", vep_settings.password.expose_secret()))
        .arg(format!("--port={}", vep_settings.port))
        .arg(format!("--fork={}", vep_settings.forks))
        .arg(format!("--input_data={}", variants.join("\n")))
        .arg("--format=hgvs")
        .arg("--output_file=STDOUT")
        .arg("--everything")
        .arg("--hgvsg")
        .arg("--no_stats")
        .arg("--xref_refseq")
        .arg("--json")
        .output()
        .expect("Failed to execute VEP script");
    let output = match output.status.success() {
        true => output.stdout,
        false => output.stderr,
    };
    String::from_utf8(output)
        .expect("Failed to parse VEP output")
}

fn split_variants(variants: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
    variants.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect::<Vec<_>>()
}