use std::process::Command;

use anyhow::{anyhow, Context};
use axum::{body::Body, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use secrecy::ExposeSecret;

use crate::configuration::VepSettings;

pub async fn annotate_hgvs_post(State(vep_settings): State<VepSettings>, Json(payload): Json<Vec<String>>) -> Result<Response, VepError> {
    annotate_variants(payload, vep_settings)
}

pub async fn annotate_hgvs_get(State(vep_settings): State<VepSettings>, Path(variant): Path<String>) -> Result<Response, VepError> {
    annotate_variants(vec![variant], vep_settings)
}

fn annotate_variants(variants: Vec<String>, vep_settings: VepSettings) -> Result<Response, VepError> {
    let variant_chunks = split_variants(variants, 10);
    let processed_chunks = variant_chunks.par_iter().map(|chunk| run_vep(chunk, &vep_settings)).collect::<Result<Vec<String>, VepError>>()?;

    let json_formatted_response = format!("[{}]", processed_chunks.join("").replace("\n{", ",{"));
    Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json_formatted_response))
        .context("Failed to build reponse")
        .map_err(VepError)
}

fn run_vep(variants: &[String], vep_settings: &VepSettings) -> Result<String, VepError> {
    let base_path = std::env::current_dir()
        .context("Failed to determine current directory")
        .map_err(VepError)?;
    let vep_path= base_path.join("scripts/vep.sh").to_str()
        .context("Failed to parse VEP path")
        .map_err(VepError)?
        .to_owned();

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
        .context("Failed to execute VEP script")
        .map_err(VepError)?;

    let output_stream = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };
    let output_text = String::from_utf8(output_stream)
        .context("Failed to parse VEP output")
        .map_err(VepError)?;
    match output.status.success() {
        true => Ok(output_text),
        false => Err(VepError(anyhow!(output_text))),
    }
}

fn split_variants(variants: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
    variants.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect::<Vec<_>>()
}

pub struct VepError(anyhow::Error);

impl IntoResponse for VepError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            self.0.to_string(),
        )
        .into_response()
    }
}