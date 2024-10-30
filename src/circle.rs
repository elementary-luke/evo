use crate::point::*;
use crate::force::*;
use crate::settings::*;
use macroquad::color::*;
use macroquad::rand;
use macroquad::prelude::draw_circle;

#[derive(Clone, Debug)]
pub struct Circle
{
    pub pos : Point,
    pub r : f32,
    pub color : Color,
    pub slip : f32,
    pub velocity : Point,
    pub forces : Vec<Force>,
    pub on_floor : bool,
}


impl Circle
{
    pub fn update(&mut self, body_pos : Point, settings : &Settings)
    {
        let mut acceleration : Point = Point {x: 0.0, y: 0.0};
        for i in 0..self.forces.len()
        {
            match self.forces[i].from
            {
                ForceTypes::Gravity | ForceTypes::Muscle=> 
                {
                    acceleration += self.forces[i].strength;
                }
            }
        }

        // add velocity based on all forces
        self.velocity *= Point {x : settings.drag, y : settings.drag};
        self.velocity += acceleration;
        self.forces.clear();
        self.forces.push(Force {
            from : ForceTypes::Gravity,
            strength : Point {x: 0.0, y: settings.grav},
        });

        //collision with terrain
        for i in 0..settings.terrain.len()
        {
            let terrain = settings.terrain[i];
            let abs_x = body_pos.x + self.pos.x;
            let abs_y = body_pos.y + self.pos.y;

            //horizontal collision
            if abs_y + self.r >= terrain.1 && abs_y - self.r <= terrain.3 // if y is in the range of the terrain
            {
                if abs_x <= terrain.0 // if circle is to the left of the terrain
                {
                    if abs_x + self.r + self.velocity.x >= terrain.0 // if circle will be in the terrain
                    {
                        self.pos.x += terrain.0 - (abs_x + self.r);
                        self.velocity.x = 0.0;
                        self.velocity.y *= self.slip;
                    }
                }
                if abs_x >= terrain.2 // if circle is to the right of the terrain
                {
                    if abs_x - self.r + self.velocity.x <= terrain.2 // if circle will be in the terrain
                    {
                        self.pos.x += terrain.2 - (abs_x - self.r);
                        self.velocity.x = 0.0;
                        self.velocity.y *= self.slip;
                    }
                }
                
            }
            //vertical collision
            if abs_x + self.r >= terrain.0 && abs_x - self.r <= terrain.2 // if x is in the range of the terrain
            {
                if abs_y <= terrain.1 // if circle is above
                {
                    if abs_y + self.r + self.velocity.y >= terrain.1 // if circle will be in the terrain
                    {
                        self.pos.y += terrain.1 - (abs_y + self.r);
                        self.velocity.y = 0.0;
                        self.velocity.x *= self.slip;
                    }
                }
                if abs_y >= terrain.3 // if circle below
                {
                    if abs_y - self.r + self.velocity.y <= terrain.3 // if circle will be in the terrain
                    {
                        self.pos.y += terrain.3 - (abs_y - self.r);
                        self.velocity.y = 0.0;
                        self.velocity.x *= self.slip;
                    }
                }
                
            }
        }
        
        //floor collision
        if self.velocity.y >= 0.0 && body_pos.y + self.pos.y + self.r + self.velocity.y >= settings.floor_y
        {
            self.pos.y += settings.floor_y - (body_pos.y + self.pos.y + self.r);
            self.velocity.y = 0.0;
            self.on_floor = true;
        }
        if self.on_floor && body_pos.y + self.pos.y < settings.floor_y - self.r
        {
            self.on_floor = false;
        }
        if self.on_floor
        {
            self.velocity.x *= self.slip;
        }

        self.pos += self.velocity;
    }

    pub fn draw(&mut self, body_pos : Point)
    {
        draw_circle((self.pos + body_pos).x, (self.pos + body_pos).y, self.r, self.color)
    }

    pub fn new_random(pos : Point, settings : &Settings) -> Circle
    {
        let slip = rand::gen_range(settings.slip_min, settings.slip_max);
        Circle {
            pos,
            r: 5.0, 
            color: Color { r: 0.9 * slip, g:slip, b: slip, a : 1.0}, 
            slip,
            velocity : Point {x : 0.0, y : 0.0},
            forces : vec![],
            on_floor : false,
        }
    }

    pub fn mutate(&mut self, settings : &Settings)
    {
        for _ in 0..2// change to how many mutations you want
        {
            match rand::gen_range(0, 3)
            {
                0 => {
                    self.pos.x += rand::gen_range(-50.0, 50.0);
                    self.pos.x = self.pos.x.clamp(-settings.x_bound / 2.0, settings.x_bound / 2.0);
                },
                1 => {
                    self.pos.y += rand::gen_range(-50.0, 50.0);
                    self.pos.y = self.pos.y.clamp(-settings.y_bound / 2.0, settings.y_bound / 2.0);
                },
                2 => {
                    self.slip += rand::gen_range(-0.2, 0.2);
                    self.slip = self.slip.clamp(0.0, 1.0);
                },
                _ => (),
            }
        }
        //make sure properties don't go out of limits defined at the start
        self.pos.x = self.pos.x.clamp(-settings.x_bound / 2.0, settings.x_bound / 2.0);
        self.pos.y = self.pos.y.clamp(-settings.y_bound / 2.0, settings.y_bound / 2.0);
        self.slip = self.slip.clamp(settings.cslip_min, settings.cslip_max);
    }
}
