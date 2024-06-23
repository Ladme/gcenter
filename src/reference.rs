// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

//! Implementation of reference atoms selection.

use colored::Colorize;
use groan_rs::errors::GroupError;
use groan_rs::structures::dimension::Dimension;
use groan_rs::system::System;

use crate::argparse::Args;
use crate::errors::RunError;

/// Create the specified reference group.
fn create_reference(
    system: &mut System,
    name: &str,
    query: &str,
    silent: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let autodetect = match system.group_create(name, query) {
        // ignore group overwrite
        Ok(_) | Err(GroupError::AlreadyExistsWarning(_)) => false,
        // if the reference group is 'Protein' and such group does not exist, try autodetecting the protein atoms
        Err(GroupError::InvalidQuery(_)) if query == "Protein" => {
            match system.group_create(name, "@protein") {
                Ok(_) | Err(GroupError::AlreadyExistsWarning(_)) => {
                    if !silent {
                        println!(
                            "{} group '{}' not found. Autodetected {} protein atoms.\n",
                            "warning:".yellow().bold(),
                            "Protein".yellow(),
                            format!("{}", system.group_get_n_atoms(name).unwrap()).bright_blue()
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
    if system.group_get_n_atoms(name).unwrap() == 0 {
        if !autodetect {
            return Err(Box::new(RunError::EmptyReference(query.to_owned())));
        } else {
            return Err(Box::new(RunError::AutodetectionFailed));
        }
    }

    Ok(())
}

/// Check whether two groups contain the same atoms.
fn groups_are_same(system: &System, name1: &str, name2: &str) -> bool {
    if system.group_get_n_atoms(name1).unwrap() != system.group_get_n_atoms(name2).unwrap() {
        return false;
    }

    // we should maybe check more properties
    for (atom1, atom2) in system
        .group_iter(name1)
        .unwrap()
        .zip(system.group_iter(name2).unwrap())
    {
        if atom1.get_atom_number() != atom2.get_atom_number()
            || atom1.get_atom_name() != atom2.get_atom_name()
        {
            return false;
        }
    }

    true
}

/// Convert references to a vector of centering operations that should be performed.
fn groups2operations<'a>(
    system: &'a System,
    mut groups: [Option<&'a str>; 3],
) -> Vec<(String, Dimension)> {
    let mut operations = Vec::new();
    if let Some(xref) = groups[0] {
        let mut operation = (xref.to_owned(), [true, false, false]);

        for (i, group) in groups.iter_mut().enumerate().skip(1) {
            match group {
                None => (),
                Some(next) => {
                    if groups_are_same(system, xref, next) {
                        operation.1[i] = true;
                        *group = None;
                    }
                }
            }
        }

        operations.push(operation);
    }

    if let Some(yref) = groups[1] {
        let mut operation = (yref.to_owned(), [false, true, false]);

        match groups[2] {
            None => (),
            Some(next) => {
                if groups_are_same(system, yref, next) {
                    operation.1[2] = true;
                    groups[2] = None;
                }
            }
        }

        operations.push(operation);
    }

    if let Some(zref) = groups[2] {
        operations.push((zref.to_owned(), [false, false, true]));
    }

    operations
        .into_iter()
        .map(|x| (x.0, x.1.into()))
        .collect::<Vec<(String, Dimension)>>()
}

/// Select reference atoms for centering.
/// Returns the names of groups to use for centering.
pub fn create_references(
    system: &mut System,
    dim: Dimension,
    args: &Args,
) -> Result<Vec<(String, Dimension)>, Box<dyn std::error::Error + Send + Sync>> {
    // create the main reference group if it is required
    if (args.xreference.is_none() && dim.is_x())
        || (args.yreference.is_none() && dim.is_y())
        || (args.zreference.is_none() && dim.is_z())
    {
        create_reference(system, crate::MAIN_REFERENCE, &args.reference, args.silent)?;
    }

    // no dimension-specific groups
    if args.xreference.is_none() && args.yreference.is_none() && args.zreference.is_none() {
        return Ok(vec![(crate::MAIN_REFERENCE.to_owned(), dim)]);
    }

    // create dimension-specific reference groups
    let mut references = [None; 3];
    for (i, ((query, name), dimension)) in [&args.xreference, &args.yreference, &args.zreference]
        .into_iter()
        .zip([crate::X_REFERENCE, crate::Y_REFERENCE, crate::Z_REFERENCE].into_iter())
        .zip([dim.is_x(), dim.is_y(), dim.is_z()].into_iter())
        .enumerate()
    {
        if !dimension {
            continue;
        }

        match query {
            None => references[i] = Some(crate::MAIN_REFERENCE),
            Some(x) => {
                create_reference(system, name, x, args.silent)?;
                references[i] = Some(name);
            }
        }
    }

    // convert references to list of operations to perform
    Ok(groups2operations(system, references))
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::*;

    #[test]
    fn create_references_main() {
        let command_line = ["gcenter", "-c=tests/test_files/input.gro", "-o=output.gro"];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_main_xy() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "-o=output.gro",
            "-r=@protein",
            "-xy",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XY, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XY);
    }

    #[test]
    fn create_references_yzmain_xref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[1].1, Dimension::YZ);
    }

    #[test]
    fn create_references_yzmain_xref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=resid 1-21",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_ymain_xref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "-o=output.gro",
            "-y",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XY, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
    }

    #[test]
    fn create_references_ymain_xref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@protein",
            "-o=output.gro",
            "-y",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XY, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XY);
    }

    #[test]
    fn create_references_zmain_xref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "-o=output.gro",
            "-z",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Z);
    }

    #[test]
    fn create_references_xzmain_yref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "-r=@protein",
            "--yref=@membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XZ);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
    }

    #[test]
    fn create_references_xzmain_yref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "-r=@protein",
            "--yref=Protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_xmain_yref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "-r=@protein",
            "--yref=@membrane",
            "-o=output.gro",
            "-x",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XY, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
    }

    #[test]
    fn create_references_xymain_zref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--zref=serial 1 to 43",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XY);
        assert_eq!(&operations[1].0, crate::Z_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Z);
    }

    #[test]
    fn create_references_xymain_zref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--zref=serial 1 to 42",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_ymain_zref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--zref=serial 1 to 43",
            "-o=output.gro",
            "-y",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::YZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::Y);
        assert_eq!(&operations[1].0, crate::Z_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Z);
    }

    #[test]
    fn create_references_ymain_zref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--zref=serial 1 to 42",
            "-o=output.gro",
            "-y",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::YZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::YZ);
    }

    #[test]
    fn create_references_zmain_xyref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--yref=@membrane or Protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 3);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
        assert_eq!(&operations[2].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[2].1, Dimension::Z);
    }

    #[test]
    fn create_references_zmain_xyref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--yref=Membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XY);
        assert_eq!(&operations[1].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Z);
    }

    #[test]
    fn create_references_zmain_xyref_all_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--yref=Membrane",
            "-r=Membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_ymain_xzref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--zref=@water",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 3);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
        assert_eq!(&operations[2].0, crate::Z_REFERENCE);
        assert_eq!(operations[2].1, Dimension::Z);
    }

    #[test]
    fn create_references_ymain_xzref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--zref=resname POPC",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XZ);
        assert_eq!(&operations[1].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
    }

    #[test]
    fn create_references_ymain_xzref_all_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=Protein",
            "--zref=@protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_xmain_yzref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--yref=@membrane",
            "--zref=@water",
            "--reference=Protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 3);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
        assert_eq!(&operations[2].0, crate::Z_REFERENCE);
        assert_eq!(operations[2].1, Dimension::Z);
    }

    #[test]
    fn create_references_xmain_yzref_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--yref=@membrane",
            "--zref=@membrane",
            "--reference=Protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::YZ);
    }

    #[test]
    fn create_references_xmain_yzref_all_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--yref=@protein",
            "--zref=resid 1 to 21",
            "--reference=Protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::MAIN_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_xyzref() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--yref=@water",
            "--zref=@ion",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 3);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
        assert_eq!(&operations[2].0, crate::Z_REFERENCE);
        assert_eq!(operations[2].1, Dimension::Z);
    }

    #[test]
    fn create_references_xyzref_all_same() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=Membrane",
            "--yref=@membrane",
            "--zref=resname POPC",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XYZ, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XYZ);
    }

    #[test]
    fn create_references_xyref_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--yref=@water",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XY, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Y_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Y);
    }

    #[test]
    fn create_references_xzref_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "--zref=@water",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XZ, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
        assert_eq!(&operations[1].0, crate::Z_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Z);
    }

    #[test]
    fn create_references_xzref_same_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@protein",
            "--zref=Protein",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::XZ, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::XZ);
    }

    #[test]
    fn create_references_yzref_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--yref=@membrane",
            "--zref=@water",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::YZ, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 2);
        assert_eq!(&operations[0].0, crate::Y_REFERENCE);
        assert_eq!(operations[0].1, Dimension::Y);
        assert_eq!(&operations[1].0, crate::Z_REFERENCE);
        assert_eq!(operations[1].1, Dimension::Z);
    }

    #[test]
    fn create_references_xref_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--xref=@membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::X, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::X_REFERENCE);
        assert_eq!(operations[0].1, Dimension::X);
    }

    #[test]
    fn create_references_yref_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--yref=@membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::Y, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(system.group_exists(crate::Y_REFERENCE));
        assert!(!system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::Y_REFERENCE);
        assert_eq!(operations[0].1, Dimension::Y);
    }

    #[test]
    fn create_references_zref_nomain() {
        let command_line = [
            "gcenter",
            "-c=tests/test_files/input.gro",
            "--zref=@membrane",
            "-o=output.gro",
        ];
        let args = Args::parse_from(command_line);

        let mut system = System::from_file("tests/test_files/input.gro").unwrap();
        system.read_ndx("tests/test_files/index.ndx").unwrap();

        let operations = create_references(&mut system, Dimension::Z, &args).unwrap();

        assert!(!system.group_exists(crate::MAIN_REFERENCE));
        assert!(!system.group_exists(crate::X_REFERENCE));
        assert!(!system.group_exists(crate::Y_REFERENCE));
        assert!(system.group_exists(crate::Z_REFERENCE));

        assert_eq!(operations.len(), 1);
        assert_eq!(&operations[0].0, crate::Z_REFERENCE);
        assert_eq!(operations[0].1, Dimension::Z);
    }
}
