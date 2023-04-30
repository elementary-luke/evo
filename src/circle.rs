use crate::point::*;
use crate::force::*;
use crate::settings::Settings;
use macroquad::color::*;
use macroquad::window::*;
use macroquad::qrand as rand;
use macroquad::prelude::draw_circle;

#[derive(Clone, Debug)]
pub struct Circle
{
    pub pos : Point,
    pub r : f32,
    pub color : Color,
    pub slip : f32,
    pub velocity : Point,
    pub acceleration : Point,
    pub forces : Vec<Force>,
    pub on_floor : bool,
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
                    self.acceleration.y += Settings::GRAV;
                },
                ForceTypes::Muscle => 
                {
                    impulse += self.forces[i].strength;
                },
            }
        }
        self.velocity = impulse;
        self.forces.clear();
        self.forces.push(Force {
            from : ForceTypes::Gravity,
            strength : Point {x: 0.0, y: Settings::GRAV},
        });

        //self.velocity += self.acceleration;
        self.velocity += Point {x:0.0, y: Settings::GRAV};
        
        if self.velocity.y >= 0.0 && body_pos.y + self.pos.y + self.r + self.velocity.y >= Settings::FLOOR_Y
        {
            //TODO move the circle to the floor
            self.pos.y += Settings::FLOOR_Y - (body_pos.y + self.pos.y + self.r);
            self.velocity.y = 0.0;
            self.acceleration.y = 0.0;
            self.on_floor = true;
        }
        if self.on_floor && body_pos.y + self.pos.y < Settings::FLOOR_Y - self.r
        {
            self.on_floor = false;
        }

        self.pos += self.velocity;

        
    }
    pub fn draw(&mut self, body_pos : Point)
    {
        draw_circle((self.pos + body_pos).x, (self.pos + body_pos).y, self.r, self.color)
    }
    pub fn new_random(pos : Point) -> Circle
    {
        let slip = rand::gen_range(Settings::SLIP_MIN, Settings::SLIP_MAX);
        Circle {
            pos,
            r: 5.0, 
            color: Color { r: slip, g: slip, b: slip, a : 1.0}, 
            slip,
            velocity : Point {x : 0.0, y : 0.0},
            acceleration : Point {x : 0.0, y : 0.0},
            forces : vec![],
            on_floor : false,
        }
    }
    pub fn mutate(&mut self)
    {
        for _ in 0..2// change to how many mutations you want
        {
            match rand::gen_range(0, 3)
            {
                0 => {
                    self.pos.x += rand::gen_range(-50.0, 50.0);
                    self.pos.x = self.pos.x.clamp(-Settings::X_BOUND, Settings::X_BOUND);
                },
                1 => {
                    self.pos.y += rand::gen_range(-50.0, 50.0);
                    self.pos.y = self.pos.y.clamp(-Settings::Y_BOUND, Settings::Y_BOUND);
                },
                2 => {
                    self.slip += rand::gen_range(-0.2, 0.2);
                    self.slip = self.slip.clamp(0.0, 1.0);
                },
                _ => (),
            }
        }
    }
}
