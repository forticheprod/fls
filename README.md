# f[rame]ls

## Description

**framels** project is intent to deliver a *ls* command inspired by *bls* from Buf Compagnie or more recently *rvls* from RV

## Installation

### Pre-compile bin

You can download the last version <https://github.com/doubleailes/fls/releases>

#### Windows

Support Windows 10 x86

#### Linux

Support current linux distro tested Fedora 37, Centos 7, Manjaro 22

### From Cargo

You can install via [<https://crates.io/>](https://crates.io/crates/framels)

Run `cargo install framels`

## Usage

### Help

Run `fls --help`

### Basic

Run `fls` to list your current directory.

Run `fls -- /path/of/directory/` to list a specific directory.

### Exr Metadata

Inspired by rvls. You can use `-l`

Run `fls -l` to list your current directory and display EXR informations.

Run `fls -l -- /path/of/directory/` to list a specific directory and display EXR informations.

## Thanks

Thanks to:
- Ben Legros for the passive mentorship over a beer ( or many )
- Djl for the idea
- Tcherno
- Mercenaries Eng for the best render engine ( samples/big from [Guerilla render](http://guerillarender.com/) )
