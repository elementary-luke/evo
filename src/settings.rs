#[derive(Clone, Debug)]
pub struct Settings {
    
    pub random_seed : bool,
    pub seed : u64,

    pub population_size : usize,

    //creation of base creatures
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

    //children settings
    pub cx_bound : f32,
    pub cy_bound : f32,

    pub cslip_min : f32,
    pub cslip_max : f32,

    pub cstrength_min : f32,
    pub cstrength_max : f32,

    pub ccontracted_time_min : f32,
    pub ccontracted_time_max : f32,

    pub cextended_time_min : f32,
    pub cextended_time_max : f32,

    pub ccontracted_len_min : f32,
    pub ccontracted_len_max : f32,

    pub cextended_len_min : f32,
    pub cextended_len_max : f32,

    pub cmin_circles : usize,
    pub cmax_circles : usize,

    //physics
    pub time_given : f32,
    pub grav : f32,
    pub drag : f32,
    pub floor_y : f32,
    pub distance_based_on : usize, // 0 is avg, 1 is max 
    pub hurdles : bool,
    pub ceiling : bool,
    pub stairs : bool,
    pub terrain : Vec<(f32, f32, f32, f32)>, //x1, y1, x2, y2
    pub heuristic : usize,
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
            cx_bound: 10000.0,
            cy_bound: 10000.0, 
            cslip_min: 0.0, 
            cslip_max: 1.0,
            cstrength_min: 0.0, 
            cstrength_max: 10000.0, 
            ccontracted_time_min: 0.0, 
            ccontracted_time_max: 10000.0, 
            cextended_time_min: 0.0, 
            cextended_time_max: 10000.0, 
            ccontracted_len_min: 0.0, 
            ccontracted_len_max: 10000.0, 
            cextended_len_min: 0.0, 
            cextended_len_max: 10000.0, 
            cmin_circles: 1, 
            cmax_circles: 10, 
            time_given: 20.0, 
            grav: 0.4, 
            drag: 0.9, 
            floor_y: 400.0,
            distance_based_on: 0,
            hurdles: false,
            ceiling: false,
            stairs: false,
            terrain: vec![], //(x1, y1, x2, y2)
            heuristic: 0,
        }
    }
}