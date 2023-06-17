pub struct Settings {
    //creation of creatures
    pub random_seed : bool,
    pub seed : u64,

    pub population_size : usize,

    pub x_bound : f32,
    pub y_bound : f32,

    pub slip_min : f32,
    pub slip_max : f32,

    pub strength_min : f32,
    pub strength_max : f32,

    pub contracted_time_min : f32,
    pub contracted_time_max : f32,

    pub extended_time_min : f32,
    pub extended_time_max : f32,

    pub contracted_len_min : f32,
    pub contracted_len_max : f32,

    pub extended_len_min : f32,
    pub extended_len_max : f32,

    pub min_circles : usize,
    pub max_circles : usize,

    //physics
    pub time_given : f32,
    pub grav : f32,
    pub drag : f32,
    pub floor_y : f32,

}

impl Default for Settings
{
    fn default() -> Self
    {
        Settings { 
            random_seed: true, 
            seed: 1684620265, 
            population_size: 600, 
            x_bound: 200.0, 
            y_bound: 200.0, 
            slip_min: 0.0, 
            slip_max: 1.0, 
            strength_min: 30.0, 
            strength_max: 80.0, 
            contracted_time_min: 0.5, 
            contracted_time_max: 1.5, 
            extended_time_min: 0.5, 
            extended_time_max: 1.5, 
            contracted_len_min: 10.0, 
            contracted_len_max: 50.0, 
            extended_len_min: 55.0, 
            extended_len_max: 100.0, 
            min_circles: 3, 
            max_circles: 5, 
            time_given: 20.0, 
            grav: 0.4, 
            drag: 0.9, 
            floor_y: 400.0,
        }
    }
}