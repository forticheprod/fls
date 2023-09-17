+++
title = "Library"
description = "A usage of library"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'A usage of library'
toc = true
top = false
+++

## Basic listing of the library

The library is documented with rustdoc and you can find the documentation [here](https://docs.rs/framels/latest/framels/).

But here is a basic listing of the library

### Description

The function **basic_listing** is the main function of the library it use a list of
filename as in input and pack the frame sequences using a new filename
like `toto.***.jpg@158-179`
It take a `Vec<String>` of entries as an input

- Pack the frames
- Return a Vector of path packed

### Example

```rust
use framels::{basic_listing, parse_dir, paths::{Paths, PathsPacked}};

let source = "./samples/big".to_string();
let paths: Paths = parse_dir(&source);
let results: PathsPacked = basic_listing(paths);
println!("{}", results);
``
