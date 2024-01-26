use exr::meta::header::Header;
use exr::prelude::*;

/// # read_meta
///
/// ## Todo
///
/// This function need to be refactored to generate a cleaner output.
/// https://github.com/forticheprod/fls/issues/236
///
/// ## Description
///
/// Read the metadata of an exr file
///
/// ## Example
/// The module is private no need to document it.
pub fn read_meta(path: String) -> String {
    let msg = format!("run path `{}` to generate the required file", &path);
    let meta_data = MetaData::read_from_file(
        &path,
        false, // do not throw an error for invalid or missing attributes, skipping them instead
    )
    .expect(&msg);
    // Iterate over the headers
    let mut metadata: Vec<String> = Vec::new();
    for (layer_index, image_headers) in meta_data.headers.iter().enumerate() {
        metadata.push(format!(
            "{} {}",
            &path,
            trim_medata(layer_index, image_headers)
        ));
    }
    // Join the headers
    metadata.join(",")
}
#[test]
fn test_read_meta() {
    let source = "./samples/big/RenderPass_Beauty_1_00000.exr".to_string();
    let expect = "./samples/big/RenderPass_Beauty_1_00000.exr layer #0 size:Vec2(320, 143); channels:ChannelList { list: [ChannelDescription { name: exr::Text(\"A\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"B\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"G\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Plane_Beauty.A\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Plane_Beauty.B\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Plane_Beauty.G\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Plane_Beauty.R\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"R\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Spheres_Beauty.A\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Spheres_Beauty.B\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Spheres_Beauty.G\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }, ChannelDescription { name: exr::Text(\"Spheres_Beauty.R\"), sample_type: F16, quantize_linearly: false, sampling: Vec2(1, 1) }], bytes_per_pixel: 24, uniform_sample_type: Some(F16) }".to_string();
    assert_eq!(expect, read_meta(source));
}

fn trim_medata(layer_index: usize, image_headers: &Header) -> String {
    format!(
        "layer #{} size:{:?}; channels:{:?}",
        layer_index, image_headers.layer_size, image_headers.channels
    )
}
