use serde::Deserialize;

#[derive(Deserialize)]
pub struct BlurParams {
    pub radius: u32,
    pub iterations: u32,
}

impl Default for BlurParams {
    fn default() -> Self {
        Self {
            radius: 1,
            iterations: 1,
        }
    }
}
