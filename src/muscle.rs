use crate::circle::*;
use crate::point::*;
use crate::force::*;
use macroquad::qrand as rand;
use macroquad::color::*;
use macroquad::prelude::draw_line;
pub struct Muscle
{
    pub from : usize, // index of circles list
    pub to : usize,
    pub contracted_len : f32,
    pub extended_len : f32,
    pub contracted_time : f32,
    pub extended_time : f32,
    pub strength : f32,
    pub contracting : (bool, f32), // bool is if it is contracting, i32 is the time it started contracting
}

impl Muscle
{
    pub fn update(&mut self, time : f32, circles : &mut Vec<Circle>)
    {
        if self.contracting.0 && time - self.contracting.1 >= self.contracted_time
        {
            self.contracting = (false, time);
            println!("{:?}, {:?}", circles[0].friction, circles[0].pos.x);
            println!("{:?}, {:?}", circles[1].friction, circles[1].pos.x);
            return;
        }
        else if !self.contracting.0 && time - self.contracting.1 >= self.extended_time
        {
            self.contracting = (true, time);
            println!("{:?}, {:?}", circles[0].friction, circles[0].pos.x);
            println!("{:?}, {:?} \n", circles[1].friction, circles[1].pos.x);
            return;
        }
        
        

        if self.contracting.0
        {
            let mag = ((circles[self.to].pos - circles[self.from].pos).magnitude() - self.contracted_len) / 2.0;
            
            let mut accel_from = Point {
                    x : circles[self.to].pos.x - circles[self.from].pos.x,
                    y : circles[self.to].pos.y - circles[self.from].pos.y,
                };

            accel_from = accel_from * Point { 
                x: mag / (circles[self.to].pos - circles[self.from].pos).magnitude(), 
                y: mag / (circles[self.to].pos - circles[self.from].pos).magnitude()
            };
            accel_from *= Point {x: self.strength, y: self.strength};
            let mut accel_to = accel_from.clone() * Point { x: -1.0, y: -1.0 };
            
            // remake!
            let mut temp = accel_to.clone();
            if circles[self.from].on_floor
            {
                accel_to.x -=  accel_from.x - accel_from.x * (1.0 - circles[self.from].friction);
                accel_from.x *= 1.0 - circles[self.from].friction;
            }
            else if circles[self.to].on_floor
            {
                accel_from.x -=  temp.x - temp.x * (1.0 - circles[self.to].friction);
                temp.x *= 1.0 - circles[self.to].friction;
            }
                

            circles[self.from].forces.push(Force {
                from : ForceTypes::Muscle,
                strength : accel_from,
            });
            
            circles[self.to].forces.push(Force {
                from : ForceTypes::Muscle,
                strength : accel_to
            });
        } 
        else 
        {
            //TODO ADD extending OFF OF THE FLOOR
            let mag = (self.extended_len - (circles[self.to].pos - circles[self.from].pos).magnitude()) / 2.0;
            
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

            let mut temp = accel_to.clone();
            if circles[self.from].on_floor
            {
                accel_to.x -=  accel_from.x - accel_from.x * (1.0 - circles[self.from].friction);
                accel_from.x *= (1.0 - circles[self.from].friction);
            }
            else if circles[self.to].on_floor
            {
                accel_from.x -=  temp.x - temp.x * (1.0 - circles[self.to].friction);
                temp.x *= 1.0 - circles[self.to].friction;
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
    }

    pub fn draw(&mut self, body_pos : Point, from : Point, to : Point)
    {
        draw_line(body_pos.x + from.x, body_pos.y + from.y, body_pos.x + to.x, body_pos.y + to.y, 3.0, Color {r: 1.0 * self.strength, g: 0.5 * self.strength, b: 0.0, a: self.strength});
    }
    pub fn new_random(from : usize, to : usize, ) -> Muscle
    {   
        let contracted_len = rand::gen_range(0.0, 100.0);
        let extended_len = rand::gen_range(contracted_len, contracted_len + 200.0);
        Muscle {
            from, 
            to, 
            contracted_len, 
            extended_len, 
            contracted_time : rand::gen_range(0.1, 1.0),
            extended_time : rand::gen_range(0.1, 1.0),
            strength : rand::gen_range(0.0, 1.0), 
            contracting : ([true, false][rand::gen_range(0, 1)], 0.0),
        }
    }
}