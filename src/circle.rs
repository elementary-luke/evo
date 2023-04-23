use crate::point::*;
use crate::force::*;
use macroquad::color::*;
use macroquad::window::*;
use macroquad::qrand as rand;
use macroquad::prelude::draw_circle;

pub struct Circle
{
    pub pos : Point,
    pub r : f32,
    pub color : Color,
    pub friction : f32,
    pub velocity : Point,
    pub acceleration : Point,
    pub forces : Vec<Force>,
}


impl Circle
{
    pub fn update(&mut self, body_pos : Point)
    {
        let mut impulse : Point = Point {x: 0.0, y: 0.0};
        for i in 0..self.forces.len()
        {
            match self.forces[i].from
            {
                ForceTypes::Gravity => 
                {
                    self.acceleration.y += Point::grav;
                },
                ForceTypes::Muscle => 
                {
                    impulse += self.forces[i].strength;
                },
            }
        }
        self.velocity = impulse;
        self.forces.clear();
        //self.velocity += self.acceleration;
        
        if self.velocity.y >= 0.0 && body_pos.y + self.pos.y + self.r + self.velocity.y >= screen_height()-40.0
        {
            self.pos.y = screen_height()-40.0;
            self.velocity.y = 0.0;
        }
        //println!("{:?}", self.velocity);
        self.pos += self.velocity;
        
    }
    pub fn draw(&mut self, body_pos : Point)
    {
        draw_circle((self.pos + body_pos).x, (self.pos + body_pos).y, self.r, self.color)
    }
    pub fn new_random(pos : Point) -> Circle
    {
        let fr = rand::gen_range(0.0, 1.0);
        Circle {
            pos,
            r: 5.0, 
            color: Color { r: fr, g: fr, b: fr, a : 1.0}, 
            friction: fr,
            velocity : Point {x : 0.0, y : 0.0},
            acceleration : Point {x : 0.0, y : Point::grav},
            forces : vec![],
        }
    }
}
