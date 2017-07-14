// External modules:
use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};
use image;
use image::{GenericImage, FilterType};

// Internal modules:
use config::PanolutionConfig;

pub struct ImageArrangement {
    file_name: String,
    pos_x: u64,
    pos_y: u64,
    rotation: f64,
}

pub struct Solution {
    arrangement: Vec<ImageArrangement>,
}

fn run_darwin(solution: Solution, max_iteration: u64) -> Solution {
    solution
}

pub fn optimize(solution: Option<&Solution>, config: &PanolutionConfig, thumbnail_path: &Vec<String>) -> Solution {
    let mut result = Solution{ arrangement: Vec::new() };

    // Prepare data
    if let Some(solution) = solution {
        result.arrangement = solution.arrangement.iter().zip(thumbnail_path).map(|(arrangement, path)| ImageArrangement {
                file_name: path.clone(),
                pos_x: arrangement.pos_x,
                pos_y: arrangement.pos_y,
                rotation: arrangement.rotation,
            }
        ).collect();
    } else {
        result.arrangement = thumbnail_path.iter().map(|path| ImageArrangement {
                file_name: path.clone(),
                pos_x: 0,
                pos_y: 0,
                rotation: 0.0
            }
        ).collect();
    }

    run_darwin(result, config.max_iteration)
}
