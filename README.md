# gcenter: Center Any Group in a Gromacs Trajectory 

Center your chosen group within a Gromacs trajectory or structure file effortlessly using the [Bai & Breen algorithm](https://doi.org/10.1080/2151237X.2008.10129266).

`gcenter` can accurately center atom groups, even when they span multiple molecules that may extend beyond the box boundaries. `gcenter` exclusively supports orthogonal simulation boxes.

`gcenter` supports gro, pdb, and tpr structure files and xtc and trr trajectories and it can autodetect protein residues. Use VMD-like [groan selection language](https://docs.rs/groan_rs/latest/groan_rs/#groan-selection-language) to select groups of atoms to center.

## Installation

0) Install [rust](https://www.rust-lang.org/tools/install).
1) Run `cargo install gcenter`.

## Example usage

```
gcenter -c system.gro -f trajectory.xtc -o output_trajectory.xtc
```

## Options

```text
Usage: gcenter [OPTIONS] --structure <STRUCTURE> --output <OUTPUT>

Options:
  -s, --structure <STRUCTURE>
          Path to a gro, pdb, pqr, or tpr file containing the system structure. If a trajectory is also provided, the coordinates from the structure file are ignored.

  -f, --trajectory [<TRAJECTORIES>...]
          Path to xtc or trr file(s) or to a single gro file containing the trajectory (or trajectories) to be manipulated. 
          If not provided, the centering operation will use the structure file itself.
          Multiple files separated by whitespace can be provided. These will be concatenated into one output file.
          All trajectory files must be of the same type (i.e., all must be either xtc or trr files).
          When joining trajectories, the last frame of each trajectory and the first frame of the following trajectory are checked for matching simulation steps. 
          If the simulation steps coincide, only the first of these frames is centered and written to output.

  -n, --index <INDEX>
          Path to an ndx file containing groups associated with the system.
          
          [default: index.ndx]

  -o, --output <OUTPUT>
          Name of the output file, which can be in gro, pdb, or pqr format if no trajectory is provided, 
          or in xtc, trr, or gro format if a trajectory is provided.

  -r, --reference <REFERENCE>
          Specify the group to be centered. Define the group using the VMD-like 'groan selection language', which also supports ndx group names.
          
          [default: Protein]

  -b, --begin <START_TIME>
          Time of the first frame to read from the trajectory (in ps). All previous frames will be skipped.
          This option is only applicable when trajectory file(s) is/are provided.
          This option cannot be used when the trajectory is a gro file since gro files are not guaranteed to contain simulation time information.
          
          [default: 0.0]

  -e, --end <END_TIME>
          Time of the last frame to read from the trajectory (in ps). All following frames will be skipped.
          This option is only applicable when trajectory file(s) is/are provided.
          This option cannot be used when the trajectory is a gro file since gro files are not guaranteed to contain simulation time information.
          
          [default: NaN]

  -t, --step <STEP>
          Center and write only every <STEP>th frame of the trajectory to the output file.
          This option is only applicable when trajectory file(s) is/are provided.
          
          [default: 1]

  -x
          Perform centering operation in the x-dimension. This can be combined with other dimensions. If no dimensions are selected, it defaults to '-xyz'.

  -y
          Perform centering operation in the y-dimension. This can be combined with other dimensions. If no dimensions are selected, it defaults to '-xyz'.

  -z
          Perform centering operation in the z-dimension. This can be combined with other dimensions. If no dimensions are selected, it defaults to '-xyz'.

      --xref <XREFERENCE>
          Center the specified selection of atoms along the x dimension. 
          This option, in conjunction with `yref` and `zref`, allows you to center multiple groups, each along a different dimension. 
          Define the group using the VMD-like 'groan selection language', which also supports ndx group names. 
          This selection acts as the reference selection for the x dimension, while the `reference` selection will still be centered in other specified dimensions.

      --yref <YREFERENCE>
          Center the specified selection of atoms along the y dimension. 
          This option, in conjunction with `xref` and `zref`, allows you to center multiple groups, each along a different dimension. 
          Define the group using the VMD-like 'groan selection language', which also supports ndx group names. 
          This selection acts as the reference selection for the y dimension, while the `reference` selection will still be centered in other specified dimensions.

      --zref <ZREFERENCE>
          Center the specified selection of atoms along the z dimension. 
          This option, in conjunction with `xref` and `yref`, allows you to center multiple groups, each along a different dimension. 
          Define the group using the VMD-like 'groan selection language', which also supports ndx group names. 
          This selection acts as the reference selection for the z dimension, while the `reference` selection will still be centered in other specified dimensions.

      --com
          Use center of mass instead of center of geometry when centering the reference group. This requires information about atom masses. 
          If they are not explicitly provided using a tpr file, the masses are guessed.

      --whole
          Do not wrap all atoms into the simulation box but keep molecules whole. This requires providing a tpr file as an input structure file.

      --silent
          Suppress all standard output generated by the 'gcenter' tool, except for error messages written to stderr.

      --overwrite
          Enable this option to overwrite existing files with the same name as the output file. No backup copies will be created.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Limitations

Only tested on Linux but should work on any modern OS.

Only supports orthogonal simulation boxes!