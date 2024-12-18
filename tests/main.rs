// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

#[cfg(test)]
mod pass_tests {
    use assert_cmd::Command;
    use std::fs::{self, File};
    use std::io::prelude::*;
    use tempfile::Builder;

    #[test]
    fn xyz_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_com() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_com_guessed.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn z_gro_com() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "--com",
                "-z",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_z_com_guessed.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_to_pdb() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pdb",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xy_gro_to_pdb_com() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "--com",
                "-xy",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xy_com_guessed.pdb",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_pdb_to_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.pdb", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_pdb.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_pdb_to_pdb() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.pdb", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pdb",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_no_velocities() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input_no_velocities.gro", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_pdb.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_explicit() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-xyz"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_explicit_protein() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-xyz",
                "-rProtein",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xy_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-xy"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xy.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xz_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-xz"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-yz"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn x_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-x"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_x.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn y_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-y"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_y.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn z_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-z"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_z.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_water() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-rW",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_water.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_tpr_to_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.tpr", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_tpr.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_tpr_to_gro_molwith() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-rmolwith serial 3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_tpr.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xy_tpr_to_gro_whole_molecules() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "--whole",
                "-xy",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xy_whole_from_tpr.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn z_tpr_to_pdb_membrane() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ntests/test_files/index.ndx",
                "-rMembrane",
                "-z",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_z_membrane_from_tpr.pdb",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_pqr_to_pqr() {
        let output = Builder::new().suffix(".pqr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.pqr", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pqr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_to_pqr() {
        let output = Builder::new().suffix(".pqr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pqr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_pqr_to_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.pqr", &output_arg, "-yz"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz_from_pqr.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_tpr_to_pqr() {
        let output = Builder::new().suffix(".pqr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.tpr", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_tpr.pqr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_pdb_to_pqr() {
        let output = Builder::new().suffix(".pqr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.pdb", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pqr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_pqr_to_pdb() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.pqr", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pdb",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_pqr_struct() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.pqr",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_to_gro_traj() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_traj_xyz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_traj_to_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_traj.gro",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_traj_xyz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_traj_to_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_traj.gro",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_gro.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_traj_to_xtc_same_structure() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_traj.gro",
                &output_arg,
                "-ftests/test_files/input_traj.gro",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_gro.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_com() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_com_guessed.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_aa_nocom() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_nocom.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_explicit() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-xyz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_to_trr() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_xtc.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_to_trr() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.trr",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_trr.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_to_trr_com() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.trr",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_com_guessed.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_to_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.trr",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xy_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-xy",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xy.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xz_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-xz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-yz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn x_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-x",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_x.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn y_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-y",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_y.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn z_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-z",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_z.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_water() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-rW",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_water.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_xtc_com_begin_end_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
                "--com",
                "-yz",
                "-b10",
                "-e80",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz_com_guessed_begin_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_xtc_com_begin_end_step_pqr_struct() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.pqr",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
                "--com",
                "-yz",
                "-b10",
                "-e80",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz_com_guessed_begin_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn x_trr_com_begin_end_step() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.trr",
                "--com",
                "-x",
                "-b10",
                "-e80",
                "-t2",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_x_com_guessed_begin_end_step.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn complicated_group() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-r(resname ASN and serial 35 to 45 and name BB)",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_group.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn regex() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index.ndx",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-rr'^T.*_all$'",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_begin() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_end() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-e700",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_end.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_traj_step_to_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_traj.gro",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_step_from_gro.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_begin_end() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
                "-e800",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_begin_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_end_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-e800",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_begin_end_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
                "-e800",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs_begin() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-b400",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs_end() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-e700",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_end.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs_begin_end() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-b400",
                "-e800",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs_begin_end_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-t3",
                "-b400",
                "-e800",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_inputs_end_early() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-e100",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_end_early.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_begin_end() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.trr",
                "-b400",
                "-e800",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_begin_end_step() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.trr",
                "-b400",
                "-e800",
                "-t3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end_step.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_to_trr_multiple_inputs() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_xtc.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_to_trr_multiple_inputs() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.trr",
                "-ftests/test_files/input_part2.trr",
                "-ftests/test_files/input_part3.trr",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_from_trr.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_to_xtc_multiple_inputs() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.trr",
                "-ftests/test_files/input_part2.trr",
                "-ftests/test_files/input_part3.trr",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_trr_multiple_inputs_begin_end() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.trr",
                "-ftests/test_files/input_part2.trr",
                "-ftests/test_files/input_part3.trr",
                "-b400",
                "-e800",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_multiple_noncontinuous() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part3.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_swapped.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn backup() {
        let mut file = File::create("tests/test_files/temporary.gro").unwrap();
        file.write_all(b"Some content to test.").unwrap();

        let output = "tests/test_files/temporary.gro";
        let output_arg = format!("-o{}", output);

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg])
            .assert()
            .success();

        // check that the file has been successfully written
        assert!(file_diff::diff("tests/test_files/output_xyz.gro", output));

        // check that the backup has been created
        let backups: Vec<Result<std::path::PathBuf, glob::GlobError>> =
            glob::glob("tests/test_files/#temporary.gro*")
                .unwrap()
                .collect();
        assert_eq!(backups.len(), 1);

        let mut content = String::new();
        let mut read = File::open(backups[0].as_ref().unwrap()).unwrap();
        read.read_to_string(&mut content).unwrap();

        assert_eq!(content, "Some content to test.");

        fs::remove_file(backups[0].as_ref().unwrap()).unwrap();
        fs::remove_file("tests/test_files/temporary.gro").unwrap();
    }

    #[test]
    fn overwrite() {
        let mut file = File::create("tests/test_files/temporary_overwrite.gro").unwrap();
        file.write_all(b"Some content to test.").unwrap();

        let output = "tests/test_files/temporary_overwrite.gro";
        let output_arg = format!("-o{}", output);

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "--overwrite"])
            .assert()
            .success();

        // check that the file has been successfully written
        assert!(file_diff::diff("tests/test_files/output_xyz.gro", output));

        // check that the backup has NOT been created
        let backups: Vec<Result<std::path::PathBuf, glob::GlobError>> =
            glob::glob("tests/test_files/#temporary_overwrite.gro*")
                .unwrap()
                .collect();
        assert_eq!(backups.len(), 0);

        fs::remove_file("tests/test_files/temporary_overwrite.gro").unwrap();
    }

    #[test]
    fn silent() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "--silent",
            ])
            .assert()
            .success()
            .stdout("");

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn group_overwrite_default() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index_with_reference.ndx",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-r(resname ASN and serial 35 to 45 and name BB)",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_group.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn group_overwrite_during_autodetection() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index_with_reference.ndx",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-rProtein",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_element_query() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-relement name carbon nitrogen hydrogen oxygen",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_peptide.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_elname_query() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-relname carbon nitrogen hydrogen oxygen",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_peptide.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_elsymbol_query() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "-relsymbol C N H O",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_peptide.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_element_queries_individual() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "--xref=elname carbon nitrogen hydrogen oxygen",
                "--yref=elsymbol C N H O",
                "--zref=element symbol C N H O",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_peptide.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_aa_nocom() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.tpr",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_nocom.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_cg_nocom() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_aa_com() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.tpr",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_com.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_trr_tpr_cg_com() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.trr",
                "--com",
                "-yz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz_cg_com.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xy_xtc_tpr_aa_nocom_range_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.tpr",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
                "-b10",
                "-e80",
                "-t4",
                "-xy",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xy_aa_nocom_rangestep.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_cg_nocom_range_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
                "-e800",
                "-t3",
                "-xyz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn z_trr_tpr_aa_com_range_step() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.tpr",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.trr",
                "-b10",
                "-e80",
                "-t2",
                "-z",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_z_aa_com.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xz_xtc_tpr_cg_com_range_step() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.trr",
                "-b400",
                "-e800",
                "-t3",
                "-xz",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xz_cg_com_rangestep.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_cg_nocom_multi() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn yz_trr_tpr_cg_com_multi() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input_part1.trr",
                "-ftests/test_files/input_part2.trr",
                "-ftests/test_files/input_part3.trr",
                "--com",
                "-yz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yz_cg_com.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_cg_nocom_range_step_multi() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-b400",
                "-e800",
                "-t3",
                "-xyz",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_begin_end_step.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xz_xtc_tpr_cg_com_range_step_multi() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input_part1.trr",
                "-ftests/test_files/input_part2.trr",
                "-ftests/test_files/input_part3.trr",
                "-b400",
                "-e800",
                "-t3",
                "-xz",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xz_cg_com_rangestep.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_trp_molwith() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-rmolwith serial 3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_tpr_aa_element() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.tpr",
                &output_arg,
                "-ftests/test_files/input_aa_peptide.xtc",
                "-relement name carbon nitrogen hydrogen oxygen",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_aa_nocom.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_whole() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "--whole",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_whole.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn y_trr_whole() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.trr",
                "--whole",
                "-y",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_y_whole.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xy_xtc_whole_complex() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-ntests/test_files/index.ndx",
                "-rMembrane",
                "--whole",
                "-b400",
                "-e800",
                "-t3",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xy_whole_complex.trr",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_gro_nonorthogonal() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_nonorthogonal.gro",
                "-ftests/test_files/input.xtc",
                &output_arg,
            ])
            .assert()
            .success()
            .stderr("warning: input structure file has a non-orthogonal simulation box.\n\n");

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_gro_invalid() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_invalid_box.gro",
                "-ftests/test_files/input.xtc",
                &output_arg,
            ])
            .assert()
            .success()
            .stderr("warning: input structure file has an invalid simulation box (some dimensions are not positive).\n\n");

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_xtc_gro_undefined() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_no_box.pdb",
                "-ftests/test_files/input.xtc",
                &output_arg,
            ])
            .assert()
            .success()
            .stderr("warning: input structure file has an undefined simulation box.\n\n");

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyzref_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--xref=Protein",
                "--yref=@membrane",
                "--zref=@water",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyzref.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyzref_some_same_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--xref=Protein",
                "--yref=resid 1 to 21",
                "--zref=@water",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xzref_protein_y.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyzref_all_same_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index.ndx",
                "--xref=Protein",
                "--yref=resid 1 to 21",
                "--zref=@protein",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyzref_pdb() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.pdb",
                "--xref=Protein",
                "--yref=@membrane",
                "--zref=@water",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyzref.pdb",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyzref_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                "-ftests/test_files/input.xtc",
                "--xref=Protein",
                "--yref=@membrane",
                "--zref=@water",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyzref.xtc",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyzref_all_same_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index.ndx",
                "-ftests/test_files/input.xtc",
                "--xref=Protein",
                "--yref=resid 1 to 21",
                "--zref=@protein",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.xtc",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyzref_trr() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.pdb",
                "-ftests/test_files/input.trr",
                "--xref=Protein",
                "--yref=@membrane",
                "--zref=@water",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyzref.trr",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyref_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--xref=Protein",
                "--yref=@membrane",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyref.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xzref_protein_y_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--xref=Protein",
                "--zref=resname W",
                "-y",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xzref_protein_y.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn yref_protein_xz_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--yref=Membrane",
                "-ntests/test_files/index.ndx",
                "-xyz",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_yref_protein_xz.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xyref_same_water_z_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index.ndx",
                "--xref=Protein",
                "--yref=@protein",
                "-r@water",
                "-yz",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xzref_protein_y.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xref_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ntests/test_files/index.ndx",
                "--xref=serial <= 42",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_x.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xtc_whole_complex_with_multiple_references() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-ntests/test_files/index.ndx",
                "--xref=@water",
                "--yref=molecule with serial 3",
                "-z",
                "--reference=W",
                "--whole",
                "-b400",
                "-e800",
                "-t3",
                "--com",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/complex_with_multiple_references.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn tpr_whole_atomistic() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/large_aa.tpr",
                "-rresname SOL",
                "--whole",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/large_aa_whole_expected.gro",
            output.path().to_str().unwrap()
        ))
    }

    #[test]
    fn xtc_whole_atomistic() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/large_aa.tpr",
                "-ftests/test_files/large_aa.xtc",
                "-rresname POPC",
                "--whole",
                &output_arg,
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/large_aa_whole_expected.xtc",
            output.path().to_str().unwrap()
        ))
    }
}

#[cfg(test)]
mod fail_tests {
    use assert_cmd::Command;
    use tempfile::Builder;

    #[test]
    fn file_protection_gro() {
        std::fs::copy(
            "tests/test_files/input.gro",
            "tests/test_files/tmp_input.gro",
        )
        .unwrap();

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/tmp_input.gro",
                "-otests/test_files/tmp_input.gro",
            ])
            .assert()
            .failure();

        std::fs::remove_file("tests/test_files/tmp_input.gro").unwrap();
    }

    #[test]
    fn file_protection_xtc() {
        std::fs::copy(
            "tests/test_files/input.xtc",
            "tests/test_files/tmp_input.xtc",
        )
        .unwrap();

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ftests/test_files/tmp_input.xtc",
                "-otests/test_files/tmp_input.xtc",
            ])
            .assert()
            .failure();

        std::fs::remove_file("tests/test_files/tmp_input.xtc").unwrap();
    }

    #[test]
    fn file_protection_xtc_multiple() {
        std::fs::copy(
            "tests/test_files/input.xtc",
            "tests/test_files/tmp_input2.xtc",
        )
        .unwrap();

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-ftests/test_files/input.xtc",
                "-ftests/test_files/tmp_input2.xtc",
                "-otests/test_files/tmp_input2.xtc",
            ])
            .assert()
            .failure();

        std::fs::remove_file("tests/test_files/tmp_input2.xtc").unwrap();
    }

    #[test]
    fn nonexistent_group() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-rNonexistent",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn empty_group() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-r(resname LYS and name PO4)",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn gro_file_not_found() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_nonexistent.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn xtc_file_not_found() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_nonexistent.xtc",
                "-ntests/test_files/index.ndx",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn ndx_file_not_found() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-ntests/test_files/index_nonexistent.ndx",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn inconsistent_gro_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_tiny.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn nonorthogonal_box() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input_nonorthogonal.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn invalid_box() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input_invalid_box.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn undefined_box() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input_no_box.pdb", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn no_extension() {
        let output = Builder::new().tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn unsupported_extension_gro() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn unsupported_extension_xtc() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn begin_requires_traj() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-b400"])
            .assert()
            .failure();
    }

    #[test]
    fn end_requires_traj() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-e700"])
            .assert()
            .failure();
    }

    #[test]
    fn step_requires_traj() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-stests/test_files/input.gro", &output_arg, "-s3"])
            .assert()
            .failure();
    }

    #[test]
    fn multiple_inputs_identical() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part1.xtc",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn unknown_mass() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--com",
                "-r@membrane",
                &output_arg,
            ])
            .assert()
            .failure();
    }

    #[test]
    fn molwith_unsupported() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "-rmolecule with serial 17",
                &output_arg,
            ])
            .assert()
            .failure();
    }

    #[test]
    fn molwith_unsupported_xref() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--xref=molecule with serial 17",
                &output_arg,
            ])
            .assert()
            .failure();
    }

    #[test]
    fn molwith_unsupported_yref() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--yref=molecule with serial 17",
                &output_arg,
            ])
            .assert()
            .failure();
    }

    #[test]
    fn molwith_unsupported_zref() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                "--zref=molecule with serial 17",
                &output_arg,
            ])
            .assert()
            .failure();
    }

    #[test]
    fn whole_without_tpr() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input_aa_peptide.gro",
                &output_arg,
                "--whole",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn xtc_trr_mixed_inputs() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.trr",
                "-ftests/test_files/input_part3.xtc",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn xtc_trp_cg_element() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.tpr",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-relement name carbon",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn gro_traj_multiple_inputs() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/output_z.gro",
                "-ftests/test_files/output_yz.gro",
                "-ftests/test_files/output_xz.gro",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn gro_traj_begin() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.gro",
                "-b100",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn gro_traj_end() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-stests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.gro",
                "-e1000",
            ])
            .assert()
            .failure();
    }
}
