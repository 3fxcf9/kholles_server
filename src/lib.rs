pub const LISTEN_PATH_ENV_NAME: &str = "FS_LISTEN_PATH";
pub const PROOF_SUBFOLDER_NAME: &str = "proofs";
pub const WEEK_SUBFOLDER_NAME: &str = "weeks";
pub const GITHUB_WEBHOOK_SECRET_NAME: &str = "GITHUB_WEBHOOK_SECRET";

pub mod error;
pub mod list_fs;
pub mod md_to_html;
pub mod types;
pub mod webhook;
pub mod webhook_error;
