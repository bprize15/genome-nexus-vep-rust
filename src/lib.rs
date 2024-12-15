mod configuration;
mod routes;

pub use configuration::get_configuration;
pub use routes::{annotate_hgvs_get, annotate_hgvs_post};