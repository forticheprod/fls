# f[rame]ls

## Description

**framels** project is intent to deliver a *ls* command inspired by *bls* from
[Buf Compagnie](https://buf.com/) or more recently *rvls* from [RV(https://www.autodesk.com/products/flow-production-tracking/rv)].

## User Guide

The user guide is [here](https://forticheprod.github.io/fls/).

## Installation

### Pre-compile bin

You can download the last version <https://github.com/forticheprod/fls/releases>.

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

### basic_listing

Run `fls` to list your current directory.

![demo](./images/fls_demo.gif)

```bash
$ touch aaa.001.tif aaa.002.tif aaa.003.tif aaa.004.tif aaa.005.tif foo_bar.exr
$ fls
foo_bar.exr
aaa.***.tif@1-5
```

Run `fls /path/of/directory/` to list a specific directory.

Exemple with the sample set **big** at the root level of the repo

```bash
$ fls samples/big/
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

### Exr Metadata

Inspired by rvls. You can use `-l`

Run `fls -l` to list your current directory and display EXR informations.

Run `fls -l /path/of/directory/` to list a specific directory and display EXR informations.

```bash
$ fls -l samples/big/
RenderPass_Beauty_1_*****.exr@0-96
RenderPass_Occlusion_1_*****.exr@0-73,75-96
RenderPass_Id_1_*****.exr@0-96
RenderPass_Occlusion_1_***.exr@74
RenderPass_Ncam_1_00042.exr.bkp
RenderPass_SpecularRim_1_*****.exr@0-96
RenderPass_DiffuseKey_1_*****.exr@0-96
RenderPass_Pcam_1_*****.exr@0-96
RenderPass_Reflection_1_*****.exr@0-96
RenderPass_Specular_1_*****.exr@0-96
RenderPass_Diffuse_1_*****.exr@0-96
RenderPass_IndDiffuse_1_*****.exr@0-96
RenderPass_Ncam_1_*****.exr@0-41,43-96
samples/big/RenderPass_Occlusion_1_074.exr layer #0 w x h: 320 x 156; channels: A,B,G,R, bit depth: F16
Not an exr
samples/big/RenderPass_Beauty_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,Plane_Beauty.A,Plane_Beauty.B,Plane_Beauty.G,Plane_Beauty.R,R,Spheres_Beauty.A,Spheres_Beauty.B,Spheres_Beauty.G,Spheres_Beauty.R, bit depth: F16
samples/big/RenderPass_DiffuseKey_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_Reflection_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_Pcam_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F32
samples/big/RenderPass_Occlusion_1_00000.exr layer #0 w x h: 320 x 139; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_Id_1_00000.exr layer #0 w x h: 140 x 52; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_Diffuse_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_Specular_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_Ncam_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_IndDiffuse_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
samples/big/RenderPass_SpecularRim_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,R, bit depth: F16
```

### Recursive

You can use a recursive approch of the directory and sub-folder

You can use `-r`

Run `fls -r /path/of/directory/` to list a specific directory and his
subfolder

```bash
$ fls -r .\samples\
.\samples\big\RenderPass_Diffuse_1_*****.exr@0-96
.\samples\big\RenderPass_Ncam_1_00042.exr.bkp
.\samples\big\RenderPass_DiffuseKey_1_*****.exr@0-96
.\samples\big\RenderPass_Occlusion_1_*****.exr@0-73,75-96
.\samples\big\RenderPass_Occlusion_1_***.exr@74
.\samples\small\foo_bar.exr
.\samples\big\RenderPass_SpecularRim_1_*****.exr@0-96
.\samples\big\RenderPass_Pcam_1_*****.exr@0-96
.\samples\big\RenderPass_Ncam_1_*****.exr@0-41,43-96
.\samples\small
.\samples\big
.\samples\big\RenderPass_Id_1_*****.exr@0-96
.\samples\big\RenderPass_Specular_1_*****.exr@0-96
.\samples\big\RenderPass_Beauty_1_*****.exr@0-96
.\samples
.\samples\big\RenderPass_IndDiffuse_1_*****.exr@0-96
.\samples\big\RenderPass_Reflection_1_*****.exr@0-96
.\samples\small\aaa.***.tif@1-5
```

### Tree

You can use a tree approch of the directory and sub-folder

You can use `-t`

Always use `-r` to use the tree mode.

Run `fls -t -r /path/of/directory/` to list a specific directory and his
subfolder

```bash
$ fls -t -r .\samples\
┗ 
    ┗ samples
        ┗ big
            ┗ RenderPass_Beauty_1_*****.exr@0-96
            ┗ RenderPass_DiffuseKey_1_*****.exr@0-96
            ┗ RenderPass_Diffuse_1_*****.exr@0-96
            ┗ RenderPass_Id_1_*****.exr@0-96
            ┗ RenderPass_IndDiffuse_1_*****.exr@0-96
            ┗ RenderPass_Ncam_1_*****.exr@0-41,43-96
            ┗ RenderPass_Ncam_1_00042.exr.bkp
            ┗ RenderPass_Occlusion_1_*****.exr@0-73,75-96
            ┗ RenderPass_Occlusion_1_***.exr@74
            ┗ RenderPass_Pcam_1_*****.exr@0-96
            ┗ RenderPass_Reflection_1_*****.exr@0-96
            ┗ RenderPass_SpecularRim_1_*****.exr@0-96
            ┗ RenderPass_Specular_1_*****.exr@0-96
        ┗ mega
            ┗ response_1689510067951.json
        ┗ small
            ┗ aaa.***.tif@1-5
            ┗ foo_bar.exr
```

## Benchmarks

![fast](https://camo.githubusercontent.com/e8a50ee9600d66095bf0046f06e65ef8fe0675a40122db2a801d1f66e595add6/68747470733a2f2f692e726564642e69742f74376e733971746235676838312e6a7067)

Using the sample **big**, some time comparaison with [rvls](https://www.shotgridsoftware.com/rv/download/), [lsseq](https://github.com/jrowellfx/lsseq) or [lss](https://github.com/rsgalloway/pyseq).

Here benchmarks done with [hyperfine](https://github.com/sharkdp/hyperfine) with a warmup of 3 iterations

### Simple file listing

| Tool |`fls 0.7.7`| `rvls 2023.0.1`|`lsseq -l`| `lss`    |
|------|-----------|----------------|----------|----------|
| Time | 3.9 ms    | 85.9 ms        | 75.5 ms  | 61.9 ms  |

```bash
hyperfine -N --warmup 3 'target/release/fls ./samples/big/'
Benchmark 1: target/release/fls ./samples/big/
  Time (mean ± σ):       4.7 ms ±   0.7 ms    [User: 4.9 ms, System: 1.7 ms]
  Range (min … max):     4.3 ms …  11.5 ms    560 runs
```

```bash
hyperfine -N --warmup 3 '~/Downloads/rv-centos7-x86-64-2023.0.1/bin/rvls ./samples/big/'
Benchmark 1: ~/Downloads/rv-centos7-x86-64-2023.0.1/bin/rvls ./samples/big/
  Time (mean ± σ):      23.3 ms ±   7.1 ms    [User: 20.1 ms, System: 2.3 ms]
  Range (min … max):    19.4 ms …  51.5 ms    75 runs
```

### Exr reading

| Tool | `fls -l` | `rvls -l`|
|------|----------|----------|
| Time | 7.3 ms   | 95.8 ms  |

## Issues and PR

Issues, feedback and PR are welcome

Some hard rules, i won't trade off

- Windows compatible
- Speed
- UTF8 friendly
- Pure Rust

## Thanks

Thanks to:

- Ben Legros for the indirect mentorship over a beer ( or many )
- Djl for the idea
- Tcherno
- Mercenaries Eng for the best render engine ( samples/big render with [Guerilla render](http://guerillarender.com/) )
