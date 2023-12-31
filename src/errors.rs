// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

//! Implementation of errors originating from the `gcenter` program.

use colored::Colorize;
use thiserror::Error;

/// Errors originating directly from `gcenter`.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum RunError {
    #[error("{} invalid value '{}' for '{}': output path matches input path\n\nFor more information, try '{}'.", "error:".red().bold(), .0.yellow(), "--output <OUTPUT>".bold(), "--help".bold())]
    IOMatch(String),
    #[error("{} invalid value '{}' for '{}': unsupported file extension\n\nFor more information, try '{}'.", "error:".red().bold(), .0.yellow(), "--output <OUTPUT>".bold(), "--help".bold())]
    OutputUnsupported(String),
    #[error("{} invalid values '{}' and '{}' for '{}': paths correspond to the same file\n\nFor more information, try '{}'.", "error:".red().bold(), .0.yellow(), .1.yellow(), "--trajectory [<TRAJECTORIES>...]".bold(), "--help".bold())]
    IdenticalInputFiles(String, String),
    #[error("{} invalid value '{}' for '{}': when multiple input trajectories are provided, <STEP> must be 1\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--step <STEP>".bold(), "--help".bold())]
    StepJoinUnsupported(usize),
    #[error("{} invalid value '{}' for '{}': input structure file does not exist\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--structure <STRUCTURE>".bold(), "--help".bold())]
    InputStructureNotFound(String),
    #[error("{} invalid value '{}' for '{}': input trajectory file does not exist\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--trajectory [<TRAJECTORIES>...]".bold(), "--help".bold())]
    InputTrajectoryNotFound(String),
    #[error("{} reference group '{}' is empty\n", "error:".red().bold(), .0.yellow())]
    EmptyReference(String),
    #[error("{} no protein atoms autodetected\n", "error:".red().bold())]
    AutodetectionFailed,
    #[error("{} simulation box is not orthogonal; this is not supported, sorry\n", "error:".red().bold())]
    BoxNotOrthogonal,
    #[error("{} simulation box is not a valid simulation box; some required dimensions are not positive\n", "error:".red().bold())]
    BoxNotValid,
}

/* // [DEV] print all RunErrors
pub fn print_all_errors() {
    let errors = vec![
        RunError::IOMatch(String::from("N/A")),
        RunError::OutputUnsupported(String::from("N/A")),
        RunError::IdenticalInputFiles(String::from("N/A"), String::from("N/A")),
        RunError::StepJoinUnsupported(3),
        RunError::InputStructureNotFound(String::from("N/A")),
        RunError::InputTrajectoryNotFound(String::from("N/A")),
        RunError::EmptyReference(String::from("N/A")),
        RunError::AutodetectionFailed,
        RunError::BoxNotOrthogonal,
        RunError::BoxNotValid,
    ];

    for e in errors {
        println!("{}", e);
    }
}
*/
