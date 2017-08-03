// External modules:
use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};
use image;
// use image::{GenericImage, FilterType, DynamicImage, imageops};
use rand::Rng;
use rand;
use walkdir::{WalkDir};


// Std modules:
use std::cmp;
use std::path::Path;
use std::f32::consts::PI;

// Internal modules:
use config::PanolutionConfig;



fn valid_image_file(file_name: &str) -> bool {
    let extension = file_name.split(".").last().unwrap_or("").to_lowercase();

    let supported: Vec<String> = vec!["jpg", "jpeg", "gif", "png", "tif", "tiff"].iter().map(|s| s.to_string()).collect();

    supported.contains(&extension)
}




#[derive(Clone)]
pub struct ImageArrangement {
    pub file_name: String, // TODO: Share path between individuals
    // TODO: Add image and share it, so it doesn't have to be re-loaded every time
    pub x: u32,
    pub y: u32,
    pub angle: f32,
    // TODO: add more image operations
}

#[derive(Clone)]
pub struct Solution {
    pub arrangement: Vec<ImageArrangement>,
}


fn make_all_populations(num_of_individuals: u32, num_of_populations: u32, initial_solution: &Solution) -> Vec<Population<Solution>> {
    let mut result = Vec::new();

    let mut initial_population = Vec::new();

    for _ in 0..num_of_individuals {
        initial_population.push(initial_solution.clone());
    }

    for i in 0..num_of_populations {
        let pop = PopulationBuilder::<Solution>::new()
            .set_id(i + 1)
            .initial_population(&initial_population)
            .reset_limit_end(0)
            .increasing_mutation_rate()
            .finalize().unwrap();

        result.push(pop);
    }

    result
}

impl Individual for Solution {
    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let num_of_operations: u8 = 4;

        let index1: usize = rng.gen_range(0, self.arrangement.len());

        // Probably use this create and enums for operation:
        // https://github.com/andersk/enum_primitive-rs
        let operation: u8 = rng.gen_range(0, num_of_operations);

        match operation {
            0 => {
                // Change image order
                let mut index2: usize = rng.gen_range(0, self.arrangement.len());
                while index1 == index2 {
                    index2 = rng.gen_range(0, self.arrangement.len());
                }

                self.arrangement.swap(index1, index2);
            },
            1 => {
                // Move x
                self.arrangement[index1].x = self.arrangement[index1].x + rng.gen_range(0, 500) - 250;
                if self.arrangement[index1].x < 0 {
                    self.arrangement[index1].x = 0;
                }
            },
            2 => {
                // Move y
                self.arrangement[index1].y = self.arrangement[index1].y + rng.gen_range(0, 500) - 250;
                if self.arrangement[index1].y < 0 {
                    self.arrangement[index1].y = 0;
                }
            },
            3 => {
                // Rotate
                self.arrangement[index1].angle = self.arrangement[index1].angle + (rng.next_f32() * 2.0 * PI);
            },
            op => {
                warn!("mutate(): unknown operation: {}", op)
            }
        }
    }

    fn calculate_fitness(&mut self) -> f64 {
        let mut fitness = 0.0;

        fitness
    }

    fn reset(&mut self) {
        for arrangement in &mut self.arrangement {
            // Reset all image operation to original image
            arrangement.x = 0;
            arrangement.y = 0;
            arrangement.angle = 0.0;
        }
    }
}

fn run_darwin(solution: &Solution, config: &PanolutionConfig) -> Solution {
    info!("Run darwin with maximum number of iterations: {}", config.max_iteration);

    let pano = SimulationBuilder::<Solution>::new()
        .iterations(config.max_iteration)
        .threads(config.num_of_threads)
        .add_multiple_populations(make_all_populations(10, 8, &solution))
        .finalize();

    match pano {
        Err(e) => {
            error!("An error occured");

            for e in e.iter().skip(1) {
                error!("Caused by '{}'", e)
            }

            solution.clone()
        },
        Ok(mut pano_simulation) => {
            pano_simulation.run();

            info!("Total run time: {} ms", pano_simulation.total_time_in_ms);
            info!("Improvement factor: {}", pano_simulation.simulation_result.improvement_factor);
            info!("Number of iterations: {}", pano_simulation.simulation_result.iteration_counter);

            pano_simulation.simulation_result.fittest[0].individual.clone()
        }
    }
}

pub fn optimize(solution: Option<&Solution>, config: &PanolutionConfig) -> Solution {
    match solution {
        None => {
            let mut arrangement = Vec::new();

            for entry in WalkDir::new(&config.input_path) {
                if let Ok(entry) = entry {
                    if entry.file_type().is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if valid_image_file(file_name) {
                                arrangement.push(ImageArrangement{
                                    file_name: entry.path().to_str().unwrap().to_string(),
                                    x: 0,
                                    y: 0,
                                    angle: 0.0,
                                });
                            } else {
                                info!("Image format currently not supported: {}", file_name);
                            }
                        } else {
                            info!("Could not convert file name to str: {:?}", entry);
                        }
                    } else {
                        info!("Ignore non-file: {:?}", entry);
                    }
                } else {
                    info!("Error in WalkDir");
                }
            }

            run_darwin(&Solution{arrangement: arrangement}, &config)
        },
        Some(solution) => {
            run_darwin(solution, &config)
        }
    }
}
