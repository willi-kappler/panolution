// External modules:
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use error::{Result, ResultExt};
use image;
use image::imageops::resize;
use image::{GenericImage, FilterType};

// Internal modules:
use config::PanolutionConfig;
use optimizer::{Solution, ImageArrangement};

// Std modules:
use std::path::PathBuf;

fn is_supported_format(entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
        if let Some(file_name) = entry.file_name().to_str() {
            let extension = file_name.split(".").last().unwrap_or("").to_lowercase();

            let supported: Vec<String> = vec!["jpg", "jpeg", "gif", "png", "tif", "tiff"].iter().map(|s| s.to_string()).collect();

            supported.contains(&extension)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn create_thumbnails(config: &PanolutionConfig) -> Result<Vec<Solution>> {
    let mut all_image_paths: Vec<PathBuf> = Vec::new();
    let mut result = Vec::new();
    let walker = WalkDir::new(&config.input_path).into_iter();

    for entry in walker.filter_entry(|e| is_supported_format(e)) {
        let entry = entry.chain_err(|| "error in WalkDir")?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if file_name.starts_with("thumb_") {
                    info!("Ignore thumbnail file: {:?}", path);
                } else {
                    all_image_paths.push(path.to_path_buf());
                }
            } else {
                info!("Could not converto to str: {:?}", file_name);
            }
        } else {
            info!("Could not get file_name: {:?}", path)
        }
    }

    for scale_factor in &config.scale_factors {
        let mut arrangement = Vec::new();

        for path in &all_image_paths {
            if *scale_factor >= 1.0 {
                info!("Add original image");

                arrangement.push(
                    ImageArrangement{
                        file_name: path.to_str().unwrap().to_string(),
                        x: 0.5,
                        y: 0.5,
                        angle: 0.0
                    }
                );
            } else {
                let base_path = path.parent().unwrap();
                let file_name = path.file_name().unwrap(); // Save to unwrap since we checked above
                let thumb_path = base_path.join(format!("thumb_{}_{}", scale_factor, file_name.to_str().unwrap()));

                if thumb_path.exists() {
                    info!("Thumbnail was already generated, using old one: '{:?}'", &thumb_path)
                } else {
                    info!("Loading image file: '{:?}'", path);
                    let orig_img = image::open(&path).chain_err(|| format!("can't open image: '{:?}", path))?;
                    let width = orig_img.width();
                    let height = orig_img.height();
                    info!("No thumbnail found for scale factor '{}', generating new one", scale_factor);
                    let (orig_w, orig_h) = orig_img.dimensions();
                    let (thumb_w, thumb_h) = (((orig_w as f64) * scale_factor) as u32, ((orig_h as f64) * scale_factor) as u32);
                    info!("image sizes: ({}, {}) -> ({}, {})", orig_w, orig_h, thumb_w, thumb_h);
                    let thumb_img = resize(&orig_img, thumb_w, thumb_h, FilterType::Nearest);
                    let _ = thumb_img.save(&thumb_path).chain_err(|| "can't save thumbnail image")?;
                }

                arrangement.push(
                    ImageArrangement{
                        file_name: thumb_path.to_str().unwrap().to_string(),
                        x: 0.5,
                        y: 0.5,
                        angle: 0.0
                    }
                );
            }
        }

        result.push(
            Solution {
                arrangement: arrangement
            }
        );
    }

    Ok(result)
}
