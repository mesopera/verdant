use anyhow::Result;
use crate::github::GitHubClient;
use crate::generator::{ContentGenerator, MetricsGenerator};
use tracing::{info, error};

pub async fn execute_commit(
    client: &GitHubClient,
    content_gen: &mut ContentGenerator,
    metrics_gen: &mut MetricsGenerator,
) -> Result<()> {
    info!("Executing commit cycle...");

    // Generate new content
    let (filename, content, commit_message) = content_gen.generate();
    
    // Create and push the commit
    match client.create_commit(&filename, &content, &commit_message).await {
        Ok(_) => {
            info!("✓ Successfully created commit");
            
            // Also update metrics occasionally (20% chance)
            if rand::random::<f32>() < 0.2 {
                update_metrics(client, metrics_gen).await?;
            }
            
            Ok(())
        }
        Err(e) => {
            error!("Failed to create commit: {}", e);
            Err(e)
        }
    }
}

async fn update_metrics(
    client: &GitHubClient,
    metrics_gen: &mut MetricsGenerator,
) -> Result<()> {
    info!("Updating metrics data...");
    
    let metrics_json = metrics_gen.generate_json()
        .map_err(|e| anyhow::anyhow!("Failed to generate metrics JSON: {}", e))?;
    
    client.create_commit(
        "frontend/data/metrics.json",
        &metrics_json,
        "Update real-time purr-formance analytics dashboard with latest enterprise metrics",
    ).await?;
    
    info!("✓ Metrics updated");
    Ok(())
}
