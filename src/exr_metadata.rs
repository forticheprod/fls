use exr::meta::{attribute::ChannelList, header::Header};
use exr::prelude::*;
use std::str;

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
    let expect = "./samples/big/RenderPass_Beauty_1_00000.exr layer #0 w x h: 320 x 143; channels: A,B,G,Plane_Beauty.A,Plane_Beauty.B,Plane_Beauty.G,Plane_Beauty.R,R,Spheres_Beauty.A,Spheres_Beauty.B,Spheres_Beauty.G,Spheres_Beauty.R, bit depth: F16".to_string();
    assert_eq!(expect, read_meta(source));
}

fn get_channels_name(channels: ChannelList) -> String {
    let mut channels_name: Vec<String> = Vec::new();
    for channels_description in channels.list {
        channels_name.push(
            str::from_utf8(channels_description.name.as_slice())
                .unwrap()
                .to_string(),
        );
    }
    //channels_name.sort_by(|a, b| a.len().cmp(&b.len()));
    channels_name.join(",")
}

fn get_type(channels: ChannelList) -> String {
    let mut channels_type: Vec<String> = Vec::new();
    for channels_description in channels.list {
        channels_type.push(format!("{:?}", channels_description.sample_type));
    }
    channels_type.dedup();
    channels_type.join(",")
}

fn trim_medata(layer_index: usize, image_headers: &Header) -> String {
    let w = image_headers.layer_size.0;
    let h = image_headers.layer_size.1;

    format!(
        "layer #{} w x h: {} x {}; channels: {}, bit depth: {}",
        layer_index,
        w,
        h,
        get_channels_name(image_headers.channels.clone()),
        get_type(image_headers.channels.clone())
    )
}
