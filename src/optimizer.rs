// External modules:
use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};

// Internal modules:
use config::PanolutionConfig;

pub struct ImageArrangement {
    file_name: String,
    pos_x: u64,
    pos_y: u64,
    rotation: f64,
}

pub fn optimize(arrangement: Option<&Vec<ImageArrangement>>, config: &PanolutionConfig, thumbnail_path: &Vec<String>) -> Vec<ImageArrangement> {
    let mut result = Vec::new();

    if let Some(arrangement) = arrangement {
        result = arrangement.iter().zip(thumbnail_path).map(|(arrangement, path)| ImageArrangement {
                file_name: path.clone(),
                pos_x: arrangement.pos_x,
                pos_y: arrangement.pos_y,
                rotation: arrangement.rotation,
            }
        ).collect();
    } else {
        result = thumbnail_path.iter().map(|path| ImageArrangement {
                file_name: path.clone(),
                pos_x: 0,
                pos_y: 0,
                rotation: 0.0
            }
        ).collect();
    }

    result
}
