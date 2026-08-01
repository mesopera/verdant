use anyhow::{Context, Result};
use octocrab::Octocrab;
use git2::{Repository, Signature};
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug};

pub struct GitHubClient {
    octocrab: Octocrab,
    token: String,
    username: String,
    repo_name: String,
    local_repo_path: Option<PathBuf>,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(token: String, username: String, repo_name: String) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(token.clone())
            .build()
            .context("Failed to create GitHub client")?;
        
        Ok(Self {
            octocrab,
            token,
            username,
            repo_name,
            local_repo_path: None,
        })
    }

    /// Verify that the client can authenticate
    pub async fn verify_auth(&self) -> Result<()> {
        let user = self.octocrab.current().user().await
            .context("Failed to authenticate with GitHub")?;
        
        info!("✓ Authenticated as: {}", user.login);
        Ok(())
    }

    /// Check if repository exists
    pub async fn check_repo_exists(&self) -> Result<bool> {
        match self.octocrab
            .repos(&self.username, &self.repo_name)
            .get()
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Clone or open repository locally
    pub async fn setup_local_repo(&mut self) -> Result<PathBuf> {
        let temp_dir = std::env::temp_dir().join("verdant-repo");
        
        if temp_dir.exists() {
            debug!("Local repository already exists at {:?}", temp_dir);
            // Try to open existing repo
            match Repository::open(&temp_dir) {
                Ok(_) => {
                    self.local_repo_path = Some(temp_dir.clone());
                    return Ok(temp_dir);
                }
                Err(_) => {
                    // Remove corrupted repo
                    fs::remove_dir_all(&temp_dir)?;
                }
            }
        }

        // Clone repository
        let url = format!("https://github.com/{}/{}.git", self.username, self.repo_name);
        info!("Cloning repository: {}", url);
        
        let _repo = Repository::clone(&url, &temp_dir)
            .context("Failed to clone repository")?;
        
        info!("✓ Repository cloned to {:?}", temp_dir);
        self.local_repo_path = Some(temp_dir.clone());
        
        Ok(temp_dir)
    }

    /// Create and push a commit with content
    pub async fn create_commit(
        &self,
        file_path: &str,
        content: &str,
        commit_message: &str,
    ) -> Result<()> {
        let repo_path = self.local_repo_path.as_ref()
            .context("Local repository not set up. Call setup_local_repo first.")?;
        
        let repo = Repository::open(repo_path)
            .context("Failed to open local repository")?;

        // Pull latest changes first
        self.pull_latest(&repo).await?;

        // Write file content
        let full_path = repo_path.join(file_path);
        
        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&full_path, content)
            .context("Failed to write file content")?;

        // Stage the file
        let mut index = repo.index()?;
        index.add_path(Path::new(file_path))?;
        index.write()?;

        // Create commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let parent_commit = repo.head()?.peel_to_commit()?;
        
        let signature = Signature::now("Verdant Engine", "verdant@feline.corp")
            .context("Failed to create commit signature")?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent_commit],
        )?;

        info!("✓ Created commit: {}", commit_message);

        // Push to remote
        self.push_to_remote(&repo).await?;

        Ok(())
    }

    /// Pull latest changes from remote
    async fn pull_latest(&self, repo: &Repository) -> Result<()> {
        debug!("Pulling latest changes...");
        
        // This is a simplified pull - in production, handle merge conflicts
        let mut remote = repo.find_remote("origin")?;
        let fetch_commit = self.do_fetch(&repo, &["main"], &mut remote)?;
        self.do_merge(&repo, "main", fetch_commit)?;
        
        Ok(())
    }

    fn do_fetch<'a>(
        &self,
        repo: &'a Repository,
        refs: &[&str],
        remote: &'a mut git2::Remote,
    ) -> Result<git2::AnnotatedCommit<'a>> {
        let mut cb = git2::RemoteCallbacks::new();
        
        // Add authentication for fetch
        let token = self.token.clone();
        cb.credentials(move |_url, _username_from_url, _allowed_types| {
            git2::Cred::userpass_plaintext("x-access-token", &token)
        });
        
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(cb);
        
        remote.fetch(refs, Some(&mut fo), None)?;
        
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        Ok(repo.reference_to_annotated_commit(&fetch_head)?)
    }

    fn do_merge(
        &self,
        repo: &Repository,
        remote_branch: &str,
        fetch_commit: git2::AnnotatedCommit,
    ) -> Result<()> {
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        
        if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", remote_branch);
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else if analysis.0.is_normal() {
            // For simplicity, we'll just use their version
            let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
            self.normal_merge(&repo, &head_commit, &fetch_commit)?;
        }
        
        Ok(())
    }

    fn normal_merge(
        &self,
        repo: &Repository,
        local: &git2::AnnotatedCommit,
        remote: &git2::AnnotatedCommit,
    ) -> Result<()> {
        let local_tree = repo.find_commit(local.id())?.tree()?;
        let remote_tree = repo.find_commit(remote.id())?.tree()?;
        let ancestor = repo.find_commit(repo.merge_base(local.id(), remote.id())?)?.tree()?;
        
        let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;
        
        if idx.has_conflicts() {
            repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }
        
        let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
        let sig = Signature::now("Verdant Engine", "verdant@feline.corp")?;
        
        let local_commit = repo.find_commit(local.id())?;
        let remote_commit = repo.find_commit(remote.id())?;
        
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Merge",
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;
        
        repo.checkout_head(None)?;
        Ok(())
    }

    /// Push commits to remote
    async fn push_to_remote(&self, repo: &Repository) -> Result<()> {
        debug!("Pushing to remote...");
        
        let mut remote = repo.find_remote("origin")?;
        
        // Setup authentication with GitHub token
        let mut callbacks = git2::RemoteCallbacks::new();
        let token = self.token.clone();
        callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            git2::Cred::userpass_plaintext("x-access-token", &token)
        });
        
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        remote.push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )?;
        
        info!("✓ Pushed to remote");
        Ok(())
    }

    /// Get contribution data (for analytics)
    #[allow(dead_code)]
    pub async fn get_contribution_count(&self, days: u32) -> Result<u32> {
        // This is simplified - GitHub's GraphQL API is better for this
        // For now, return a mock value
        Ok(days * 2) // Mock: ~2 contributions per day
    }
}
