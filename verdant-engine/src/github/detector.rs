use anyhow::Result;
use octocrab::Octocrab;
use tracing::{info, warn};

/// Detect the user's fork of the verdant repository
pub async fn detect_repo(token: &str, username: &str) -> Result<String> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;
    
    // Check if user is mesopera (original repo owner)
    if username == "mesopera" {
        info!("✓ Using original repository: mesopera/verdant");
        return Ok("verdant".to_string());
    }
    
    // Check if user has a fork
    info!("Checking for fork of mesopera/verdant...");
    
    match octocrab
        .repos(username, "verdant")
        .get()
        .await
    {
        Ok(repo) => {
            if repo.fork == Some(true) {
                info!("✓ Found fork: {}/verdant", username);
                Ok("verdant".to_string())
            } else {
                info!("✓ Found repository: {}/verdant", username);
                Ok("verdant".to_string())
            }
        }
        Err(_) => {
            warn!("No fork found. You should fork mesopera/verdant first.");
            println!("\n╔════════════════════════════════════════════════════════════════╗");
            println!("║  Fork Required                                                 ║");
            println!("╚════════════════════════════════════════════════════════════════╝\n");
            println!("Please fork the repository first:");
            println!("1. Visit: https://github.com/mesopera/verdant");
            println!("2. Click 'Fork' button");
            println!("3. Run this setup again\n");
            
            anyhow::bail!("Repository {}/verdant not found", username)
        }
    }
}
