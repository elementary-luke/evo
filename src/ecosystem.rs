use crate::body::*;
use crate::point::*;
use crate::settings::*;
use macroquad::color::*;
use macroquad::color_u8;
use macroquad::prelude::Rect;
use macroquad::window::*;
use macroquad::text::*;
use macroquad::qrand as rand;
use macroquad::math::vec2;
use macroquad::camera::*;
use macroquad::shapes::*;
use egui_macroquad::*;
use macroquad::input::*;

pub struct Ecosystem
{
    seed : u64,
    time : f32,
    bodies : Vec<Vec<Body>>,
    rbodies : Vec<Body>,
    gen : u32,
    add_gens_text : String,
    show : ShowTypes,
    custom_show : usize,
}

impl Ecosystem
{
    pub fn new() -> Ecosystem
    {
        Ecosystem {
            seed : 0,
            time : 0.0,
            bodies : Vec::new(),
            rbodies : Vec::new(),
            gen : 1,
            add_gens_text : "1".to_string(),
            show : ShowTypes::Best,
            custom_show : 1,
        }
    }

    pub fn initialise(&mut self)
    {
        if Settings::RANDOM_SEED
        {
            self.seed = macroquad::miniquad::date::now() as u64;
        }
        else
        {
            self.seed = Settings::SEED;
        }
        rand::srand(self.seed);

        self.bodies.push(Vec::new());
        self.create(Settings::POPULATION_SIZE);
        self.simulate();
        self.rbodies = vec![self.bodies[self.gen as usize -1][0].clone()];
    }

    pub fn create (&mut self, n : usize)
    {
        let bodies = &mut self.bodies[self.gen as usize - 1];
        for _ in 0..n
        {
            bodies.push(Body::new_random(Settings::X_BOUND, Settings::Y_BOUND));
        }
    }

    pub fn kill(&mut self)
    {
        let bodies = &mut self.bodies.last_mut().unwrap();
        let mut to_kill : Vec<usize> = Vec::new();
        for i in 0..(bodies.len() / 2)
        {
            let f : f32 = i as f32 / bodies.len() as f32;
            let r : f32 = (rand::gen_range(-1.0 as f32, 1.0 as f32).powf(3.0) + 1.0) / 2.0;
            if f > r
            {
                to_kill.push(i);
            }
            else 
            {
                to_kill.push(bodies.len() - 1 - i);
            }
        }
        
        to_kill.sort();
        to_kill.reverse();
        for i in to_kill
        {
            bodies.remove(i);
        }
    }

    pub fn repopulate(&mut self)
    {
        let bodies = &mut self.bodies.last_mut().unwrap();
        let mut new_bodies : Vec<Body> = Vec::new();
        for i in 0..bodies.len()
        {
            let mut new_body = bodies[i].clone();
            new_body.parent = Some((self.gen as usize, i));
            bodies[i].children.push((self.gen as usize + 1, new_bodies.len()));
            new_body.distance = None;
            new_body.mutate();
            new_bodies.push(new_body);
        }
        bodies.append(&mut new_bodies);
    }

    //TODO CHANGE IT SO TEMP IS CHANGED NOT THE ACTUAL BODY
    pub fn simulate(&mut self)
    {
        let bodies = &mut self.bodies.last_mut().unwrap();
        for i in 0..bodies.len()
        {
            if bodies[i].distance.is_some()
            {
                continue;
            }
            bodies[i].pos = Point {x : screen_width()/2.0, y : Settings::FLOOR_Y - Settings::Y_BOUND / 2.0};
            bodies[i].set_start_avg();

            let dist = bodies[i].clone().simulate();
    
            //if it went backwards flip it
            if dist < 0.0
            {
                //bodies[i].flip(); // TODO FIX THIS AS IF BEST IS FLIPPED DOESNT DO THE SAME THING
            }
            bodies[i].distance = Some(dist.abs());
        }
        bodies.retain(|b| !b.distance.unwrap().is_nan());
        bodies.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
    }

    pub fn run_view(&mut self)
    {
        // if is_key_pressed(KeyCode::R)
        // {
        //     self.time = 0.0;
        //     self.update_rbodies();
        //     self.rbodies.push(self.rbodies[0].clone());
        //     self.rbodies[0].flip();
        //     self.rbodies[0].pos = Point {x : screen_width()/2.0, y : Settings::FLOOR_Y - Settings::Y_BOUND / 2.0};
        //     self.rbodies[1].pos = Point {x : screen_width()/2.0, y : Settings::FLOOR_Y - Settings::Y_BOUND / 2.0};
        // }

        //draw signs
        for i in 0..(((self.bodies[self.gen as usize - 1][0].distance.unwrap() + screen_width() / 2.0) / 200.0).ceil() as usize + 1)
        {
            let y = Settings::FLOOR_Y - 250.0;
            let w = 80.0;
            let h = 40.0;
            let x = screen_width()/2.0 + i as f32 * 200.0 + self.bodies[self.gen as usize - 1][0].start_avg_x - w / 2.0;
            draw_rectangle(x, 
                y, 
                w, 
                h, 
                WHITE
            );
            draw_triangle(vec2(x + w / 2.0, y + h + 10.0), vec2(x + w / 2.0 + 10.0, y + h), vec2(x + w / 2.0 - 10.0, y + h), WHITE);
            draw_line(x + w / 2.0, Settings::FLOOR_Y, x + w / 2.0, y, 2.0, Color {r : 1.0, g : 1.0, b : 1.0, a : 0.4});
            draw_text(&(i * 200).to_string(), x + 10.0, y + 30.0, 40.0, BLACK);
        }
        for i in 0..(((self.bodies[self.gen as usize - 1][0].distance.unwrap() + screen_width() / 2.0) / 200.0).ceil() as usize + 1)
        {
            let y = Settings::FLOOR_Y - 250.0;
            let w = 80.0;
            let h = 40.0;
            let x = screen_width()/2.0 + i as f32 * -200.0 + self.bodies[self.gen as usize - 1][0].start_avg_x - w / 2.0;
            draw_rectangle(x, 
                y, 
                w, 
                h, 
                WHITE
            );
            draw_triangle(vec2(x + w / 2.0, y + h + 10.0), vec2(x + w / 2.0 + 10.0, y + h), vec2(x + w / 2.0 - 10.0, y + h), WHITE);
            draw_line(x + w / 2.0, Settings::FLOOR_Y, x + w / 2.0, y, 2.0, Color {r : 1.0, g : 1.0, b : 1.0, a : 0.4});
            draw_text(&(i * 200).to_string(), x + 10.0, y + 30.0, 40.0, BLACK);
        }

        //draw ground
        draw_rectangle(-screen_width(), 
            Settings::FLOOR_Y, 
            screen_width() * 2.0 + self.bodies[self.gen as usize - 1][0].distance.unwrap(), 
            screen_height() - Settings::FLOOR_Y, 
            color_u8!(192.0, 255.0, 133.0, 255.0)
        );

        self.time += 1.0/60.0;
        for i in 0..self.rbodies.len()
        {
            self.rbodies[i].update(self.time);
            self.rbodies[i].draw();
            if self.rbodies.len() > 1
            {
                self.rbodies[i].set_alpha((self.bodies[self.gen as usize - 1].len()-1-i) as f32/(self.bodies[self.gen as usize - 1].len() - 1) as f32);
            }
            else 
            {
                self.rbodies[0].set_alpha(0.8);
            }
        }
    }

    pub fn run_gui(&mut self)
    {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("dashboard")
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Seed ".to_string() + &self.seed.to_string());
                        if ui.button("📋").clicked()
                        {
                            ui.output_mut(|o| o.copied_text = self.seed.to_string());
                        }
                    });
                    ui.collapsing("Controls", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("View Gen".to_string());
                            if ui.add(egui::Slider::new(&mut self.gen, 1..=self.bodies.len() as u32)).changed()
                            {
                                self.update_rbodies();
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Do").clicked()
                            {
                                ui.output_mut(|o| o.copied_text = self.seed.to_string());
                                if self.add_gens_text.parse::<u32>().is_ok()
                                {
                                    for _ in 0..self.add_gens_text.parse::<u32>().unwrap() as usize
                                    {
                                        self.bodies.push(self.bodies[self.gen as usize - 1].clone());
                                        self.kill();
                                        self.repopulate();
                                        self.simulate();
                                        self.time = 0.0;
                                    }
                                    self.gen = self.bodies.len() as u32;
                                    self.update_rbodies();
                                }
                                else
                                {
                                    self.add_gens_text = "1".to_string();
                                }
                            }
                            ui.add(egui::TextEdit::singleline(&mut self.add_gens_text)
                                .desired_width(30.0)
                                .desired_rows(1)
                            );
                            ui.label("Gen(s) ".to_string());

                        });
                        let show_before = self.show.clone();
                        ui.horizontal(|ui| {
                            ui.label("View");
                            egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", self.show))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                                ui.selectable_value(&mut self.show, ShowTypes::Best, "Best");
                                ui.selectable_value(&mut self.show, ShowTypes::Median, "Median");
                                ui.selectable_value(&mut self.show, ShowTypes::Worst, "Worst");
                                ui.selectable_value(&mut self.show, ShowTypes::Custom, "Custom");
                                ui.selectable_value(&mut self.show, ShowTypes::All, "All");
                            });
                        });

                        if self.show == ShowTypes::Custom
                        {
                            let suffix = match self.custom_show
                            {
                                1 => "st place",
                                2 => "nd place",
                                3 => "rd place",
                                _ => "th place",
                            };

                            let text = self.custom_show.to_string() + suffix;
                            if ui.add(egui::Slider::new(&mut self.custom_show, 1..=self.bodies[self.gen as usize - 1].len()).text(text)).changed()
                            {
                                self.rbodies = vec![self.bodies[self.gen as usize - 1][self.custom_show - 1].clone()];
                            }
                        }
                        
                        if self.show != show_before
                        {
                            self.time = 0.0;
                            self.update_rbodies();
                        }
                    });
                    
                    ui.collapsing("Gen Info", |ui| {
                        
                        ui.label("Best dist ".to_string() + &self.bodies[self.gen as usize - 1][0].distance.unwrap().to_string());
                        ui.label("Mean dist ".to_string() + &(self.bodies[self.gen as usize - 1].iter().map(|i| i.distance.unwrap()).sum::<f32>() / self.bodies[self.gen as usize - 1].len() as f32).to_string());
                        ui.label("Median dist ".to_string() + &self.bodies[self.gen as usize - 1][self.bodies[self.gen as usize - 1].len() / 2].distance.unwrap().to_string());
                        ui.label("Worst dist ".to_string() + &self.bodies[self.gen as usize - 1].last().unwrap().distance.unwrap().to_string());
                    });
                    if self.show != ShowTypes::All
                    {
                        ui.collapsing("Viewing Creature Info", |ui| {
                            ui.label("Distance In Time ".to_string() + &self.rbodies[0].distance.unwrap().to_string());
                            ui.label("Time ".to_string() + &self.time.to_string());
                            ui.label("Distance ".to_string() + &self.rbodies[0].get_average_distance().to_string());
                            if self.rbodies[0].parent.is_some()
                            {
                                ui.label("Parent: gen ".to_string() + &self.rbodies[0].parent.unwrap().0.to_string() + &" index ".to_string() + &self.rbodies[0].parent.unwrap().1.to_string());
                            }
                        });
                    }
                    
                });
            
            let mut visuals = egui::Visuals::light();
            visuals.window_shadow.extrusion = 0.0;

            let style = egui::Style {
                visuals,
                ..Default::default()
            };
            egui_ctx.set_style(style);
        });
        egui_macroquad::draw();
    }

    pub fn cam(&mut self)
    {
        let zoom = 0.0;
        let mut cam = Camera2D::from_display_rect(Rect::new(zoom, zoom, screen_width() - zoom, screen_height() - zoom));
        cam.target.x = self.rbodies[0].pos.x + self.rbodies[0].get_average_distance();
        cam.target.y = Settings::FLOOR_Y - 100.0;
        set_camera(&cam);
    }

    pub fn update_rbodies(&mut self)
    {
        match self.show
        {
            ShowTypes::Best => {
                self.rbodies = vec![self.bodies[self.gen as usize - 1][0].clone()]
            },
            ShowTypes::Median => {
                self.rbodies = vec![self.bodies[self.gen as usize - 1][ self.bodies[self.gen as usize - 1].len() / 2].clone()]
            },
            ShowTypes::Worst => {
                self.rbodies = vec![self.bodies[self.gen as usize - 1].last().unwrap().clone()]
            },
            ShowTypes::Custom => {
                self.rbodies = vec![self.bodies[self.gen as usize - 1][self.custom_show - 1].clone()];
            },
            ShowTypes::All => {
                self.rbodies = self.bodies[self.gen as usize - 1].clone();
            },
            _ => println!("ERR update_rbodies(&mut self) in ecosystem.rs {:?}", self.show),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ShowTypes
{
    Best,
    Median,
    Worst,
    Custom,
    All,
}