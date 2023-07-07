use exr::prelude::*;

/// Print the metadatas of a an EXR using a path as an input
pub fn read_meta(path: String) {
    let msg = format!("run path `{}` to generate the required file", &path);
    let meta_data = MetaData::read_from_file(
        &path,
        false, // do not throw an error for invalid or missing attributes, skipping them instead
    )
    .expect(&msg);
    // Iterate over the headers
    for (layer_index, image_layer) in meta_data.headers.iter().enumerate() {
        println!(
            "{} layer #{} size:{:?}; channels:{:?}",
            &path, layer_index, image_layer.layer_size, image_layer.channels
        );
    }
}
