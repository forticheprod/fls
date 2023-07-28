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
- Windows 10 x86_64+
- current linux distro tested: Fedora 37, Centos 7, Manjaro 22
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

### Basic Listing

Run `fls` to list your current directory.

![](./docs/content/docs/getting-started/images/fls_demo.gif)