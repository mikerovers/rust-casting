pub struct Player {
    pub x_position: f32,
    pub y_position: f32,
    pub angle: f32,
    pub fov: f32
}

impl Player {
    pub fn new(x_position: f32, y_position: f32, angle: f32, fov: f32) -> Player {
        Player{x_position, y_position, angle, fov}
    }
}