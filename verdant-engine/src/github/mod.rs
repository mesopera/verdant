pub mod auth;
pub mod client;
pub mod detector;

pub use auth::authenticate;
pub use client::GitHubClient;
pub use detector::detect_repo;
