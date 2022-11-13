//! Author extraction for cargo manifest (c) 2018 Cargo Developers.
//!
//! Obtained from [Cargo](https://github.com/rust-lang/cargo/blob/fa05862cd0c6b899b801fda0f256ac5b9bae69d9/src/cargo/ops/cargo_new.rs#L690-L750).

use anyhow::Error;
use git2::{Config as GitConfig, Repository as GitRepository};

use std::env;

fn get_environment_variable(variables: &[&str]) -> Option<String> {
    variables.iter().filter_map(|var| env::var(var).ok()).next()
}

/// Attempts to find the name and email of the author from the current environment.
pub(super) fn discover() -> Result<(String, Option<String>), Error> {
    let cwd = env::current_dir()?;
    let git_config = if let Ok(repo) = GitRepository::discover(cwd) {
        repo.config()
            .ok()
            .or_else(|| GitConfig::open_default().ok())
    } else {
        GitConfig::open_default().ok()
    };
    let git_config = git_config.as_ref();
    let name_variables = [
        "CARGO_NAME",
        "GIT_AUTHOR_NAME",
        "GIT_COMMITTER_NAME",
        "USER",
        "USERNAME",
        "NAME",
    ];
    let name = get_environment_variable(&name_variables[0..3])
        .or_else(|| git_config.and_then(|g| g.get_string("user.name").ok()))
        .or_else(|| get_environment_variable(&name_variables[3..]));

    let name = match name {
        Some(name) => name,
        None => {
            let username_var = if cfg!(windows) { "USERNAME" } else { "USER" };
            anyhow::bail!(
                "could not determine the current user, please set ${}",
                username_var
            )
        }
    };
    let email_variables = [
        "CARGO_EMAIL",
        "GIT_AUTHOR_EMAIL",
        "GIT_COMMITTER_EMAIL",
        "EMAIL",
    ];
    let email = get_environment_variable(&email_variables[0..3])
        .or_else(|| git_config.and_then(|g| g.get_string("user.email").ok()))
        .or_else(|| get_environment_variable(&email_variables[3..]));

    let name = name.trim().to_string();
    let email = email.map(|s| {
        let mut s = s.trim();

        // In some cases emails will already have <> remove them since they
        // are already added when needed.
        if s.starts_with('<') && s.ends_with('>') {
            s = &s[1..s.len() - 1];
        }

        s.to_string()
    });

    Ok((name, email))
}
