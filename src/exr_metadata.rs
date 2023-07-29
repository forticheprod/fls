use exr::prelude::*;

/// Print the metadatas of a an EXR using a path as an input, the output is
/// trying to the closer as `rvls -l` without the formating
pub fn read_meta(path: String) -> String {
    let msg = format!("run path `{}` to generate the required file", &path);
    let meta_data = MetaData::read_from_file(
        &path,
        false, // do not throw an error for invalid or missing attributes, skipping them instead
    )
    .expect(&msg);
    // Iterate over the headers
    let mut metadata: Vec<String> = Vec::new();
    for (layer_index, image_layer) in meta_data.headers.iter().enumerate() {
        metadata.push(format!(
            "{} layer #{} size:{:?}",
            &path, layer_index, image_layer.layer_size
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
