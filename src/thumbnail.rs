// External modules:
use walkdir::WalkDir;
use error::{Result, ResultExt};
use image;
use image::imageops::resize;
use image::{GenericImage, FilterType};

// Std modules:
use std::path::Path;

// Internal modules:
use config::PanolutionConfig;
use util::is_supported_format;

fn split_path(path: &str) -> (&str, &str) {
    let path = Path::new(path);

    (path.parent().unwrap().to_str().unwrap(),
     path.file_name().unwrap().to_str().unwrap())
}

pub fn create_thumbnails(config: &PanolutionConfig) -> Result<(Vec<String>, Vec<Vec<String>>)> {
    let mut all_image_paths = Vec::new();
    let mut all_thumbnail_paths = Vec::new();

    for entry in WalkDir::new(&config.input_path) {
        let entry = entry.chain_err(|| "error in WalkDir")?;
        if entry.file_type().is_file() {
            let entry = entry.file_name().to_str().unwrap();
            if is_supported_format(entry) {
                if entry.starts_with("thumb_") {
                    info!("Ignore thumbnail file: {}", entry);
                } else {
                    let full_path = format!("{}/{}", config.input_path, entry); // TODO: windows path separator
                    all_image_paths.push(full_path);
                }
            } else {
                info!("Image format not supported yet: {}", entry);
            }
        }
    }

    for scale_factor in &config.scale_factors {
        let mut thumb_paths = Vec::new();
        for full_path in &all_image_paths {
            let (base_path, file_name) = split_path(&full_path);
            let thumb_path = format!("{}/thumb_{}_{}", base_path, scale_factor, file_name);
            if Path::new(&thumb_path).exists() {
                info!("Thumbnail was already generated, using old one: '{}'", &thumb_path)
            } else {
                info!("Loading image file: '{}'", full_path);
                let orig_img = image::open(&full_path).chain_err(|| format!("can't open image: '{}", full_path))?;
                info!("No thumbnail found for scale factor '{}', generating new one", scale_factor);
                let (orig_w, orig_h) = orig_img.dimensions();
                let (thumb_w, thumb_h) = (((orig_w as f64) * scale_factor) as u32, ((orig_h as f64) * scale_factor) as u32);
                info!("image sizes: ({}, {}) -> ({}, {})", orig_w, orig_h, thumb_w, thumb_h);
                let thumb_img = resize(&orig_img, thumb_w, thumb_h, FilterType::Nearest);
                let _ = thumb_img.save(&thumb_path).chain_err(|| "can't save thumbnail image")?;
            }
            thumb_paths.push(thumb_path);
        }
        all_thumbnail_paths.push(thumb_paths);
    }

    Ok((all_image_paths, all_thumbnail_paths))
}
