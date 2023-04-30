pub struct Settings {}

impl Settings {
    //creation of creatures
    pub const X_BOUND : f32 = 200.0;
    pub const Y_BOUND : f32 = 200.0;
    pub const SLIP_MIN : f32 = 0.0;
    pub const SLIP_MAX : f32 = 0.4;

    //physics
    pub const GRAV : f32 = 3.2;
    pub const FLOOR_Y :f32 = 400.0;
}