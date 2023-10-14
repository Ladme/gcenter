// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

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
            .args(["-ctests/test_files/input.gro", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.gro",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_gro_to_pdb() {
        let output = Builder::new().suffix(".pdb").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input.gro", &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz.pdb",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn xyz_pdb_to_gro() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input.pdb", &output_arg])
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
            .args(["-ctests/test_files/input.pdb", &output_arg])
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
            .args(["-ctests/test_files/input_no_velocities.gro", &output_arg])
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-xyz"])
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
                "-ctests/test_files/input.gro",
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-xy"])
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-xz"])
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-yz"])
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-x"])
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-y"])
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-z"])
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
                "-ctests/test_files/input.gro",
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
    fn xyz_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
    fn xyz_xtc_explicit() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
    fn xyz_trr_to_xtc() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
    fn complicated_group() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-s3",
            ])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/output_xyz_step.xtc",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
                "-s3",
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
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-e800",
                "-s3",
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
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.xtc",
                "-b400",
                "-e800",
                "-s3",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
    fn xyz_trr_begin_end() {
        let output = Builder::new().suffix(".trr").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input.trr",
                "-b400",
                "-e800",
                "-s3",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
    fn xyz_xtc_and_trr_multiple_inputs() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.trr",
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
    fn xyz_xtc_multiple_noncontinuous() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
            .args(["-ctests/test_files/input.gro", &output_arg])
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
        let mut read = File::open(&backups[0].as_ref().unwrap()).unwrap();
        read.read_to_string(&mut content).unwrap();

        assert_eq!(content, "Some content to test.");

        fs::remove_file(&backups[0].as_ref().unwrap()).unwrap();
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
            .args(["-ctests/test_files/input.gro", &output_arg, "--overwrite"])
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/tmp_input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input_nonexistent.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input.gro",
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
                "-ctests/test_files/input_tiny.gro",
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
            .args([
                "-ctests/test_files/input_tiny_nonorthogonal.gro",
                &output_arg,
            ])
            .assert()
            .failure();
    }

    #[test]
    fn invalid_box() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input_tiny_invalid.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn no_extension() {
        let output = Builder::new().tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn unsupported_extension_gro() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input.gro", &output_arg])
            .assert()
            .failure();
    }

    #[test]
    fn unsupported_extension_xtc() {
        let output = Builder::new().suffix(".gro").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
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
            .args(["-ctests/test_files/input.gro", &output_arg, "-b400"])
            .assert()
            .failure();
    }

    #[test]
    fn end_requires_traj() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input.gro", &output_arg, "-e700"])
            .assert()
            .failure();
    }

    #[test]
    fn step_requires_traj() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args(["-ctests/test_files/input.gro", &output_arg, "-s3"])
            .assert()
            .failure();
    }

    #[test]
    fn multiple_input_step() {
        let output = Builder::new().suffix(".xtc").tempfile().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("gcenter")
            .unwrap()
            .args([
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part2.xtc",
                "-ftests/test_files/input_part3.xtc",
                "-s3",
            ])
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
                "-ctests/test_files/input.gro",
                &output_arg,
                "-ftests/test_files/input_part1.xtc",
                "-ftests/test_files/input_part1.xtc",
            ])
            .assert()
            .failure();
    }
}
