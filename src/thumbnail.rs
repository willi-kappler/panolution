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

fn valid_image_file(file_name: &str) -> bool {
    let extension = file_name.split(".").last().unwrap_or("").to_lowercase();

    let supported: Vec<String> = vec!["jpg", "jpeg", "gif", "png", "tif", "tiff"].iter().map(|s| s.to_string()).collect();

    supported.contains(&extension)
}

fn create_image_arrangement(path: &PathBuf) -> Result<ImageArrangement> {
    info!("Loading image file: '{:?}'", path);
    let img = image::open(&path).chain_err(|| format!("can't open image: '{:?}", path))?;
    let width = img.width();
    let height = img.height();

    Ok(ImageArrangement{
        file_name: path.to_str().unwrap().to_string(),
        w: width,
        h: height,
        x: 0.5,
        y: 0.5,
        angle: 0.0
    })
}

pub fn create_thumbnails(config: &PanolutionConfig) -> Result<Vec<Solution>> {
    let mut all_image_paths: Vec<PathBuf> = Vec::new();
    let mut result = Vec::new();

    for entry in WalkDir::new(&config.input_path) {
        let entry = entry.chain_err(|| "error in WalkDir")?;

        if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                if valid_image_file(file_name) {
                    if file_name.starts_with("thumb_") {
                        info!("Ignore thumbnail file: {:?}", file_name);
                    } else {
                        all_image_paths.push(entry.path().to_path_buf());
                    }
                } else {
                    info!("Image format currently not supported: {}", file_name);
                }
            } else {
                info!("Could not convert file name to str: {:?}", entry);
            }
        } else {
            info!("Ignore non-file: {:?}", entry);
        }
    }

    for scale_factor in &config.scale_factors {
        let mut arrangement = Vec::new();

        for path in &all_image_paths {
            if *scale_factor >= 1.0 {
                info!("Add original image");

                arrangement.push(create_image_arrangement(&path)?);
            } else {
                let base_path = path.parent().unwrap();
                let file_name = path.file_name().unwrap(); // Save to unwrap since we checked above
                let thumb_path = base_path.join(format!("thumb_{}_{}", scale_factor, file_name.to_str().unwrap()));

                if thumb_path.exists() {
                    info!("Thumbnail was already generated, using old one: '{:?}'", &thumb_path);
                    arrangement.push(create_image_arrangement(&thumb_path)?);
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

                    arrangement.push(
                        ImageArrangement{
                            file_name: thumb_path.to_str().unwrap().to_string(),
                            w: thumb_w,
                            h: thumb_h,
                            x: 0.5,
                            y: 0.5,
                            angle: 0.0
                        }
                    );
                }
            }
        }

        result.push(Solution{arrangement: arrangement});
    }

    Ok(result)
}
