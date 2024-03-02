// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

//! Implementation of reference atoms selection.

use colored::Colorize;
use groan_rs::errors::GroupError;
use groan_rs::system::System;

use crate::argparse::Args;
use crate::errors::RunError;

/// Select reference atoms for centering.
pub fn create_reference(
    system: &mut System,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let autodetect = match system.group_create("Reference", &args.reference) {
        // ignore group overwrite
        Ok(_) | Err(GroupError::AlreadyExistsWarning(_)) => false,
        // if the reference group is 'Protein' and such group does not exist, try autodetecting the protein atoms
        Err(GroupError::InvalidQuery(_)) if &args.reference == "Protein" => {
            match system.group_create("Reference", "@protein") {
                Ok(_) | Err(GroupError::AlreadyExistsWarning(_)) => {
                    if !args.silent {
                        println!(
                            "{} group '{}' not found. Autodetected {} protein atoms.\n",
                            "warning:".yellow().bold(),
                            "Protein".yellow(),
                            format!("{}", system.group_get_n_atoms("Reference").unwrap())
                                .bright_blue()
                        );
                    }

                    true
                }
                Err(_) => panic!("\ngcenter: Fatal Error. Autodetection failed."),
            }
        }
        // propagate all the other errors
        Err(e) => return Err(Box::from(e)),
    };

    // check that the reference group is not empty
    if system.group_get_n_atoms("Reference").unwrap() == 0 {
        if !autodetect {
            return Err(Box::new(RunError::EmptyReference(
                args.reference.to_owned(),
            )));
        } else {
            return Err(Box::new(RunError::AutodetectionFailed));
        }
    }

    Ok(())
}
