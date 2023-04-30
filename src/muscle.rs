use crate::circle::*;
use crate::point::*;
use crate::force::*;
use macroquad::qrand as rand;
use macroquad::color::*;
use macroquad::prelude::draw_line;

#[derive(Clone, Debug)]
pub struct Muscle
{
    pub from : usize, // index of circles list
    pub to : usize,
    pub contracted_len : f32,
    pub extended_len : f32,
    pub contracted_time : f32,
    pub extended_time : f32,
    pub strength : f32,
    pub contracting : (bool, f32), // bool is if it is contracting, f32 is the time it started contracting
}


impl Muscle
{
    pub fn update(&mut self, time : f32, circles : &mut Vec<Circle>)
    {
        if self.contracting.0 && time - self.contracting.1 >= self.contracted_time
        {
            self.contracting = (false, time);
            return;
        }
        else if !self.contracting.0 && time - self.contracting.1 >= self.extended_time
        {
            self.contracting = (true, time);
            return;
        }
        
        

        let mag : f32;
        if self.contracting.0
        {
            mag = (self.contracted_len - (circles[self.to].pos - circles[self.from].pos).magnitude()) / 2.0;
        }
        else 
        {
            mag = (self.extended_len - (circles[self.to].pos - circles[self.from].pos).magnitude()) / 2.0;
        }
        
        let mut accel_from = Point {
                x : circles[self.from].pos.x - circles[self.to].pos.x,
                y : circles[self.from].pos.y - circles[self.to].pos.y,
            };

        accel_from = accel_from * Point { 
            x: mag / (circles[self.to].pos - circles[self.from].pos).magnitude(), 
            y: mag / (circles[self.to].pos - circles[self.from].pos).magnitude()
        };

        accel_from *= Point {x: self.strength, y: self.strength};
        let mut accel_to = accel_from.clone() * Point {x: -1.0, y: -1.0};


        if circles[self.from].on_floor && circles[self.to].on_floor
        {
            let total_slip = (circles[self.from].slip + circles[self.to].slip);
            let x_movement = (accel_to.x.abs() + accel_from.x.abs()) * total_slip / 2.0;
            if total_slip > 0.0
            {
                accel_from.x = accel_from.x.signum() * circles[self.from].slip / total_slip * x_movement;
                accel_to.x = accel_to.x.signum() *  circles[self.to].slip / total_slip * x_movement;
            }
            else 
            {
                accel_from.x = 0.0;
                accel_to.x = 0.0;
            }
        }
        else if circles[self.from].on_floor != circles[self.to].on_floor
        {
            if circles[self.from].on_floor
            {
                accel_to.x -= accel_from.x - (accel_from.x * circles[self.from].slip).abs();
                accel_from.x *= circles[self.from].slip;
                if accel_from.y < 0.0
                {
                    accel_to.y -= accel_from.y;
                }
            }
            else
            {
                accel_from.x -= accel_to.x - (accel_to.x * circles[self.to].slip).abs();
                accel_to.x *= circles[self.to].slip;
                if accel_to.y < 0.0
                {
                    accel_from.y -= accel_to.y;
                }
            }
        }

        circles[self.from].forces.push(Force {
            from : ForceTypes::Muscle,
            strength : accel_from,
        });
        
        circles[self.to].forces.push(Force {
            from : ForceTypes::Muscle,
            strength : accel_to,
        });
    }

    pub fn draw(&mut self, body_pos : Point, from : Point, to : Point)
    {
        draw_line(body_pos.x + from.x, body_pos.y + from.y, body_pos.x + to.x, body_pos.y + to.y, 3.0, Color {r: 1.0 * self.strength, g: 0.5 * self.strength, b: 0.0, a: self.strength});
    }
    pub fn new_random(from : usize, to : usize, ) -> Muscle
    {   
        let contracted_len = rand::gen_range(50.0, 100.0);
        let extended_len = rand::gen_range(contracted_len, contracted_len + 100.0);
        Muscle {
            from, 
            to, 
            contracted_len, 
            extended_len, 
            contracted_time : rand::gen_range(0.4, 1.0),
            extended_time : rand::gen_range(1.0, 1.5),
            strength : rand::gen_range(0.6, 0.8), 
            contracting : ([true, false][rand::gen_range(0, 1)], 0.0),
        }
    }
    pub fn mutate(&mut self)
    {
        for i in 0..2
        {
            match rand::gen_range(0, 5)
            {
                0 => self.contracted_len += rand::gen_range(-10.0, 10.0),
                1 => self.extended_len += rand::gen_range(-10.0, 10.0),
                2 => self.contracted_time += rand::gen_range(-0.1, 0.1),
                3 => self.extended_time += rand::gen_range(-0.1, 0.1),
                4 => {
                    self.strength += rand::gen_range(-0.1, 0.1);
                    self.strength = self.strength.clamp(0.0, 1.0);
                },
                5 => self.contracting.0 = !self.contracting.0,
                _ => (),
            }
        }
    }
}