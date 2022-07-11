// External modules:
use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};
use image;
// use image::{GenericImage, FilterType, DynamicImage, imageops};
use image::{GenericImageView, DynamicImage, Rgba};
use rand::Rng;
use rand;
use walkdir::{WalkDir};

// use std::path::Path;
use std::f64::consts::PI;
use std::cmp;
use std::sync::Arc;

// Internal modules:
use config::PanolutionConfig;

#[derive(Clone)]
pub struct ImageArrangement {
    pub image: Arc<DynamicImage>,
    pub full_path: String,
    pub samples: u32,
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32,
    pub angle: f64,
    // TODO: add more image operations
}

#[derive(Clone)]
pub struct Solution {
    pub arrangement: Vec<ImageArrangement>,
}

struct SamplePixel{
    x: u32,
    y: u32,
    pixels: Vec<(u8, u8, u8)>
}

fn valid_image_file(file_name: &str) -> bool {
    let extension = file_name.split(".").last().unwrap_or("").to_lowercase();

    let supported: Vec<String> = vec!["jpg", "jpeg", "gif", "png", "tif", "tiff"].iter().map(|s| s.to_string()).collect();

    supported.contains(&extension)
}

fn calc_canvas_size(arrangement: &Vec<ImageArrangement>) -> (u32, u32, u32, u32) {
    arrangement.iter().fold((u32::max_value(), u32::max_value(), 0, 0), |(cx0, cy0, cx1, cy1), im_ar| {
        (cmp::min(cx0, im_ar.x0), cmp::min(cy0, im_ar.y0), cmp::max(cx1, im_ar.x1), cmp::max(cy1, im_ar.y1))
    })
}

fn get_pixel(cx: f64, cy: f64, im_ar: &ImageArrangement) -> Option<(u8, u8, u8)> {
    // Center of image
    let mx = ((im_ar.x0 + im_ar.x1) / 2) as f64;
    let my = ((im_ar.y0 + im_ar.y1) / 2) as f64;

    // Move center of rotation to origin (0,0)
    let ox = cx - mx;
    let oy = cy - my;

    // Rotate canvas point in reverse of image orientation (clockwise)
    let cos_a = im_ar.angle.cos();
    let sin_a = im_ar.angle.sin();

    let rx = (ox * cos_a) + (oy * sin_a);
    let ry = (-ox * sin_a) + (oy * cos_a);

    // Move back to image coordinate system
    let imx = ((rx + mx) as i32) - (im_ar.x0 as i32);
    let imy = ((ry + my) as i32) - (im_ar.y0 as i32);

    let width = im_ar.image.width() as i32;
    let height = im_ar.image.height() as i32;

    // Check if point is inside the image:

    if (imx >= 0) && (imx < width) && (imy >= 0) && (imy < height) {
        let x = imx as u32;
        let y = imy as u32;
        let pixel = im_ar.image.get_pixel(x, y) as Rgba<u8>;
        return Some((pixel.0[0], pixel.0[1], pixel.0[2]));
    }

    None
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

                let px0 = self.arrangement[index1].x0;
                let py0 = self.arrangement[index1].y0;
                let px1 = self.arrangement[index1].x1;
                let py1 = self.arrangement[index1].y1;

                self.arrangement[index1].x0 = self.arrangement[index2].x0;
                self.arrangement[index1].y0 = self.arrangement[index2].y0;
                self.arrangement[index1].x1 = self.arrangement[index2].x1;
                self.arrangement[index1].y1 = self.arrangement[index2].y1;

                self.arrangement[index2].x0 = px0;
                self.arrangement[index2].y0 = py0;
                self.arrangement[index2].x1 = px1;
                self.arrangement[index2].y1 = py1;
            },
            1 => {
                // Move x
                let dx = rng.gen_range::<i32>(0, 500) - 250;
                let new_x = (self.arrangement[index1].x0 as i32) + dx;

                if new_x >= 0 {
                    self.arrangement[index1].x0 = new_x as u32;
                    self.arrangement[index1].x1 = ((self.arrangement[index1].x1 as i32) + dx) as u32;
                }
            },
            2 => {
                // Move y
                let dy = rng.gen_range::<i32>(0, 500) - 250;
                let new_y = (self.arrangement[index1].y0 as i32) + dy;

                if new_y >= 0 {
                    self.arrangement[index1].y0 = new_y as u32;
                    self.arrangement[index1].y1 = ((self.arrangement[index1].y1 as i32) + dy) as u32;
                }
            },
            3 => {
                // Rotate
                self.arrangement[index1].angle = self.arrangement[index1].angle + (rng.next_f64() * 0.1);
                if self.arrangement[index1].angle < 0.0 {
                    self.arrangement[index1].angle = 0.0;
                } else if self.arrangement[index1].angle > 2.0 * PI {
                    self.arrangement[index1].angle = 0.0;
                }
            },
            op => {
                warn!("mutate(): unknown operation: {}", op)
            }
        }
    }

    fn calculate_fitness(&mut self) -> f64 {
        let num_of_samples = self.arrangement[0].samples;

        let (cx0, cy0, cx1, cy1) = calc_canvas_size(&self.arrangement);

        // Create sample points on canvas:
        let mut rng = rand::thread_rng();
        let mut sample_pixels = Vec::new();

        for _ in 0..num_of_samples {
            sample_pixels.push(SamplePixel{
                x: rng.gen_range::<u32>(cx0, cx1),
                y: rng.gen_range::<u32>(cy0, cy1),
                pixels: Vec::new()
            });
        }

        for im_ar in &self.arrangement {
            for sp in &mut sample_pixels {
                if let Some(pixel) = get_pixel(sp.x as f64, sp.y as f64, &im_ar) {
                    sp.pixels.push(pixel);
                }
            }
        }

        let error_sum = sample_pixels.iter().fold(0_u32, |sum, sp| {
            let num_of_pixels = sp.pixels.len();

            if num_of_pixels == 0 {
                // No pixel at all, random sample is outside of any image
                sum
            } else if num_of_pixels == 1 {
                // No overlap, add penalty
                sum + 255
            } else {
                let r0 = sp.pixels[0].0;
                let g0 = sp.pixels[0].1;
                let b0 = sp.pixels[0].2;
                let max_diff = sp.pixels.iter().fold((0_u32, r0, g0, b0), |(diff, r1, g1, b1), &(r2, g2, b2)| {
                    let rdiff = if r1 > r2 {r1 - r2} else {r2 - r1};
                    let gdiff  = if g1 > g2 {g1 - g2} else {g2 - g1};
                    let bdiff  = if b1 > b2 {b1 - b2} else {b2 - b1};

                    (cmp::max(diff, (rdiff as u32) + (gdiff as u32) + (bdiff as u32)), r2, g2, b2)
                });
                sum + max_diff.0
            }

        });

        (error_sum as f64) / (num_of_samples as f64)
    }

    fn reset(&mut self) {
        for arrangement in &mut self.arrangement {
            // Reset all image operation to original image
            arrangement.x1 = arrangement.x1 - arrangement.x0;
            arrangement.y1 = arrangement.y1 - arrangement.y0;
            arrangement.x0 = 0;
            arrangement.y0 = 0;
            arrangement.angle = 0.0;
        }
    }
}

fn run_darwin(solution: &Solution, config: &PanolutionConfig) -> Solution {
    let pano = SimulationBuilder::<Solution>::new()
        .iterations(config.max_iteration)
        .threads(4)
        .output_every(0)
        .add_multiple_populations(make_all_populations(20, 8, &solution))
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
            info!("New fitness: {}", pano_simulation.simulation_result.fittest[0].fitness);

            pano_simulation.simulation_result.fittest[0].individual.clone()
        }
    }
}

pub fn optimize(solution: Option<&Solution>, config: &PanolutionConfig, sample_index: usize) -> Solution {
    match solution {
        None => {
            let mut arrangement = Vec::new();

            for entry in WalkDir::new(&config.input_path) {
                if let Ok(entry) = entry {
                    if entry.file_type().is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if valid_image_file(file_name) {
                                if let Some(full_path) = entry.path().to_str() {

                                    if let Ok(img) = image::open(&full_path) {
                                        arrangement.push(ImageArrangement{
                                            image: Arc::new(img.clone()),
                                            full_path: full_path.to_string(),
                                            samples: config.num_of_samples[sample_index],
                                            x0: 0,
                                            y0: 0,
                                            x1: img.width(),
                                            y1: img.height(),
                                            angle: 0.0,
                                        });
                                    } else {
                                        error!("Could not open image: {}", full_path)
                                    }
                                } else {
                                    error!("Could not convert full path to str: {:?}", entry);
                                }
                            } else {
                                error!("Image format currently not supported: {}", file_name);
                            }
                        } else {
                            error!("Could not convert file name to str: {:?}", entry);
                        }
                    } else {
                        info!("Ignore non-file: {:?}", entry);
                    }
                } else {
                    error!("Error in WalkDir");
                }
            }

            let solution = Solution{arrangement: arrangement};

            if solution.arrangement.len() > 0 {
                run_darwin(&solution, &config)
            } else {
                // An error occured
                // TODO: return Result<Soultion> and use chain_err
                solution
            }
        },
        Some(solution) => {
            let mut updated_arrangement = solution.arrangement.clone();

            for im_ar in &mut updated_arrangement {
                im_ar.samples = config.num_of_samples[sample_index];
            }

            run_darwin(&Solution{arrangement: updated_arrangement}, &config)
        }
    }
}
