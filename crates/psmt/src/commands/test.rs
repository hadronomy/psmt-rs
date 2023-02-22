use std::env;
use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use eyre::{Context, Result};
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, RemoteCallbacks};
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use libpsmt::Error;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command()]
pub struct TestCommand {
    /// The template repository url
    repository: String,
}

impl TestCommand {
    pub fn exec(&self) -> Result<()> {
        let multiprogress = Arc::new(MultiProgress::new());
        let spinner = multiprogress.add(ProgressBar::new_spinner());
        spinner.set_style(
            ProgressStyle::with_template("{spinner:.cyan/} {msg}")
                .wrap_err("Failed to create template style")?,
        );
        spinner.set_message("Cloning repo");
        spinner.enable_steady_tick(Duration::from_millis(100));
        let progress_bar = multiprogress.add(ProgressBar::new(0));
        progress_bar.set_style(ProgressStyle::with_template("{msg}{spinner:.green} {elapsed_precise:.cyan/blue} {wide_bar:.green/red} {pos}/{len} ({eta})")
            .wrap_err("Failed to create template style")?
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("━⭰━"));
        progress_bar.enable_steady_tick(Duration::from_millis(100));
        let mut callback = RemoteCallbacks::new();
        callback.transfer_progress(|stats| {
            progress_bar.set_length(stats.total_objects() as u64);
            progress_bar.set_position(stats.received_objects() as u64);
            true
        });
        let pwd = env::current_dir().wrap_err("Failed to obtain current working directory")?;
        let psmt_dir = Path::new(pwd.as_os_str()).join(".psmt");
        let repo_dir = psmt_dir.join("template");
        let checkout_builder = CheckoutBuilder::new();
        let mut options = FetchOptions::new();
        options.remote_callbacks(callback);
        let mut builder = RepoBuilder::new();
        let repo = builder
            .fetch_options(options)
            .with_checkout(checkout_builder);
        match repo.clone(&self.repository, repo_dir.as_path()) {
            Ok(_) => {
                progress_bar.finish_with_message("✔".green().bold().to_string());
                progress_bar.finish_and_clear();
                spinner.suspend(|| {
                    eprintln!("{} Cloning Repo", "✔ ".green().bold());
                });
                spinner.finish_and_clear();
                Ok(())
            }
            Err(err) => {
                // Use Emoji for Printing emojis
                progress_bar.abandon_with_message("❌");
                Err(Error::Unknown(err.to_string()).into())
            }
        }
    }
}
