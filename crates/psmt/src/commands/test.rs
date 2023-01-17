use std::fmt::Write;
use std::{path::Path};
use std::env;

use clap::Parser;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, RemoteCallbacks};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use libpsmt::Error;

#[derive(Parser, Debug)]
#[command()]
pub struct TestCommand {
    repository: String,
}

impl TestCommand {
    pub fn exec(&self) -> Result<(), Error> {
        let pwd = env::current_dir().unwrap();
        let psmt_dir = Path::new(pwd.as_os_str()).join(".psmt");
        let repo_dir = psmt_dir.join("template");
        let progress_bar = ProgressBar::new(0);
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise:.cyan/blue}] [{wide_bar:.green/red}] {pos}/{len} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("━⭰━"));
        let mut callback = RemoteCallbacks::new();
        callback.transfer_progress(|stats| {
            progress_bar.set_length(stats.total_objects() as u64);
            progress_bar.set_position(stats.received_objects() as u64);
            true
        });
        let checkout_builder = CheckoutBuilder::new();
        let mut options = FetchOptions::new();
        options.remote_callbacks(callback);
        let mut builder = RepoBuilder::new();
        let repo = builder
            .fetch_options(options)
            .with_checkout(checkout_builder);
        match repo.clone(&self.repository, &repo_dir.as_path()) {
            Ok(_) => {
                progress_bar.finish_with_message("Done");
                Ok(())
            },
            Err(err) => {
                progress_bar.finish_with_message("Error");
                Err(Error::Unknown(err.to_string()))
            },
        }
    }
}
