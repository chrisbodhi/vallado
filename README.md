# Vallado
## Notes on Vallado's "Principles of Astrodynamics and Applications (4th Edition)" in Rust

>Astrodynamics is the study of the motion of man-made objects in space, subject to both natural and artificially induced forces.

&mdash; Griffin and French, _Space Vehicle Design_, 1991

### Architecture

#### Almagest

A library for performing astrodynamics calculations. Written in Rust, it provides a set of tools for solving problems in astrodynamics. `no_std` by default for maximum portability.

#### Notebooks

After getting the environment set up and running Bash via `nix develop`, you can run Jupyter Lab by running the following command:

```sh
jupyter lab
```

and visit [localhost:8888/lab](http://localhost:8888/lab)

### Development

#### macOS

Once you have Nix installed, run the following command to get the environment set up:

```sh
nix develop
```
