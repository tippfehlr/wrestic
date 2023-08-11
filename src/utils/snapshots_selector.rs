use std::{process::Command, time::Duration};

use anyhow::Result;
use color_print::cformat;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use regex::Regex;

pub fn snapshots_selector(bucket: &str, repository: &str) -> Result<String> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading snapshots...");
    let restic = Command::new("restic")
        .arg("-r")
        .arg(format!("b2:{}:{}", bucket, repository))
        .arg("--verbose")
        .arg("--verbose")
        .arg("snapshots")
        .output()?;
    pb.finish_and_clear();

    let restic = String::from_utf8(restic.stdout)?;

    let restic_rev = restic
        .lines()
        .rev()
        .collect::<Vec<&str>>()
        .to_owned()
        .join("\n");

    let selections = Regex::new(r"(\w+)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")?
        .captures_iter(&restic_rev)
        .map(|cap| format!("[{}] - {}", &cap[1], &cap[2]))
        .collect::<Vec<String>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<g>Snapshots:"))
        .default(0)
        .max_length(10)
        .items(&selections[..])
        .interact()?;

    let selection = Regex::new(r"(\w+)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")?
        .captures_iter(&restic_rev)
        .map(|cap| format!("{}", &cap[1]))
        .collect::<Vec<String>>()[selection]
        .clone();

    Ok(selection)
}