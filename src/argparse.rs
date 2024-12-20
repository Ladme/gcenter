// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

//! Implementation of a command line argument parser.

use std::path::Path;

use clap::Parser;
use groan_rs::files::FileType;

use crate::errors::RunError;

// Center Gromacs trajectory or structure file.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Center your chosen group within a Gromacs trajectory or structure file effortlessly using the Bai & Breen algorithm.\n
With `gcenter`, you can accurately center atom groups, even when they span multiple molecules that may extend beyond the box boundaries.
Be aware that `gcenter` exclusively supports orthogonal simulation boxes."
)]
pub struct Args {
    #[arg(
        short = 's',
        long = "structure",
        help = "Input structure file",
        long_help = "Path to a gro, pdb, pqr, or tpr file containing the system structure. If a trajectory is also provided, the coordinates from the structure file are ignored.",
        value_parser = validate_structure_type,
    )]
    pub structure: String,

    #[arg(
        short = 'f',
        long = "trajectory",
        help = "Input trajectory file(s)",
        long_help = "Path to xtc or trr file(s) or to a single gro file containing the trajectory (or trajectories) to be manipulated. 
If not provided, the centering operation will use the structure file itself.
Multiple files separated by whitespace can be provided. These will be concatenated into one output file.
All trajectory files must be of the same type (i.e., all must be either xtc or trr files).
When joining trajectories, the last frame of each trajectory and the first frame of the following trajectory are checked for matching simulation steps. 
If the simulation steps coincide, only the first of these frames is centered and written to output.",
        num_args = 0..,
        value_parser = validate_trajectory_type,
    )]
    pub trajectories: Vec<String>,

    #[arg(
        short = 'n',
        long = "index",
        help = "Input index file [default: index.ndx]",
        long_help = "Path to an ndx file containing groups associated with the system.\n\n[default: index.ndx]"
    )]
    pub index: Option<String>,

    #[arg(
        short = 'o',
        long = "output",
        help = "Output file name",
        long_help = "Name of the output file, which can be in gro, pdb, or pqr format if no trajectory is provided, 
or in xtc, trr, or gro format if a trajectory is provided."
    )]
    pub output: String,

    #[arg(
        short = 'r',
        long = "reference",
        help = "Group to center",
        default_value = "Protein",
        long_help = "Specify the group to be centered. Define the group using the VMD-like 'groan selection language', which also supports ndx group names."
    )]
    pub reference: String,

    #[arg(
        short = 'b',
        long = "begin",
        help = "Time of the first frame to read (in ps) [default: 0.0]",
        requires = "trajectories",
        long_help = "Time of the first frame to read from the trajectory (in ps). All previous frames will be skipped.
This option is only applicable when trajectory file(s) is/are provided.
This option cannot be used when the trajectory is a gro file since gro files are not guaranteed to contain simulation time information.\n\n[default: 0.0]"
    )]
    pub start_time: Option<f32>,

    #[arg(
        short = 'e',
        long = "end",
        help = "Time of the last frame to read (in ps) [default: NaN]",
        requires = "trajectories",
        long_help = "Time of the last frame to read from the trajectory (in ps). All following frames will be skipped.
This option is only applicable when trajectory file(s) is/are provided.
This option cannot be used when the trajectory is a gro file since gro files are not guaranteed to contain simulation time information.\n\n[default: NaN]"
    )]
    pub end_time: Option<f32>,

    #[arg(
        short = 't',
        long = "step",
        help = "Write every <STEP>th frame",
        default_value_t = 1,
        requires = "trajectories",
        long_help = "Center and write only every <STEP>th frame of the trajectory to the output file.
This option is only applicable when trajectory file(s) is/are provided."
    )]
    pub step: usize,

    #[arg(
        short = 'x',
        action,
        help = "Center in the x dimension",
        default_value_t = false,
        long_help = "Perform centering operation in the x-dimension. This can be combined with other dimensions. If no dimensions are selected, it defaults to '-xyz'."
    )]
    pub xdimension: bool,

    #[arg(
        short = 'y',
        action,
        help = "Center in the y dimension",
        default_value_t = false,
        long_help = "Perform centering operation in the y-dimension. This can be combined with other dimensions. If no dimensions are selected, it defaults to '-xyz'."
    )]
    pub ydimension: bool,

    #[arg(
        short = 'z',
        action,
        help = "Center in the z dimension",
        default_value_t = false,
        long_help = "Perform centering operation in the z-dimension. This can be combined with other dimensions. If no dimensions are selected, it defaults to '-xyz'."
    )]
    pub zdimension: bool,

    #[arg(
        long = "xref",
        help = "Group to center in the x dimension",
        long_help = "Center the specified selection of atoms along the x dimension. 
This option, in conjunction with `yref` and `zref`, allows you to center multiple groups, each along a different dimension. 
Define the group using the VMD-like 'groan selection language', which also supports ndx group names. 
This selection acts as the reference selection for the x dimension, while the `reference` selection will still be centered in other specified dimensions."
    )]
    pub xreference: Option<String>,

    #[arg(
        long = "yref",
        help = "Group to center in the y dimension",
        long_help = "Center the specified selection of atoms along the y dimension. 
This option, in conjunction with `xref` and `zref`, allows you to center multiple groups, each along a different dimension. 
Define the group using the VMD-like 'groan selection language', which also supports ndx group names. 
This selection acts as the reference selection for the y dimension, while the `reference` selection will still be centered in other specified dimensions."
    )]
    pub yreference: Option<String>,

    #[arg(
        long = "zref",
        help = "Group to center in the z dimension",
        long_help = "Center the specified selection of atoms along the z dimension. 
This option, in conjunction with `xref` and `yref`, allows you to center multiple groups, each along a different dimension. 
Define the group using the VMD-like 'groan selection language', which also supports ndx group names. 
This selection acts as the reference selection for the z dimension, while the `reference` selection will still be centered in other specified dimensions."
    )]
    pub zreference: Option<String>,

    #[arg(
        long = "com",
        action,
        help = "Use center of mass",
        default_value_t = false,
        long_help = "Use center of mass instead of center of geometry when centering the reference group. This requires information about atom masses. 
If they are not explicitly provided using a tpr file, the masses are guessed."
    )]
    pub com: bool,

    #[arg(
        long = "whole",
        action,
        help = "Keep molecules whole",
        default_value_t = false,
        long_help = "Do not wrap all atoms into the simulation box but keep molecules whole. This requires providing a tpr file as an input structure file."
    )]
    pub whole: bool,

    #[arg(
        long = "silent",
        action,
        help = "Suppress standard output",
        default_value_t = false,
        long_help = "Suppress all standard output generated by the 'gcenter' tool, except for error messages written to stderr."
    )]
    pub silent: bool,

    #[arg(
        long = "overwrite",
        action,
        help = "Overwrite existing files with the same name",
        default_value_t = false,
        long_help = "Enable this option to overwrite existing files with the same name as the output file. No backup copies will be created."
    )]
    pub overwrite: bool,
}

/// Validate that the structure is gro or pdb file.
fn validate_structure_type(s: &str) -> Result<String, String> {
    match FileType::from_name(s) {
        FileType::GRO | FileType::PDB | FileType::TPR | FileType::PQR => Ok(s.to_owned()),
        _ => Err(String::from("unsupported file extension")),
    }
}

/// Validate that the trajectories are xtc or trr files.
/// Validate that no trajectory is provided multiple times.
fn validate_trajectory_type(s: &str) -> Result<String, String> {
    match FileType::from_name(s) {
        FileType::XTC | FileType::TRR | FileType::GRO => Ok(s.to_owned()),
        _ => Err(String::from("unsupported file extension")),
    }
}

/// Returns true if a query contains "molecule with" keyword or its alternatives.
fn query_contains_molecule_with(query: &str) -> bool {
    query.contains("molecule with") || query.contains("mol with") || query.contains("molwith")
}

/// Checks that the GSL queries used for references do not contain any unsupported keywords.
fn validate_queries(args: &Args, input_type: FileType) -> Result<(), RunError> {
    if input_type == FileType::TPR {
        return Ok(());
    }

    for (reference, name) in [&args.xreference, &args.yreference, &args.zreference]
        .into_iter()
        .zip(
            [
                "--xref <XREFERENCE>",
                "--yref <YREFERENCE>",
                "--zref <ZREFERENCE>",
            ]
            .into_iter(),
        )
    {
        if let Some(x) = reference {
            if query_contains_molecule_with(x) {
                return Err(RunError::UnsupportedQuery(x.to_owned(), name.to_owned()));
            }
        }
    }

    if query_contains_molecule_with(&args.reference) {
        return Err(RunError::UnsupportedQuery(
            args.reference.to_owned(),
            "--reference <REFERENCE>".to_owned(),
        ));
    }

    Ok(())
}

/// Perform various sanity checks:
/// a) Check that the input and output files are not identical.
/// This protects the user from accidentaly overwriting their data.
/// b) Check that the output file has the correct file extension.
fn sanity_check_inputs(args: &Args) -> Result<(), RunError> {
    // check that the input structure exists
    if !Path::new(&args.structure).exists() {
        return Err(RunError::InputStructureNotFound(args.structure.to_string()));
    }

    let input_type = FileType::from_name(&args.structure);

    // validate that the GSL queries do not contain any unsupported keywords
    validate_queries(args, input_type)?;

    // check that `whole` is only used when a tpr file is provided
    if args.whole && input_type != FileType::TPR {
        return Err(RunError::WholeRequiresTprFile);
    }

    // check for input-output matches
    if args.trajectories.is_empty() {
        if args.structure == args.output {
            return Err(RunError::IOMatch(args.structure.to_string()));
        }
    } else {
        for (t, traj) in args.trajectories.iter().enumerate() {
            // check that the trajectory exists
            if !Path::new(traj).exists() {
                return Err(RunError::InputTrajectoryNotFound(traj.to_string()));
            }

            // check that the trajectory does not match the output
            if traj.as_str() == args.output {
                return Err(RunError::IOMatch(traj.to_string()));
            }

            // check that if there is multiple trajectories, none are GRO files
            if t > 0 && FileType::from_name(traj) == FileType::GRO {
                return Err(RunError::OnlyOneGroTrajectory(traj.to_owned()));
            }

            for traj2 in args.trajectories.iter().skip(t + 1) {
                // check that no other trajectory file matches this one
                if traj == traj2 {
                    return Err(RunError::IdenticalInputFiles(
                        traj.to_owned(),
                        traj2.to_owned(),
                    ));
                }

                // check that all the trajectories have the same type
                if FileType::from_name(traj) != FileType::from_name(traj2) {
                    return Err(RunError::InconsistentTrajectoryFiles(
                        traj.to_owned(),
                        traj2.to_owned(),
                    ));
                }
            }
        }
    }

    // check that if `start_time` or `end_time` is provided, trajectory is not a gro file
    if let Some(file) = args.trajectories.first() {
        let file_type = FileType::from_name(file);

        if file_type == FileType::GRO {
            if args.start_time.is_some() {
                return Err(RunError::BeginNotSupportedForGro(
                    args.start_time.unwrap().to_string(),
                ));
            }

            if args.end_time.is_some() {
                return Err(RunError::EndNotSupportedForGro(
                    args.end_time.unwrap().to_string(),
                ));
            }
        }
    }

    // check the extension of the output file
    let output_type = FileType::from_name(&args.output);
    match (args.trajectories.is_empty(), output_type) {
        (true, FileType::GRO | FileType::PDB | FileType::PQR) => Ok(()),
        (true, _) => Err(RunError::OutputUnsupported(args.output.clone())),
        (false, FileType::XTC | FileType::TRR | FileType::GRO) => Ok(()),
        (false, _) => Err(RunError::OutputUnsupported(args.output.clone())),
    }
}

pub fn parse() -> Result<Args, Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    sanity_check_inputs(&args)?;

    Ok(args)
}
