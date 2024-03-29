
#[derive(Savefile, Debug)]
pub struct Bubble {
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
    pub r: f32,
    pub decay: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub lifetime: f32,
}
#[derive(Clone, PartialEq, Savefile, Debug)]
pub struct TextReadout {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub lifetime: f32,
}



