use anyhow::{Context, Result};
use std::io::{self, Write};

/// Prompt user to create a GitHub Personal Access Token
pub fn authenticate() -> Result<String> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Verdant™ - GitHub Authentication Setup                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    println!("To optimize your contribution graph, Verdant™ needs access to your");
    println!("GitHub account with repository permissions.\n");
    
    println!("Please follow these steps:\n");
    println!("1. Visit: https://github.com/settings/tokens/new");
    println!("2. Give it a name: 'Verdant Engine'");
    println!("3. Select scopes: 'repo' (Full control of private repositories)");
    println!("4. Click 'Generate token'");
    println!("5. Copy the token (ghp_...)\n");
    
    print!("Paste your GitHub Personal Access Token: ");
    io::stdout().flush()?;
    
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .context("Failed to read token")?;
    
    let token = token.trim().to_string();
    
    if token.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }
    
    if !token.starts_with("ghp_") && !token.starts_with("github_pat_") {
        println!("\n⚠️  Warning: Token doesn't look like a GitHub PAT (should start with 'ghp_' or 'github_pat_')");
        print!("Continue anyway? [y/N]: ");
        io::stdout().flush()?;
        
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm)?;
        
        if !confirm.trim().eq_ignore_ascii_case("y") {
            anyhow::bail!("Authentication cancelled");
        }
    }
    
    println!("\n✓ Token received. Validating...\n");
    
    Ok(token)
}

/// Prompt for GitHub username
pub fn get_username() -> Result<String> {
    print!("Enter your GitHub username: ");
    io::stdout().flush()?;
    
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .context("Failed to read username")?;
    
    let username = username.trim().to_string();
    
    if username.is_empty() {
        anyhow::bail!("Username cannot be empty");
    }
    
    Ok(username)
}
