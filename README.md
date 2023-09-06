# gcenter: Center Any Group in a Gromacs Trajectory 

Center your chosen group within a Gromacs trajectory or structure file effortlessly using the [Bai & Breen algorithm](https://doi.org/10.1080/2151237X.2008.10129266).

`gcenter` can accurately center atom groups, even when they span multiple molecules that may extend beyond the box boundaries. Note that `gcenter` does not employ connectivity information, so it doesn't require a tpr file as input. However, it also lacks the capability to wrap molecules into the simulation box. Be aware that `gcenter` exclusively supports orthogonal simulation boxes.

`gcenter` supports gro and pdb structure files and xtc and trr trajectories and it can autodetect protein residues. Use VMD-like [groan selection language](https://docs.rs/groan_rs/0.1.0/groan_rs/#groan-selection-language) to select groups of atoms to center.

## Installation

0) Install [rust](https://www.rust-lang.org/tools/install).
1) Run `cargo install gcenter`.

## Example usage

```
gcenter -c system.gro -f trajectory.xtc -o output_trajectory.xtc
```

Use `gcenter --help` for more information about using this program.


## Limitations

Only tested on Linux but should work anywhere.

Only supports orthogonal simulation boxes!