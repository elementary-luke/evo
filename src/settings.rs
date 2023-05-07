pub struct Settings {}

impl Settings {
    //creation of creatures
    pub const X_BOUND : f32 = 200.0;
    pub const Y_BOUND : f32 = 200.0;
    pub const SLIP_MIN : f32 = 0.0;
    pub const SLIP_MAX : f32 = 0.95;
    pub const STRENGTH_MIN : f32 = 40.0;
    pub const STRENGTH_MAX : f32= 120.0;
    pub const MAX_CIRCLES : usize = 6; // maybe remove and make circles take up energy/s to discourage too many

    //physics
    pub const GRAV : f32 = 3.2;// REVERT to 3.2
    pub const FLOOR_Y :f32 = 400.0;
}