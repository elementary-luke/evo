// use normal_rand::Rng as Rng;
use macroquad::color::*;
use macroquad::window::*;
use macroquad::shapes::*;
use macroquad::qrand as rand;
use std::cmp::min;

pub struct Dot
{
    pub x : f32,
    pub y : f32,
    pub r : f32,
    pub color : Color,
    pub friction : f32,
}

impl Dot
{
    pub fn draw(&mut self, base_x : f32, base_y : f32)
    {
        draw_circle(self.x + base_x, self.y + base_y, self.r, self.color)
    }
}

pub struct Muscle
{
    pub from : usize, // index of circles
    pub to : usize,
    pub extended_len : f32,
    pub contracted_len : f32,
    pub strength : f32,
}

impl Muscle
{
    pub fn draw(&mut self, base_x: f32, base_y : f32, from_x : f32, to_x : f32, from_y : f32, to_y : f32)
    {
        draw_line(from_x + base_x, from_y + base_y, to_x + base_x, to_y + base_y, 3.0, RED);
    }
}

pub struct Body
{
    pub x : f32,
    pub y : f32,
    pub circles : Vec<Dot>,
    pub muscles : Vec<Muscle>,
}

impl Body
{
    pub fn new() -> Body // new empty body
    {
        let circles : Vec<Dot> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body : Body = Body {x: 0.0, y: 0.0, circles, muscles};
        body
    }
    pub fn new_random(x_bound : f32, y_bound : f32) -> Body
    {
        rand::srand(macroquad::miniquad::date::now() as u64);
        let circles : Vec<Dot> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {x: 0.0, y: 0.0, circles, muscles};
        
        for _ in 0..rand::gen_range(2, 10)
        {
            let fr = rand::gen_range(0.0, 1.0);
            let x = rand::gen_range(-x_bound / 2.0, x_bound / 2.0);
            let y = rand::gen_range(-y_bound / 2.0, y_bound / 2.0);

            //was to make sure dots were overlapping but not needed
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
            
            body.circles.push(Dot {x,
                y, 
                r: 5.0, 
                color: Color { r: fr, g: fr, b: fr, a : 1.0}, 
                friction: fr}); 
        }
        
        // make sure every circle is connected
        let mut connected : Vec<usize> = Vec::new();
        connected.push(rand::gen_range(0, body.circles.len()));
        for i in 0..body.circles.len()
        {
            if connected.contains(&i)
            {
                continue;
            }
            body.muscles.push(Muscle {from: i, 
                to: connected[rand::gen_range(0, connected.len())], 
                extended_len: rand::gen_range(0.0, 100.0), 
                contracted_len: rand::gen_range(0.0, 100.0), 
                strength: rand::gen_range(0.0, 1.0)});
            connected.push(i);
        }

        //add a couple more random muscles between random circles but no repeats
        let max_connections = body.circles.len() / 2 * (1..(body.circles.len()-2)).product::<usize>();
        if body.muscles.len() >= max_connections // dont continue if no more possible muscle additions are possible
        {
            return body;
        }
        for _ in 0..rand::gen_range(0, min(max_connections - body.muscles.len(), 5)) 
        {
            //might be too rng and take too long so might have to change VV
            loop
            {
                let from = rand::gen_range(0, body.circles.len());
                let to = rand::gen_range(0, body.circles.len());

                if from == to || body.muscles.iter().any(|m| (m.from == from && m.to == to) || (m.from == to && m.to == from))
                {
                    continue;
                }

                body.muscles.push(Muscle {from, 
                    to, 
                    extended_len: rand::gen_range(0.0, 100.0), 
                    contracted_len: rand::gen_range(0.0, 100.0), 
                    strength: rand::gen_range(0.0, 1.0)});
                break;
            }
        }
        body
    }

    pub fn draw(&mut self)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(self.x, self.y, self.circles[m.from].x, self.circles[m.to].x, self.circles[m.from].y, self.circles[m.to].y));
        self.circles.iter_mut().for_each(|c| c.draw(self.x, self.y));
    }
}


// pub enum ObjTypes
// {
//     Dot(Dot),
//     Muscle(Muscle),
//     Body(Body),
// }