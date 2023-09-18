+++
title = "Quick Start"
description = "How to start using fls?"
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "How to start using fls?"
toc = true
top = false
+++

## Requirements

### Configuration

a standard machine with an OS listed below:

- Windows 10+ x86_64
- current linux distro tested: Fedora 37, Centos 7, Manjaro 22, rocky 9.2
- MacOs 10.7+

### Terminal

Before using `fls` you need to have the adminstrator right and terminal application
like **powershell** or **gnome-terminal**

## Installation

### Pre-compile bin

You can download the last version <https://github.com/doubleailes/fls/releases>

#### Windows

- Unzip the folder
- Copy the `fls.exe` file into `C:\program files\Fls`
- Add the path `C:\program files\Fls` to the `PATH` environement variable

#### Linux

- Untar the folder
- Copy the `fls` binary into `/usr/bin`

### From Cargo

You can install via [<https://crates.io/>](https://crates.io/crates/framels)

- Run `cargo install framels`
- Check `~/.cargo/bin/` is added to path if not add it

## Usage

### Help

Run `fls --help`

### Basic

#### Current directory

Run `fls` to list your current directory.

```bash
$ touch aaa.001.tif aaa.002.tif aaa.003.tif aaa.004.tif aaa.005.tif foo_bar.exr
$ fls
foo_bar.exr
aaa.***.tif@1-5
```

#### Specific directory

Run `fls /path/of/directory/` to list a specific directory.

Exemple with the sample set **big** at the root level of the repo

```bash
$ fls ./samples/big/
RenderPass_Pcam_1_*****.exr@0-96
RenderPass_Beauty_1_*****.exr@0-96
RenderPass_IndDiffuse_1_*****.exr@0-96
RenderPass_Ncam_1_*****.exr@0-41,43-96
RenderPass_Specular_1_*****.exr@0-96
RenderPass_Id_1_*****.exr@0-96
RenderPass_Occlusion_1_***.exr@74
RenderPass_Reflection_1_*****.exr@0-96
RenderPass_SpecularRim_1_*****.exr@0-96
RenderPass_Ncam_1_00042.exr.bkp
RenderPass_DiffuseKey_1_*****.exr@0-96
RenderPass_Diffuse_1_*****.exr@0-96
RenderPass_Occlusion_1_*****.exr@0-73,75-96
```
