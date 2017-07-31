// External modules:
use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};
use image;
use image::{GenericImage, FilterType, DynamicImage, imageops};
use rand::Rng;
use rand;
use imageproc::{stats, affine};

// Std modules:
use std::cmp;
use std::path::Path;

// Internal modules:
use config::PanolutionConfig;

#[derive(Clone)]
pub struct ImageArrangement {
    pub file_name: String, // TODO: Share path between individuals
    // TODO: Add image and share it, so it doesn't have to be re-loaded every time
    pub x: f64,
    pub y: f64,
    pub angle: f32,
    // TODO: add more image operations
}

#[derive(Clone)]
pub struct Solution {
    pub arrangement: Vec<ImageArrangement>,
}

impl Solution {
    pub fn update_path_from(&mut self, solution: &Solution) {
        for (new, old) in self.arrangement.iter_mut().zip(&solution.arrangement) {
            new.file_name = old.file_name.clone();
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Rectangle {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

fn calc_extendion(solution: &Solution) -> (u32, u32) {
    solution.arrangement.iter().fold(
        (0, 0), |(total_w, total_h), elem| {
            let image = image::open(&elem.file_name).unwrap();
            let img_w = image.width();
            let img_h = image.height();
            (cmp::max(total_w, img_w), cmp::max(total_h, img_h))
        }
    )
}

fn calc_intersection(x1: u32, x2: u32, y1: u32, y2: u32, w1: u32, w2: u32, h1: u32, h2: u32) -> (Rectangle, Rectangle) {
    // rect1 and rect2 refer to positions inside the images. Not the absolute positions.
    let mut rect1 = Rectangle{x: 0, y: 0, w: 0, h: 0};
    let mut rect2 = Rectangle{x: 0, y: 0, w: 0, h: 0};

    if x1 == x2 {
        rect1.x = 0;
        rect2.x = 0;

        if w1 < w2 {
            rect1.w = w1;
            rect2.w = w1;
        } else {
            rect1.w = w2;
            rect2.w = w2;
        }
    }

    if y1 == y2 {
        rect1.y = 0;
        rect2.y = 0;

        if h1 < h2 {
            rect1.h = h1;
            rect2.h = h1;
        } else {
            rect1.h = h2;
            rect2.h = h2;
        }
    }

    let dx = (x2 as i32 - x1 as i32).abs() as u32;
    let dy = (y2 as i32 - y1 as i32).abs() as u32;

    if x1 < x2 && dx < w1 {
        rect1.x = dx;
        rect1.w = cmp::min(w1 - dx, w2);
        rect2.x = 0;
        rect2.w = rect1.w;
    }

    if x2 < x1 && dx < w2 {
        rect2.x = dx;
        rect2.w = cmp::min(w2 - dx, w1);
        rect1.x = 0;
        rect1.w = rect2.w;
    }

    if y1 < y2 && dy < h1 {
        rect1.y = dy;
        rect1.h = cmp::min(h1 - dy, h2);
        rect2.y = 0;
        rect2.h = rect1.h;
    }

    if y2 < y1 && dy < h2 {
        rect2.y = dy;
        rect2.h = cmp::min(h2 - dy, h1);
        rect1.y = 0;
        rect1.h = rect2.h;
    }

    (rect1, rect2)
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
        let num_of_operations: u8 = 7;

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
                // Move x, small step
                self.arrangement[index1].x = self.arrangement[index1].x + ((rng.next_f64() - 0.5) * 0.1);
                if self.arrangement[index1].x < 0.0 {
                    self.arrangement[index1].x = 0.0;
                } else if self.arrangement[index1].x > 1.0 {
                    self.arrangement[index1].x = 1.0;
                }
            },
            2 => {
                // Move x, big step
                self.arrangement[index1].x = self.arrangement[index1].x + rng.next_f64() - 0.5;
                if self.arrangement[index1].x < 0.0 {
                    self.arrangement[index1].x = 0.0;
                } else if self.arrangement[index1].x > 1.0 {
                    self.arrangement[index1].x = 1.0;
                }
            },
            3 => {
                // Move y, small step
                self.arrangement[index1].y = self.arrangement[index1].y + ((rng.next_f64() - 0.5) * 0.1);
                if self.arrangement[index1].y < 0.0 {
                    self.arrangement[index1].y = 0.0;
                } else if self.arrangement[index1].y > 1.0 {
                    self.arrangement[index1].y = 1.0;
                }
            },
            4 => {
                // Move y, big step
                self.arrangement[index1].y = self.arrangement[index1].y + rng.next_f64() - 0.5;
                if self.arrangement[index1].y < 0.0 {
                    self.arrangement[index1].y = 0.0;
                } else if self.arrangement[index1].y > 1.0 {
                    self.arrangement[index1].y = 1.0;
                }
            },
            5 => {
                // Rotate, small step
                self.arrangement[index1].angle = self.arrangement[index1].angle + ((rng.next_f32() - 0.5) * 0.1);
            },
            6 => {
                // Rotate, big step
                self.arrangement[index1].angle = self.arrangement[index1].angle + (rng.next_f32() - 0.5);
            },
            op => {
                warn!("mutate(): unknown operation: {}", op)
            }
        }
    }

    fn calculate_fitness(&mut self) -> f64 {
        let mut fitness = 0.0;

        for i in 0..(self.arrangement.len() - 1) {
            let arrangement1 = &self.arrangement[i];
            let arrangement2 = &self.arrangement[i + 1];

            let image1 = image::open(&arrangement1.file_name).unwrap();
            let image2 = image::open(&arrangement2.file_name).unwrap();

            let mut image1 = affine::rotate_about_center(image1.as_rgb8().unwrap(), arrangement1.angle, affine::Interpolation::Nearest);
            let mut image2 = affine::rotate_about_center(image2.as_rgb8().unwrap(), arrangement1.angle, affine::Interpolation::Nearest);

            let w1 = image1.width();
            let w2 = image2.width();
            let h1 = image1.height();
            let h2 = image2.height();

            let area_max = cmp::max(w1 * h1, w2 * h2) as f64;

            let (rect1, rect2) = calc_intersection(
                arrangement1.x as u32, arrangement2.x as u32, arrangement1.y as u32, arrangement2.y as u32,
                w1, w2, h1, h2
            );

            if rect1.w == 0 || rect1.h == 0 {
                // Images don't intersect, add penalty for this individual
                fitness = fitness + area_max;
            } else {
                let intersection_area = rect1.w * rect1.h;
                let sub_image1 = imageops::crop(&mut image1, rect1.x, rect1.y, rect1.w, rect1.h);
                let sub_image2 = imageops::crop(&mut image2, rect2.x, rect2.y, rect2.w, rect2.h);

                fitness = fitness + (area_max * stats::root_mean_squared_error(&sub_image1, &sub_image2) / intersection_area as f64);
            }
        }

        fitness
    }

    fn reset(&mut self) {
        for arrangement in &mut self.arrangement {
            // Reset all image operation to original image
            arrangement.x = 0.0;
            arrangement.y = 0.0;
            arrangement.angle = 0.0;
        }
    }
}

fn run_darwin(solution: &Solution, max_iteration: u32) -> Solution {
    info!("Run darwin with maximum number of iterations: {}", max_iteration);

    let pano = SimulationBuilder::<Solution>::new()
        .iterations(max_iteration)
        .threads(4) // TODO: Make this configurable
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

pub fn optimize(solution: &Solution, config: &PanolutionConfig) -> Solution {
    run_darwin(solution, config.max_iteration)
}

#[cfg(test)]
mod test {
    use super::{calc_intersection, Rectangle};

    #[test]
    fn calc_intersection1() {
        let (r1, r2) = calc_intersection(0, 0, 0, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 20});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 10, h: 20});
    }

    #[test]
    fn calc_intersection2() {
        let (r1, r2) = calc_intersection(0, 0, 0, 0, 15, 10, 25, 20);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 20});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 10, h: 20});
    }

    #[test]
    fn calc_intersection3() {
        let (r1, r2) = calc_intersection(50, 0, 0, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 0, h: 20});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 0, h: 20});
    }

    #[test]
    fn calc_intersection4() {
        let (r1, r2) = calc_intersection(0, 50, 0, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 0, h: 20});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 0, h: 20});
    }

    #[test]
    fn calc_intersection5() {
        let (r1, r2) = calc_intersection(0, 0, 50, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 0});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 10, h: 0});
    }

    #[test]
    fn calc_intersection6() {
        let (r1, r2) = calc_intersection(0, 0, 0, 50, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 0});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 10, h: 0});
    }

    #[test]
    fn calc_intersection7() {
        let (r1, r2) = calc_intersection(50, 0, 50, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 0, h: 0});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 0, h: 0});
    }

    #[test]
    fn calc_intersection8() {
        let (r1, r2) = calc_intersection(0, 50, 0, 50, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 0, h: 0});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 0, h: 0});
    }

    #[test]
    fn calc_intersection9() {
        let (r1, r2) = calc_intersection(3, 0, 0, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 20});
        assert_eq!(r2, Rectangle{x: 3, y: 0, w: 10, h: 20});
    }

    #[test]
    fn calc_intersection10() {
        let (r1, r2) = calc_intersection(7, 0, 0, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 8, h: 20});
        assert_eq!(r2, Rectangle{x: 7, y: 0, w: 8, h: 20});
    }

    #[test]
    fn calc_intersection11() {
        let (r1, r2) = calc_intersection(0, 8, 0, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 8, y: 0, w: 2, h: 20});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 2, h: 20});
    }

    #[test]
    fn calc_intersection12() {
        let (r1, r2) = calc_intersection(0, 0, 4, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 20});
        assert_eq!(r2, Rectangle{x: 0, y: 4, w: 10, h: 20});
    }

    #[test]
    fn calc_intersection13() {
        let (r1, r2) = calc_intersection(0, 0, 9, 0, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 0, w: 10, h: 16});
        assert_eq!(r2, Rectangle{x: 0, y: 9, w: 10, h: 16});
    }

    #[test]
    fn calc_intersection14() {
        let (r1, r2) = calc_intersection(0, 0, 0, 12, 10, 15, 20, 25);

        assert_eq!(r1, Rectangle{x: 0, y: 12, w: 10, h: 8});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 10, h: 8});
    }

    #[test]
    fn calc_intersection15() {
        let (r1, r2) = calc_intersection(0, 8, 0, 9, 10, 10, 10, 10);

        assert_eq!(r1, Rectangle{x: 8, y: 9, w: 2, h: 1});
        assert_eq!(r2, Rectangle{x: 0, y: 0, w: 2, h: 1});
    }

}
