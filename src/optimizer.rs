use config::PanolutionConfig;

pub struct ImageArrangement {
    filename: String,
    pos_x: u64,
    pos_y: u64,
    rotation: u16,
}

pub fn optimize(arrangement: Option<&Vec<ImageArrangement>>, config: &PanolutionConfig, scale_index: usize) -> Vec<ImageArrangement> {
    let mut result = Vec::new();

    result
}
