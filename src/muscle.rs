use std::cmp::max;
use std::cmp::min;

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
        // TODO CHECK IF WORKS WHEN EXTENDING BUT LENGTH IS BIGGER THAN EXTENDED LENGTH
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
        
        let target_len : f32;
        if self.contracting.0
        {
            target_len = self.contracted_len;
        }
        else
        {
            target_len = self.extended_len;    
        }

        //println!("{}", current_len);
        let mut accel_from = Point { x: 0.0, y: 0.0 };
        let mut accel_to = Point { x: 0.0, y: 0.0 };

        let nfrom = & circles[self.from];
        let nto = & circles[self.to];
        let current_len = (circles[self.to].pos - circles[self.from].pos).magnitude();
        let angle = (nfrom.pos.y-nto.pos.y).atan2(nfrom.pos.x-nto.pos.x);
        let force = f32::min(f32::max(1.0-(current_len / target_len),-0.4),0.4);
        accel_from.x += (angle).cos() * force * self.strength / nfrom.r;
        accel_from.y += (angle).sin() * force * self.strength / nfrom.r;
        accel_to.x -= (angle).cos() * force * self.strength / nto.r;
        accel_to.y -= (angle).sin() * force * self.strength / nto.r;

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
            contracted_time : rand::gen_range(0.5, 1.5),
            extended_time : rand::gen_range(0.5, 1.5),
            strength : rand::gen_range(80.0, 160.0), 
            contracting : ([true, false][rand::gen_range(0, 1)], 0.0),
        }
    }
    pub fn mutate(&mut self)
    {
        for _ in 0..2
        {
            match rand::gen_range(0, 5)
            {
                0 => self.contracted_len += rand::gen_range(-20.0, 20.0),
                1 => self.extended_len += rand::gen_range(-20.0, 20.0),
                2 => self.contracted_time += rand::gen_range(-0.2, 0.2),
                3 => self.extended_time += rand::gen_range(-0.2, 0.2),
                4 => {
                    self.strength += rand::gen_range(-10.0, 10.0);
                },
                5 => self.contracting.0 = !self.contracting.0,
                _ => (),
            }
        }
    }
}