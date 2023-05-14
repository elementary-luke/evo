use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use crate::settings::Settings;
use macroquad::qrand as rand;
use std::cmp::min;

#[derive(Clone, Debug)]
pub struct Body
{
    pub start_avg_x : f32,
    pub pos : Point,
    pub circles : Vec<Circle>,
    pub muscles : Vec<Muscle>,
    pub distance : Option<f32>,
    pub parent : Option<(usize, usize)>,
    pub children : Vec<(usize, usize)>,
    pub previous : Option<usize>,
    pub next : Option<usize>,
    pub age : usize,
}

impl Default for Body
{
    fn default() -> Self
    {
        Body 
        {
            start_avg_x : 0.0,
            pos : Point {x : 0.0, y : 0.0},
            circles : Vec::new(),
            muscles : Vec::new(),
            distance : None,
            parent : None,
            children : Vec::new(),
            age : 0,
            previous : None,
            next : None,
        }
    }
}

impl Body
{
    pub fn new() -> Body // new empty body
    {
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body : Body = Body {..Default::default()};
        body
    }
    pub fn new_random(x_bound : f32, y_bound : f32) -> Body
    {
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {..Default::default()};
        
        for _ in 0..rand::gen_range(Settings::MIN_CIRCLES, Settings::MAX_CIRCLES)
        {
            let x = rand::gen_range(-x_bound / 2.0, x_bound / 2.0);
            let y = rand::gen_range(-y_bound / 2.0, y_bound / 2.0);

            //was to make sure Circles were overlapping but not needed
            // loop    
            // {
            //     x = rand::gen_range(-x_bound / 2.0, x_bound / 2.0);
            //     y = rand::gen_range(-y_bound / 2.0, y_bound / 2.0);
            //     if body.circles.iter().any(|c| (c.x - x).powi(2) + (c.y - y).powi(2) < (c.r + c.r).powi(2))
            //     {
            //         continue;
            //     }
            //     break;
            // }
            
            body.circles.push(Circle::new_random(Point {x, y})); 
        }
        
        // make sure every circle is connected
        let mut connected : Vec<usize> = Vec::new();
        connected.push(rand::gen_range(0, body.circles.len()));
        for i in 0..body.circles.len()
        {
            if connected.iter().any(|x| *x == i)
            {
                continue;
            }
            body.muscles.push( Muscle::new_random( i, connected[rand::gen_range(0, connected.len())] ) );
            connected.push(i);
        }

        body.start_avg_x = body.circles.iter().map(|c| c.pos.x).sum::<f32>() / body.circles.len() as f32;
        //add a couple more random muscles between random circles but no repeats
        let max_connections = factorial(body.circles.len()) / (2 * factorial(body.circles.len()-2));
        if body.muscles.len() >= max_connections // dont continue if no more possible muscle additions are possible
        {   
            return body;
        }
        
        for _ in 0..(rand_biased(0, (max_connections - body.muscles.len()) as i32, 2.0) as usize)
        {
            //might be too rng and take too long so might have to change VV
            let from = rand::gen_range(0, body.circles.len());
            let to = rand::gen_range(0, body.circles.len());

            if from == to || body.muscles.iter().any(|m| (m.from == from && m.to == to) || (m.from == to && m.to == from))
            {
                continue;
            }

            body.muscles.push(Muscle::new_random(from, to));
        }
        body
    }
    pub fn set_start_avg(&mut self)
    {
        self.start_avg_x = self.circles.iter().map(|c| c.pos.x).sum::<f32>() / self.circles.len() as f32;
    }

    pub fn draw(&mut self)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(self.pos));
    }
    pub fn update(&mut self, time : f32)
    {
        self.muscles.iter_mut().for_each(|m| m.update(time, &mut self.circles));
        self.circles.iter_mut().for_each(|c| c.update(self.pos));
    }
    pub fn get_average_distance(&self) -> f32
    {
        self.circles.iter().map(|c| c.pos.x).sum::<f32>() / self.circles.len() as f32 - self.start_avg_x
    }
    pub fn simulate(&mut self) -> f32
    {
        let mut time = 0.0;
        loop
        {
            
            self.update(time);
            if self.get_average_distance().is_nan()
            {
                return 0.0;
            }
            if self.circles.iter().all(|c| c.on_floor)
            {
                return self.get_average_distance();
            }
            if time > Settings::TIME_GIVEN
            {
                return self.get_average_distance();
            }
            time += 1.0/60.0;
        }
        
    }
    pub fn flip(&mut self)
    {
        self.circles.iter_mut().for_each(|c| c.pos.x *= -1.0);
        self.set_start_avg();
    }
    
    pub fn mutate(&mut self)
    {
        if  rand::gen_range(0, 3) == 0
        {
            self.major_change();
        }
        else 
        {
            self.minor_change();
        }
        self.set_start_avg();
    }
    pub fn major_change(&mut self)
    {
        match rand::gen_range(0, 4)
        {
            0 => self.add_circle(),
            1 => self.remove_circle(),
            2 => self.add_muscle(),
            3 => self.remove_muscle(),
            _ => println!("ERROR: major_change() in body.rs")
        }
            
    }
    pub fn add_circle(&mut self)
    {
        let mut can_be_connected = (0..self.circles.len()).collect::<Vec<usize>>();
        self.circles.push(Circle::new_random(Point {x : rand::gen_range(-Settings::X_BOUND, Settings::X_BOUND), y : rand::gen_range(-Settings::Y_BOUND, Settings::Y_BOUND)}));
        for _ in 0..rand_biased(1, can_be_connected.len() as i32, 2.0)
        {
            let can_be_connected_index = rand::gen_range(0, can_be_connected.len());
            let circles_index = can_be_connected[rand::gen_range(0, can_be_connected_index)];
            can_be_connected.remove(can_be_connected_index);
            self.muscles.push( Muscle::new_random(self.circles.len() - 1, circles_index));
        }
    }
    pub fn remove_circle(&mut self)
    {
        if self.circles.len() == 0
        {
            return;
        }
        let index = rand::gen_range(0, self.circles.len() - 1);
        for i in 0..self.muscles.len()
        {
            if self.muscles[i].from > index
            {
                self.muscles[i].from -= 1;
            }
            if self.muscles[i].to > index
            {
                self.muscles[i].to -= 1;
            }
        }
        self.muscles.retain(|x| x.from != index && x.to != index);
        self.circles.remove(index);
    }
    pub fn add_muscle(&mut self)
    {
        if self.circles.len() < 2
        {
            return;
        }
        let from = rand::gen_range(0, self.circles.len());
        let to = rand::gen_range(0, self.circles.len());

        if from == to || self.muscles.iter().any(|m| (m.from == from && m.to == to) || (m.from == to && m.to == from))
        {
            return;
        }

        self.muscles.push(Muscle::new_random(from, to));
    }
    pub fn remove_muscle(&mut self)
    {
        if self.muscles.len() == 0
        {
            return;
        }
        self.muscles.remove(rand::gen_range(0, self.muscles.len()));
    }
    pub fn minor_change(&mut self)
    {
        if rand::gen_range(0, 2) == 0
        {
            //mutate muscle
            if self.muscles.len() == 0
            {
                return;
            }
            let index = rand::gen_range(0, self.muscles.len());
            self.muscles[index].mutate();
        }
        else
        {
            //mutate circle
            if self.circles.len() == 0
            {
                return;
            }
            let index = rand::gen_range(0, self.circles.len());
            self.circles[index].mutate();
        }
    }
    pub fn link_loners(&mut self)
    {
    }

    pub fn set_alpha(&mut self, alpha : f32)
    {
        self.circles.iter_mut().for_each(|c| c.color.a = alpha);
        self.muscles.iter_mut().for_each(|m| m.color.a = alpha);
    }
}

fn factorial(n : usize) -> usize
{
    (1..=n).product()
}

fn rand_biased(min : i32, max : i32, p : f32) -> i32
{
    // the higher p is the more biased towards min it is https://gamedev.stackexchange.com/questions/116832/random-number-in-a-range-biased-toward-the-low-end-of-the-range/116875#116875
    (min as f32 + (max - min) as f32 * (rand::gen_range(-1.0, 1.0) as f32).powf(p).abs()).round() as i32
}

