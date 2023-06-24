use std::cmp::max;
use std::cmp::min;

use crate::circle::*;
use crate::point::*;
use crate::force::*;
use crate::settings;
use crate::settings::*;
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
    pub color : Color,
}


impl Muscle
{
    pub fn update(&mut self, time : f32, circles : &mut Vec<Circle>, settings : &Settings, energy_used : &mut f32)
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

        //put forces on circles
        let nfrom = & circles[self.from];
        let nto = & circles[self.to];
        let current_len = (circles[self.to].pos - circles[self.from].pos).magnitude();
        let angle = (nfrom.pos.y-nto.pos.y).atan2(nfrom.pos.x-nto.pos.x);
        let force = f32::min(f32::max(1.0-(current_len / target_len),-0.2), 0.2);
        accel_from.x += (angle).cos() * force * self.strength / nfrom.r;
        accel_from.y += (angle).sin() * force * self.strength / nfrom.r;
        accel_to.x -= (angle).cos() * force * self.strength / nto.r;
        accel_to.y -= (angle).sin() * force * self.strength / nto.r;
        *energy_used += (force * self.strength / nfrom.r).abs() + (force * self.strength / nto.r).abs();

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
        draw_line(body_pos.x + from.x, body_pos.y + from.y, body_pos.x + to.x, body_pos.y + to.y, 3.0, self.color);
    }
    pub fn new_random(from : usize, to : usize, settings : &Settings) -> Muscle
    {   
        let contracted_len = rand::gen_range(settings.contracted_len_min, settings.contracted_len_max);
        let extended_len = rand::gen_range(settings.extended_len_min, settings.extended_len_max);
        let mut m = Muscle {
            from, 
            to, 
            contracted_len, 
            extended_len, 
            contracted_time : rand::gen_range(settings.contracted_time_min, settings.contracted_time_max),
            extended_time : rand::gen_range(settings.extended_time_min, settings.extended_time_max),
            strength : rand::gen_range(settings.strength_min, settings.strength_max), 
            contracting : ([true, false][rand::gen_range(0, 1)], 0.0),
            color : Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0},
        };
        m.color = Color {r: 1.0 * (m.strength - settings.strength_min) / (settings.strength_max - settings.strength_min), 
            g: 0.5 *  (m.strength - settings.strength_min) / (settings.strength_max - settings.strength_min), 
            b: 0.0  * (m.strength - settings.strength_min) / (settings.strength_max - settings.strength_min), 
            a: 1.0
        };

        return m;
    }
    pub fn mutate(&mut self, settings : &Settings)
    {
        for _ in 0..2
        {
            match rand::gen_range(0, 5)
            {
                0 => self.contracted_len += rand::gen_range(-5.0, 5.0),
                1 => self.extended_len += rand::gen_range(-5.0, 5.0),
                2 => self.contracted_time += rand::gen_range(-0.2, 0.2),
                3 => self.extended_time += rand::gen_range(-0.2, 0.2),
                4 => self.strength += rand::gen_range(-10.0, 10.0),
                5 => self.contracting.0 = !self.contracting.0,
                _ => (),
            }
            // //make sure contraints are followed
            self.contracted_len =self.contracted_len.clamp(settings.ccontracted_len_min, settings.ccontracted_len_max); 
            self.extended_len = self.extended_len.clamp(settings.cextended_len_min, settings.cextended_len_max);
            self.contracted_time = self.contracted_time.clamp(settings.ccontracted_time_min, settings.ccontracted_time_max);
            self.extended_time = self.extended_time.clamp(settings.cextended_time_min, settings.cextended_time_max);
            self.strength = self.strength.clamp(settings.cstrength_min, settings.cstrength_max);
        }
    }
}