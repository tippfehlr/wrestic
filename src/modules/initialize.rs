use crate::{
    modules::selector::selector,
    utils::{
        get_config::Settings,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::env;

pub fn initialize(settings: &Vec<Settings>, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>INITIALIZE REPOSITORIES");
    println!();

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!(
            "<y>Do you want to initialize all repositories? (Y/n): "
        ))
        .default(true)
        .interact()?
    {
        for conf in settings {
            let backend = &conf.backend;
            let bucket = &conf.bucket;
            let repository = &conf.repository;

            env::set_var("USER", &conf.user);
            env::set_var("RESTIC_PASSWORD", &conf.restic_password);
            for env in &conf.env {
                for (key, value) in env {
                    env::set_var(key, value);
                }
            }

            if run_cmd!(
                restic -r $backend:$bucket:$repository init;
            )
            .is_err()
            {
                cprintln!(
                    "<g>Repository: <c>{repository}</c> already exists in bucket: <c>{bucket}</c></g>"
                );
            }
        }
        pause()?;
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
