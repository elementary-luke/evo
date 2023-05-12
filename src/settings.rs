pub struct Settings {}

impl Settings {
    //creation of creatures
    pub const RANDOM_SEED : bool = true;
    pub const SEED : u64 = 0;

    pub const POPULATION_SIZE : usize = 600;

    pub const X_BOUND : f32 = 200.0;
    pub const Y_BOUND : f32 = 200.0;

    pub const SLIP_MIN : f32 = 0.0;
    pub const SLIP_MAX : f32 = 1.0;

    pub const STRENGTH_MIN : f32 = 30.0;
    pub const STRENGTH_MAX : f32 = 80.0;

    pub const CONTRACTED_TIME_MIN : f32 = 0.5;
    pub const CONTRACTED_TIME_MAX : f32 = 1.5;

    pub const EXTENDED_TIME_MIN : f32 = 0.5;
    pub const EXTENDED_TIME_MAX : f32 = 1.5;

    pub const CONTRACTED_LEN_MIN : f32 = 10.0;
    pub const CONTRACTED_LEN_MAX : f32 = 50.0;

    pub const EXTENDED_LEN_MIN : f32 = 55.0;
    pub const EXTENDED_LEN_MAX : f32 = 100.0;

    pub const MIN_CIRCLES : usize = 3;
    pub const MAX_CIRCLES : usize = 5; // maybe remove and make circles take up energy/s to discourage too many

    //physics
    pub const TIME_GIVEN : f32 = 20.0;
    pub const GRAV : f32 = 0.4;// REVERT to 3.2
    pub const DRAG : f32 = 0.9;
    pub const FLOOR_Y :f32 = 400.0;
}