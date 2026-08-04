mod config;
mod github;
mod scheduler;
mod generator;
mod service;

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error, debug};
use tracing_subscriber::{EnvFilter, fmt};
use chrono::Utc;

use config::Config;
use github::{GitHubClient, authenticate, detect_repo};
use scheduler::{Scheduler, execute_commit};
use generator::{ContentGenerator, MetricsGenerator};

#[derive(Parser)]
#[command(name = "verdant-engine")]
#[command(about = "Verdant™ - Enterprise Feline Productivity Optimization Suite", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup authentication and configuration
    Auth,
    
    /// Interactive configuration setup
    Config,
    
    /// Run the engine in foreground (for testing)
    Run,
    
    /// Install as Windows service
    Install,
    
    /// Uninstall Windows service
    Uninstall,
    
    /// Start the Windows service
    Start,
    
    /// Stop the Windows service
    Stop,
    
    /// Run as Windows service (internal use)
    Service,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth => setup_auth().await,
        Commands::Config => setup_config().await,
        Commands::Run => run_foreground().await,
        Commands::Install => install_service(),
        Commands::Uninstall => uninstall_service(),
        Commands::Start => start_service(),
        Commands::Stop => stop_service(),
        Commands::Service => service::run_service(),
    }
}

async fn setup_auth() -> Result<()> {
    init_logging("info");
    
    println!("\n🐱 Welcome to Verdant™ Engine Setup\n");
    
    // Get GitHub token
    let token = authenticate()?;
    
    // Get username
    let username = github::auth::get_username()?;
    
    // Detect repository
    let repo_name = detect_repo(&token, &username).await?;
    
    // Load or create config
    let config_path = Config::default_path();
    let mut config = if config_path.exists() {
        Config::load(&config_path)?
    } else {
        Config::default()
    };
    
    // Update GitHub settings
    config.github.token = token;
    config.github.username = username;
    config.github.repo_name = repo_name;
    
    // Ensure config directory exists
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Save config
    config.save(&config_path)?;
    
    println!("\n✓ Configuration saved to: {}", config_path.display());
    println!("\n🚀 Next steps:");
    println!("  1. Run 'verdant-engine config' to adjust settings (optional)");
    println!("  2. Run 'verdant-engine install' to install as Windows service");
    println!("  3. Run 'verdant-engine start' to begin optimization\n");
    
    Ok(())
}

async fn setup_config() -> Result<()> {
    init_logging("info");
    
    let config_path = Config::default_path();
    
    if !config_path.exists() {
        println!("No configuration found. Please run 'verdant-engine auth' first.");
        return Ok(());
    }
    
    let config = Config::load(&config_path)?;
    
    println!("\n📊 Current Configuration\n");
    println!("GitHub:");
    println!("  Username: {}", config.github.username);
    println!("  Repository: {}", config.github.repo_name);
    println!("\nSchedule:");
    println!("  Mode: {:?}", config.schedule.mode);
    println!("  Interval: {}-{} minutes", config.schedule.min_interval_minutes, config.schedule.max_interval_minutes);
    println!("  Timezone Optimization: {}", config.schedule.timezone_optimization);
    println!("  Turbo Multiplier: {}x", config.schedule.turbo_multiplier);
    println!("\nContent:");
    println!("  Commit Style: {:?}", config.content.commit_message_style);
    println!("\nService:");
    println!("  Auto-start: {}", config.service.auto_start);
    println!("  Log Level: {}", config.service.log_level);
    
    println!("\n💡 To modify settings, edit: {}", config_path.display());
    println!("Or use the example below:\n");
    println!("# Turn it up to 11 (TURBO MODE):");
    println!("turbo_multiplier = 11.0");
    println!("\n# Make it even more aggressive:");
    println!("min_interval_minutes = 15");
    println!("max_interval_minutes = 60\n");
    
    Ok(())
}

async fn run_foreground() -> Result<()> {
    let config_path = Config::default_path();
    
    if !config_path.exists() {
        eprintln!("❌ No configuration found. Please run 'verdant-engine auth' first.");
        return Ok(());
    }
    
    let config = Config::load(&config_path)?;
    init_logging(&config.service.log_level);
    
    info!("Starting Verdant™ Engine in foreground mode");
    info!("Press Ctrl+C to stop");
    
    run_engine(config).await
}

async fn run_engine(config: Config) -> Result<()> {
    info!("Initializing GitHub client...");
    
    let mut client = GitHubClient::new(
        config.github.token.clone(),
        config.github.username.clone(),
        config.github.repo_name.clone(),
    )?;
    
    // Verify authentication
    client.verify_auth().await?;
    
    // Check if repo exists
    if !client.check_repo_exists().await? {
        error!("Repository {}/{} not found", config.github.username, config.github.repo_name);
        anyhow::bail!("Repository not found. Please fork mesopera/verdant first.");
    }
    
    // Setup local repository
    client.setup_local_repo().await?;
    
    // Initialize generators
    let mut content_gen = ContentGenerator::new();
    let mut metrics_gen = MetricsGenerator::new();
    
    // Initialize scheduler
    let mut scheduler = Scheduler::new(config.schedule.clone());
    
    info!("✓ Initialization complete");
    info!("Schedule: {}", scheduler.describe());
    info!("Repository: {}/{}", config.github.username, config.github.repo_name);
    
    // Calculate first commit time
    let mut next_commit = scheduler.next_commit_time();
    
    // Main loop
    loop {
        let now = Utc::now();
        
        if now >= next_commit {
            info!("🎯 Executing scheduled commit...");
            
            match execute_commit(&client, &mut content_gen, &mut metrics_gen).await {
                Ok(_) => {
                    info!("✓ Commit cycle completed successfully");
                    // Calculate next commit time
                    next_commit = scheduler.next_commit_time();
                }
                Err(e) => {
                    error!("Failed to execute commit: {}", e);
                    // Retry in 5 minutes
                    next_commit = Utc::now() + chrono::Duration::minutes(5);
                    info!("Will retry at: {}", next_commit.format("%Y-%m-%d %H:%M:%S UTC"));
                }
            }
        } else {
            let time_until = next_commit - now;
            let minutes = time_until.num_minutes();
            debug!("Next commit in {} minutes", minutes);
        }
        
        // Sleep for 1 minute before checking again
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

fn install_service() -> Result<()> {
    #[cfg(windows)]
    {
        service::install_service()
    }
    
    #[cfg(not(windows))]
    {
        eprintln!("❌ Windows service installation is only supported on Windows");
        Ok(())
    }
}

fn uninstall_service() -> Result<()> {
    #[cfg(windows)]
    {
        service::uninstall_service()
    }
    
    #[cfg(not(windows))]
    {
        eprintln!("❌ Windows service is only supported on Windows");
        Ok(())
    }
}

fn start_service() -> Result<()> {
    #[cfg(windows)]
    {
        use windows_service::service::ServiceAccess;
        use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

        println!("Starting Verdant Engine service...");

        let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .map_err(|e| anyhow::anyhow!("Failed to open service manager: {}", e))?;

        let service = manager
            .open_service("VerdantEngine", ServiceAccess::START)
            .map_err(|e| anyhow::anyhow!("Failed to open service: {}", e))?;

        service
            .start(&[] as &[&str])
            .map_err(|e| anyhow::anyhow!("Failed to start service: {}", e))?;

        println!("✓ Service started successfully");
        println!("\n💚 Your GitHub contribution graph optimization is now active!");
        println!("View logs in Windows Event Viewer under 'Application'");

        Ok(())
    }

    #[cfg(not(windows))]
    {
        eprintln!("❌ Windows service is only supported on Windows");
        Ok(())
    }
}

fn stop_service() -> Result<()> {
    #[cfg(windows)]
    {
        use windows_service::service::ServiceAccess;
        use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

        println!("Stopping Verdant Engine service...");

        let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .map_err(|e| anyhow::anyhow!("Failed to open service manager: {}", e))?;

        let service = manager
            .open_service("VerdantEngine", ServiceAccess::STOP)
            .map_err(|e| anyhow::anyhow!("Failed to open service: {}", e))?;

        service
            .stop()
            .map_err(|e| anyhow::anyhow!("Failed to stop service: {}", e))?;

        println!("✓ Service stopped successfully");

        Ok(())
    }

    #[cfg(not(windows))]
    {
        eprintln!("❌ Windows service is only supported on Windows");
        Ok(())
    }
}

fn init_logging(level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));
    
    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
