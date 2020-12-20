use super::image::Image;
use std::collections::HashMap;

pub fn parse_inputs(v: Vec<String>) -> Result<HashMap<usize, Image>, ()> {
    let mut index = 0;
    let mut prev_index = index;
    let mut images = HashMap::new();
    let v_len = v.len();
    loop {
        // New line
        if v[index].trim().len() == 0 || index == (v_len - 1) {
            if index == (v_len - 1) {
                index += 1;
            }
            let c_v = v[prev_index..index].to_vec();
            match Image::from_raw_image(&c_v) {
                Ok(image) => {
                    images.insert(image.id, image);
                    index += 1;
                    prev_index = index;
                }
                Err(_) => {
                    println!("Cannot continue to parse");
                    return Err(());
                }
            }
        } else {
            index += 1;
        }
        // End loop
        if index >= v_len {
            break;
        }
    }
    Ok(images)
}
