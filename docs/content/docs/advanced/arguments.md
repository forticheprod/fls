+++
title = "Arguments"
description = "A usage of arguments"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'A usage of arguments'
toc = true
top = false
+++

### list

Run `fls -l` to list your current directory and display EXR informations.

Run `fls -l -- /path/of/directory/` to list a specific directory and display EXR informations.

```bash
$ fls -l -- samples/big/
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
.\samples\big\RenderPass_Beauty_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Plane_Beauty.A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Plane_Beauty.B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Plane_Beauty.G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Plane_Beauty.R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Spheres_Beauty.A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Spheres_Beauty.B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Spheres_Beauty.G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("Spheres_Beauty.R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 24, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Occlusion_1_00000.exr layer #0 size:Vec2(320, 139); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Id_1_00000.exr layer #0 size:Vec2(140, 52); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Occlusion_1_074.exr layer #0 size:Vec2(320, 156); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
Not an exr
.\samples\big\RenderPass_SpecularRim_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_DiffuseKey_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Pcam_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F32, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F32, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F32, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F32, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 16, uniform_sample_type: Some(F32) }
.\samples\big\RenderPass_Reflection_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Specular_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Diffuse_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_IndDiffuse_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
.\samples\big\RenderPass_Ncam_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text("A"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("B"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("G"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text("R"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 8, uniform_sample_type: Some(F16) }
```

### Recursive

You can use a recursive approch of the directory and sub-folder

You can use `-r`

Run `fls -r -- /path/of/directory/` to list a specific directory and his
subfolder

```bash
$ fls -r -- .\samples\
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

Watch out, the recursive mode is not compatible with the `-l, --list` option, at the moment.
