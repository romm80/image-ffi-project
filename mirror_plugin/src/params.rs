use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct MirrorParams {
    pub horizontal: bool,
    pub vertical: bool,
}

