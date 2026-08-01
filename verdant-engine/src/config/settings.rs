use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github: GitHubConfig,
    pub schedule: ScheduleConfig,
    pub content: ContentConfig,
    pub service: ServiceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub token: String,
    pub username: String,
    pub repo_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub mode: ScheduleMode,
    pub min_interval_minutes: u64,
    pub max_interval_minutes: u64,
    pub timezone_optimization: bool,
    pub turbo_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleMode {
    Gentle,
    Balanced,
    Aggressive,
    AggressiveRandom,
    Turbo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    pub commit_message_style: CommitMessageStyle,
    pub content_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitMessageStyle {
    Normal,
    Professional,
    AbsurdProfessional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub auto_start: bool,
    pub log_level: String,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .context("Failed to read configuration file")?;
        
        let config: Config = toml::from_str(&contents)
            .context("Failed to parse configuration file")?;
        
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        fs::write(path.as_ref(), contents)
            .context("Failed to write configuration file")?;
        
        Ok(())
    }

    /// Get default configuration path
    pub fn default_path() -> PathBuf {
        if cfg!(windows) {
            let local_app_data = std::env::var("LOCALAPPDATA")
                .unwrap_or_else(|_| "C:\\ProgramData".to_string());
            PathBuf::from(local_app_data).join("Verdant").join("config.toml")
        } else {
            PathBuf::from("config.toml")
        }
    }

    /// Create default configuration
    pub fn default() -> Self {
        Config {
            github: GitHubConfig {
                token: String::new(),
                username: String::new(),
                repo_name: "verdant".to_string(),
            },
            schedule: ScheduleConfig {
                mode: ScheduleMode::AggressiveRandom,
                min_interval_minutes: 30,
                max_interval_minutes: 180,
                timezone_optimization: true,
                turbo_multiplier: 1.0,
            },
            content: ContentConfig {
                commit_message_style: CommitMessageStyle::AbsurdProfessional,
                content_types: vec!["all".to_string()],
            },
            service: ServiceConfig {
                auto_start: true,
                log_level: "info".to_string(),
            },
        }
    }
}
