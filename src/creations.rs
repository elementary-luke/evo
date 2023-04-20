// use normal_rand::Rng as Rng;
use std::ops::Add;
use macroquad::color::*;
use macroquad::window::*;
use macroquad::shapes::*;
use macroquad::qrand as rand;
use std::cmp::min;


#[derive(Clone, Copy)]
#[derive(Debug)]

pub struct Point
{
    pub x : f32,
    pub y : f32,
}

impl Add for Point
{
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Point 
{
    const grav : f32 = 0.1;
    fn magnitude(&self) -> f32
    {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

pub struct Dot
{
    pub pos : Point,
    pub r : f32,
    pub color : Color,
    pub friction : f32,
    pub velocity : Point,
    pub acceleration : Point,
}


impl Dot
{
    pub fn update(&mut self, body_pos : Point)
    {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;

        
        
        if self.velocity.y >= 0.0 && body_pos.y + self.pos.y + self.r + self.velocity.y >= screen_height()-40.0
        {
            self.velocity.y = 0.0;
        }

        self.acceleration = Point {x : 0.0, y : Point::grav};
        //println!("{:?}", self.velocity);
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
        
    }
    pub fn draw(&mut self, body_pos : Point)
    {
        draw_circle((self.pos + body_pos).x, (self.pos + body_pos).y, self.r, self.color)
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
    pub fn update(&mut self)
    {
        
    }
    pub fn draw(&mut self, body_pos : Point, from : Point, to : Point)
    {
        draw_line(body_pos.x + from.x, body_pos.y + from.y, body_pos.x + to.x, body_pos.y + to.y, 3.0, RED);
    }
}

pub struct Body
{
    pub pos : Point,
    pub circles : Vec<Dot>,
    pub muscles : Vec<Muscle>,
}

impl Body
{
    pub fn new() -> Body // new empty body
    {
        let circles : Vec<Dot> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body : Body = Body {pos : Point {x : 0.0, y : 0.0}, circles, muscles};
        body
    }
    pub fn new_random(x_bound : f32, y_bound : f32) -> Body
    {
        rand::srand(macroquad::miniquad::date::now() as u64);
        let circles : Vec<Dot> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {pos : Point {x : 0.0, y : 0.0}, circles, muscles};
        
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
            
            body.circles.push(Dot {
                pos: Point {x, y},
                r: 5.0, 
                color: Color { r: fr, g: fr, b: fr, a : 1.0}, 
                friction: fr,
                velocity : Point {x : 0.0, y : 0.0},
                acceleration : Point {x : 0.0, y : Point::grav},
            }); 
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
        let max_connections = factorial(body.circles.len()) / (2 * factorial(body.circles.len()-2));
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
        self.muscles.iter_mut().for_each(|m| m.draw(self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(self.pos));
    }
    pub fn update(&mut self)
    {
        self.muscles.iter_mut().for_each(|m| m.update());
        self.circles.iter_mut().for_each(|c| c.update(self.pos));
    }
}

fn factorial(n : usize) -> usize
{
    (1..=n).product()
}