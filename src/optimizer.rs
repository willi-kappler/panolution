// External modules:
use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};
use image;
use image::{GenericImage, FilterType};
use rand::Rng;
use rand;

// Internal modules:
use config::PanolutionConfig;

#[derive(Clone)]
pub struct ImageArrangement {
    file_name: String, // TODO: Share path between individuals
    // TODO: Add image and share it, so it doesn't have to be re-loaded every time
    pos_x: u64,
    pos_y: u64,
    rotation: f64,
    // TODO: add more image operations
}

#[derive(Clone)]
pub struct Solution {
    arrangement: Vec<ImageArrangement>,
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

        // Probably use this create and enums:
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
                self.arrangement[index1].pos_x = self.arrangement[index1].pos_x + (rng.gen_range(0, 3) - 1);
            },
            2 => {
                // Move y
                self.arrangement[index1].pos_y = self.arrangement[index1].pos_y + (rng.gen_range(0, 3) - 1);
            },
            3 => {
                // Rotate
                let angle: f64 = ((rng.gen_range(0, 21) - 10) as f64) * 0.1;
                self.arrangement[index1].rotation = self.arrangement[index1].rotation + angle;
            },
            op => {
                warn!("mutate(): unknown operation: {}", op)
            }
        }
    }

    fn calculate_fitness(&mut self) -> f64 {
        0.0
    }

    fn reset(&mut self) {
        for arrangement in &mut self.arrangement {
            arrangement.pos_x = 0;
            arrangement.pos_y = 0;
            arrangement.rotation = 0.0;
        }
    }
}

fn run_darwin(solution: Solution, max_iteration: u32) -> Solution {
    let pano = SimulationBuilder::<Solution>::new()
        .iterations(max_iteration)
        .threads(4) // TODO: Make this configurable
        .add_multiple_populations(make_all_populations(100, 8, &solution))
        .finalize();

    match pano {
        Err(e) => {
            error!("An error occured");

            for e in e.iter().skip(1) {
                error!("Caused by '{}'", e)
            }

            solution
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
