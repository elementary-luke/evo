use crate::body::*;
use crate::point::*;
use crate::settings::*;
use crate::tree_body::*;
use crate::display_body::*;
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
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use egui::plot::{Line, Plot, PlotPoints, PlotPoint};

pub struct Ecosystem
{
    pub settings : Settings,
    seed : u64,
    time : f32,
    pub screen : Screens,
    paused : bool,
    paused_before : bool,
    bodies : Vec<Vec<Body>>,
    rbodies : Vec<Body>,

    gen : usize,
    add_gens_text : String,
    show : ShowTypes,
    custom_show : usize,

    showing_stats : bool,

    tree_cam_pos : Point,
    tree_slide : Option<usize>,
    slide_index : usize,
    tree_bodies : Vec<TreeBody>,
    
    display_cam_pos : Point,
    display_bodies : Vec<DisplayBody>,
    display_gen : usize,
    mouse_down_pos : Point,
    
    zoom : f32,
    mouse_on_ui : bool,
}

impl Ecosystem
{
    pub fn new() -> Ecosystem
    {
        Ecosystem {
            settings : Settings{..Default::default()},
            seed : 0,
            time : 0.0,
            screen : Screens::Creation,
            paused : false,
            paused_before : false,
            bodies : Vec::new(),
            rbodies : Vec::new(),
            
            gen : 0,
            add_gens_text : "1".to_string(),
            show : ShowTypes::Best,
            custom_show : 1,

            //Stats
            showing_stats : false,

            //family line
            tree_cam_pos : Point { x: 0.0, y: 0.0 },
            tree_slide : None,
            slide_index : 0,
            tree_bodies : Vec::new(),

            //generation display
            display_cam_pos : Point { x: 0.0, y: 0.0 },
            display_bodies : Vec::new(),
            display_gen : 0,
            mouse_down_pos : Point { x: 0.0, y: 0.0 },
            
            zoom : 1.0,
            mouse_on_ui : false,
        }
    }

    pub fn initialise(&mut self)
    {
        if self.settings.ceiling
        {
            self.settings.terrain.push((-10000.0, -10000.0, 10000.0, 100.0))
        }
        if self.settings.hurdles
        {
            for i in 0..15
            {
                self.settings.terrain.push((410.0 - 200.0 * i as f32, 390.0 - 10.0  * i as f32, 460.0 - 200.0 * i as f32, 900.0));
            }
            for i in 0..15
            {
                self.settings.terrain.push((810.0 + 200.0 * i as f32, 390.0 - 10.0  * i as f32, 860.0 + 200.0 * i as f32, 900.0));
            }
        }
        if self.settings.stairs
        {
            for i in 0..15
            {
                self.settings.terrain.push((410.0 - 70.0 * i as f32, 390.0 - 10.0  * i as f32, 480.0 - 70.0 * i as f32, 900.0));
            }
            for i in 0..15
            {
                self.settings.terrain.push((810.0 + 70.0 * i as f32, 390.0 - 10.0  * i as f32, 880.0 + 70.0 * i as f32, 900.0));
            }
        }

        if self.settings.random_seed
        {
            self.seed = macroquad::miniquad::date::now() as u64;
        }
        else
        {
            self.seed = self.settings.seed;
        }

        rand::srand(self.seed);

        self.bodies.push(Vec::new());
        self.create(self.settings.population_size);
        self.simulate();
        self.rbodies = vec![self.bodies[self.gen][0].clone()];
    }

    pub fn create (&mut self, n : usize)
    {
        let bodies = &mut self.bodies[self.gen];
        for _ in 0..n
        {
            bodies.push(Body::new_random(self.settings.x_bound, self.settings.y_bound, &self.settings));
        }
    }

    pub fn kill(&mut self)
    {
        // kill half the population, worse ones are more likely to die
        let mut to_kill : Vec<usize> = Vec::new();
        for i in 0..(self.bodies[self.gen + 1].len() / 2)
        {
            let f : f32 = i as f32 / self.bodies[self.gen + 1].len() as f32;
            let r : f32 = (rand::gen_range(-1.0 as f32, 1.0 as f32).powf(3.0) + 1.0) / 2.0;
            if f > r
            {
                to_kill.push(i);
            }
            else 
            {
                to_kill.push(self.bodies[self.gen + 1].len() - 1 - i);
            }
        }

        for i in 0..self.bodies[self.gen + 1].len()
        {
            self.bodies[self.gen + 1][i].previous = Some(i);
            self.bodies[self.gen + 1][i].age += 1;
        }
        
        to_kill.sort();
        to_kill.reverse();
        for i in to_kill
        {
            self.bodies[self.gen + 1].remove(i);
        }
    }

    pub fn repopulate(&mut self)
    {
        //clone and mutate survivors
        let mut new_bodies : Vec<Body> = Vec::new();
        for i in 0..self.bodies[self.gen + 1].len()
        {
            let mut new_body = self.bodies[self.gen + 1][i].clone();
            new_body.parent = Some((self.gen, self.bodies[self.gen + 1][i].previous.unwrap()));
            new_body.previous = None;
            new_body.age = 0;
            new_body.distance = None;
            new_body.mutate(&self.settings);
            new_bodies.push(new_body);
        }
        self.bodies[self.gen + 1].append(&mut new_bodies);
    }

    pub fn simulate(&mut self)
    {
        // get distance for every body
        let bodies = &mut self.bodies.last_mut().unwrap();
        for i in 0..bodies.len()
        {
            if bodies[i].distance.is_some()
            {
                continue;
            }
            bodies[i].pos = Point {x : screen_width()/2.0, y : self.settings.floor_y - self.settings.y_bound / 2.0};
            bodies[i].set_start_avg();

            let mut dist = bodies[i].clone().simulate(&self.settings);
    
            //if it went backwards flip it
            // if dist < 0.0
            // {
            //     bodies[i].flip(); // TODO FIX THIS AS IF BEST IS FLIPPED DOESNT DO THE SAME THING
            // }
            if dist.is_nan()
            {
                dist = 0.0;
            }
            bodies[i].distance = Some(dist.abs());
        }
        bodies.retain(|b| !b.distance.unwrap().is_nan());
        bodies.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

        //set nexts and children
        if self.gen + 1 != self.bodies.len() // make sure doesn't do it for when first creating population
        {
            for i in 0..self.bodies[self.gen + 1].len()
            {
                if self.bodies[self.gen + 1][i].previous.is_some()
                {
                    let index = self.bodies[self.gen + 1][i].previous.unwrap();
                    self.bodies[self.gen][index].next = Some(i);
                }
                else
                //if self.bodies[self.gen + 1][i].parent.is_some()
                {
                    let (gen, index) = self.get_earliest(self.bodies[self.gen + 1][i].parent.unwrap().0, self.bodies[self.gen + 1][i].parent.unwrap().1);
                    self.bodies[gen][index].children.push((self.gen + 1, i));
                }
            }
        }
    }

    pub fn run_view(&mut self, settings : Settings)
    {
        // if is_key_pressed(KeyCode::R)
        // {
        //     self.time = 0.0;
        //     self.update_rbodies();
        //     self.rbodies.push(self.rbodies[0].clone());
        //     self.rbodies[0].flip();
        //     self.rbodies[0].pos = Point {x : screen_width()/2.0, y : self.settings.FLOOR_Y - self.settings.Y_BOUND / 2.0};
        //     self.rbodies[1].pos = Point {x : screen_width()/2.0, y : self.settings.FLOOR_Y - self.settings.Y_BOUND / 2.0};
        // }
        
        self.draw_sky();
        self.draw_ground();
        self.draw_terrain(&settings);
        self.draw_signs();

        
        //pause if over the time limit or all of creature on floor
        for i in 0..self.rbodies.len()
        {
            if !self.paused
            {
                self.rbodies[i].update(self.time, &self.settings);

                if self.time > self.settings.time_given || self.rbodies.len() == 1 && self.rbodies[0].circles.iter().all(|c| c.on_floor)
                {
                    if !self.paused_before
                    {
                        self.paused_before = true;
                        self.paused = true;
                    }
                }
            }

            //set opacity for each body when all are showing. Best are most opaque
            self.rbodies[i].draw();
            if self.rbodies.len() > 1
            {
                self.rbodies[i].set_alpha((self.bodies[self.gen].len()-1-i) as f32/(self.bodies[self.gen].len() - 1) as f32);
            }
            else 
            {
                self.rbodies[0].set_alpha(0.8);
            }
        }

        if !self.paused
        {
            self.time += 1.0/60.0;
        }
    }

    pub fn draw_sky(&mut self)
    {
        if self.paused_before
        {
            // if creature's best distance is given, goes into night
            clear_background(color_u8!(	34.0, 51.0, 64.0, 1.0));
        }
        else 
        {
            //day
            clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));
        }
    }
    pub fn draw_signs(&mut self)
    {
        for i in 0..(((self.bodies[self.gen][0].distance.unwrap() + screen_width() / 2.0) / 200.0).ceil() as usize + 1)
        {
            let y = self.settings.floor_y - 250.0;
            let w = 80.0;
            let h = 40.0;
            let x = screen_width()/2.0 + i as f32 * 200.0 + self.bodies[self.gen][0].start_avg_x - w / 2.0;
            draw_rectangle(x, 
                y, 
                w, 
                h, 
                WHITE
            );
            draw_triangle(vec2(x + w / 2.0, y + h + 10.0), vec2(x + w / 2.0 + 10.0, y + h), vec2(x + w / 2.0 - 10.0, y + h), WHITE);
            draw_line(x + w / 2.0, self.settings.floor_y, x + w / 2.0, y, 2.0, Color {r : 1.0, g : 1.0, b : 1.0, a : 0.4});
            draw_text(&(i * 200).to_string(), x + 10.0, y + 30.0, 40.0, BLACK);
        }
        for i in 0..(((self.bodies[self.gen][0].distance.unwrap() + screen_width() / 2.0) / 200.0).ceil() as usize + 1)
        {
            let y = self.settings.floor_y - 250.0;
            let w = 80.0;
            let h = 40.0;
            let x = screen_width()/2.0 + i as f32 * -200.0 + self.bodies[self.gen][0].start_avg_x - w / 2.0;
            draw_rectangle(x, 
                y, 
                w, 
                h, 
                WHITE
            );
            draw_triangle(vec2(x + w / 2.0, y + h + 10.0), vec2(x + w / 2.0 + 10.0, y + h), vec2(x + w / 2.0 - 10.0, y + h), WHITE);
            draw_line(x + w / 2.0, self.settings.floor_y, x + w / 2.0, y, 2.0, Color {r : 1.0, g : 1.0, b : 1.0, a : 0.4});
            draw_text(&(i * 200).to_string(), x + 10.0, y + 30.0, 40.0, BLACK);
        }
    }

    pub fn draw_ground(&mut self)
    {
        draw_rectangle(
            -5000.0, 
            self.settings.floor_y, 
            10000.0, 
            self.settings.floor_y, 
            color_u8!(192.0, 255.0, 133.0, 255.0)
        );
    }

    pub fn draw_terrain(&mut self, settings : &Settings)
    {
        for i in settings.terrain.clone()
        {
            let x = i.0;
            let y = i.1;
            let w = i.2 - i.0;
            let h = i.3 - i.1;

            draw_rectangle(x, y, w, h, BLACK);
        }
    }

    pub fn run_gui(&mut self)
    {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("dashboard")
                .default_pos((0.0, 0.0))
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Seed ".to_string() + &self.seed.to_string());
                        if ui.button("📋").clicked()
                        {
                            ui.output_mut(|o| o.copied_text = self.seed.to_string());
                        }
                    });
                    ui.collapsing("Controls", |ui| {
                        ui.horizontal( |ui| {
                            if ui.button(if !self.paused {"⏸"} else {"▶"} ).clicked()
                            {
                                self.paused = !self.paused;
                            }
                            
                            if self.time != 0.0
                            {
                                if ui.button("↺").clicked()
                                {
                                    self.update_rbodies();
                                }
                            }

                            ui.label("time: ".to_string() + &self.time.to_string());
                        });
                        ui.horizontal(|ui| {
                            ui.label("View Year".to_string());
                            if ui.add(egui::Slider::new(&mut self.gen, 0..=self.bodies.len() - 1)).changed()
                            {
                                self.update_rbodies();
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Do").clicked()
                            {
                                if self.add_gens_text.parse::<u32>().is_ok()
                                {
                                    self.paused_before = false;
                                    self.gen = self.bodies.len() - 1;
                                    for _ in 0..self.add_gens_text.parse::<u32>().unwrap() as usize
                                    {
                                        self.bodies.push(self.bodies[self.gen].clone());
                                        self.kill();
                                        self.repopulate();
                                        self.simulate();
                                        self.gen += 1;
                                    }
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
                            ui.label("Year(s) ".to_string());

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
                            if ui.add(egui::Slider::new(&mut self.custom_show, 1..=self.bodies[self.gen].len()).text(text).drag_value_speed(1.0)).changed()
                            {
                                self.rbodies = vec![self.bodies[self.gen][self.custom_show - 1].clone()];
                                self.update_rbodies();
                            }
                        }
                        
                        if self.show != show_before
                        {
                            self.update_rbodies();
                        }
                        
                        if ui.button(if !self.showing_stats {"Show Stats"} else {"Hide Stats"}).clicked()
                        {
                            self.showing_stats = !self.showing_stats;
                        }
                    });
                    
                    ui.collapsing("Year Info", |ui| {
                        if ui.button("See All Bodies").clicked()
                        {
                            self.screen = Screens::GenerationDisplay;
                            self.display_gen = self.gen;
                            self. zoom = 1.0;
                            self.display_cam_pos = Point {x : 0.0, y : 0.0};
                            //self.zoom = 2.5;
                            //self.display_cam_pos = Point {x: screen_width(), y: screen_height() / 2.0};
                            self.calc_generation_display();
                        }
                        ui.label("Best dist ".to_string() + &self.bodies[self.gen][0].distance.unwrap().to_string());
                        ui.label("Mean dist ".to_string() + &(self.bodies[self.gen].iter().map(|i| i.distance.unwrap()).sum::<f32>() / self.bodies[self.gen].len() as f32).to_string());
                        ui.label("Median dist ".to_string() + &self.bodies[self.gen][self.bodies[self.gen].len() / 2].distance.unwrap().to_string());
                        ui.label("Worst dist ".to_string() + &self.bodies[self.gen].last().unwrap().distance.unwrap().to_string());
                    });
                    if self.show != ShowTypes::All
                    {
                        ui.collapsing("Viewing Creature Info", |ui| {
                            if self.rbodies.len() == 1 && self.rbodies[0].parent.is_some() && ui.button("see family line").clicked()
                            {
                                self.tree_bodies.clear();
                                self.screen = Screens::FamilyTree;
                                self.calc_family_tree();
                                self.slide_index = 0;
                                self.tree_slide = None;
                                self.tree_cam_pos = Point {x : 0.0, y : 0.0};
                            }
                            ui.label("Distance In Time ".to_string() + &self.rbodies[0].distance.unwrap().to_string());
                            ui.label("Time ".to_string() + &self.time.to_string());
                            if self.settings.heuristic == 1
                            {
                                ui.label("Energy Used: ".to_string() + &self.rbodies[0].energy_used.to_string());
                            }
                            
                            if self.settings.distance_based_on == 0
                            {
                                ui.label("Distance ".to_string() + &self.rbodies[0].get_average_distance().to_string());
                            }
                            else
                            {
                                ui.label("Distance ".to_string() + &self.rbodies[0].get_max_distance().to_string());
                            }
                            
                            ui.label("age ".to_string() + &self.rbodies[0].age.to_string() + " years");
                            if self.rbodies[0].previous.is_some()
                            {
                                if ui.button("see this a year ago").clicked()
                                {
                                    self.gen -= 1;
                                    self.show = ShowTypes::Custom;
                                    self.custom_show = self.rbodies[0].previous.unwrap() + 1;
                                    self.update_rbodies();
                                }
                                if ui.button("see when born").clicked()
                                {
                                    let (gen, index) = self.get_earliest(self.gen, self.custom_show - 1);
                                    self.gen = gen;
                                    self.show = ShowTypes::Custom;
                                    self.custom_show = index + 1;
                                    self.update_rbodies();
                                }
                            }
                            if self.rbodies[0].next.is_some()
                            {
                                if ui.button("see this a year later").clicked()
                                {
                                    self.gen += 1;
                                    self.show = ShowTypes::Custom;
                                    self.custom_show = self.rbodies[0].next.unwrap() + 1;
                                    self.update_rbodies();
                                }
                                if ui.button("see most recent").clicked()
                                {
                                    let (gen, index) = self.get_latest(self.gen, self.custom_show - 1);
                                    self.gen = gen;
                                    self.show = ShowTypes::Custom;
                                    self.custom_show = index + 1;
                                    self.update_rbodies();
                                }
                            }
                            if self.rbodies[0].parent.is_some()
                            {
                                let (gen, index) = self.get_latest_up_to_view_gen(self.rbodies[0].parent.unwrap().0, self.rbodies[0].parent.unwrap().1);
                                if ui.button("see parent".to_string() + if gen < self.gen {"[dead]"} else {""}).clicked()
                                {
                                    self.gen = gen;
                                    self.show = ShowTypes::Custom;
                                    self.custom_show = index + 1;
                                    self.update_rbodies();
                                }

                                let (pgen, pindex) = self.get_earliest(gen, index);
                                if self.bodies[pgen][pindex].children.len() > 0
                                {
                                    ui.collapsing("siblings", |ui| {
                                        for i in 0..self.bodies[pgen][pindex].children.len()
                                        {
                                            if self.bodies[pgen][pindex].children[i] == (self.gen, self.custom_show - 1)
                                            {
                                                continue;
                                            }
                                            let (cgen, cindex) = self.bodies[pgen][pindex].children[i];
                                            ui.horizontal(|ui| {
                                                ui.label(format!("byear: {} place: {}", cgen, cindex + 1));
                                                if ui.button("see when born".to_string()).clicked()
                                                {
                                                    self.gen = cgen;
                                                    self.show = ShowTypes::Custom;
                                                    self.custom_show = cindex + 1;
                                                    self.update_rbodies();
                                                }
                                            });
                                            
                                        }
                                    });
                                }
                            }
                            if self.rbodies[0].children.len() > 0
                            {
                                let gen = self.gen;
                                let index = self.custom_show - 1;
                                let (gen, index) = self.get_earliest(gen, index);
                                ui.collapsing("children", |ui| {
                                    for i in 0..self.bodies[gen][index].children.len()
                                    {
                                        let (cgen, cindex) = self.bodies[gen][index].children[i];
                                        ui.horizontal(|ui| {
                                            ui.label(format!("byear: {} place: {}", cgen, cindex + 1));
                                            if ui.button("see when born".to_string()).clicked()
                                            {
                                                self.gen = cgen;
                                                self.show = ShowTypes::Custom;
                                                self.custom_show = cindex + 1;
                                                self.update_rbodies();
                                            }
                                        });
                                        
                                    }
                                });
                            }
                        });
                    }
                    
                });

        if self.showing_stats
        {
            egui::Window::new("Stats")
                .default_pos((400.0, 400.0))
                .show(egui_ctx, |ui| {
                    //let sin: PlotPoints = PlotPoints::Owned(vec![PlotPoint::new(0.0, 0.0), PlotPoint::new(1.0, 1.0), PlotPoint::new(3.0, 0.0)]) ;
                    let sin = PlotPoints::from_ys_f32(&[0.0, 1.0, 0.0]);
                    let line = Line::new(sin);
                    Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
                });
        }
        
        
        });
        egui_macroquad::draw();
    }

    pub fn run_cam(&mut self)
    {
        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
        //cam.target.x += (self.rbodies[0].pos.x + self.rbodies[0].get_average_distance() - cam.target.x) * 0.8;
        //cam.target.y += (self.rbodies[0].pos.y + self.rbodies[0].get_average_y() - cam.target.y) * 0.8;
        cam.target.x = self.rbodies[0].pos.x + self.rbodies[0].get_average_distance();
        cam.target.y = self.rbodies[0].pos.y + self.rbodies[0].get_average_y();
        set_camera(&cam);
    }

    fn get_earliest(&mut self, gen : usize, index : usize) -> (usize, usize)
    {
        if self.bodies[gen][index].previous.is_none()
        {
            return (gen, index);
        }
        return self.get_earliest(gen - 1, self.bodies[gen][index].previous.unwrap());
    }

    fn get_latest(&mut self, gen : usize, index : usize) -> (usize, usize)
    {
        if self.bodies[gen][index].next.is_none()
        {
            return (gen, index);
        }
        return self.get_latest(gen + 1, self.bodies[gen][index].next.unwrap());
    }

    fn get_latest_up_to_view_gen(&mut self, gen : usize, index : usize) -> (usize, usize)
    {
        if self.bodies[gen][index].next.is_none() || gen == self.gen
        {
            return (gen, index);
        }
        return self.get_latest(gen + 1, self.bodies[gen][index].next.unwrap());
    }

    pub fn update_rbodies(&mut self)
    {
        self.time = 0.0;
        self.paused_before = false;
        //change which body(ies) is shown on the main screen
        match self.show
        {
            ShowTypes::Best => {
                self.custom_show = 1;
                self.rbodies = vec![self.bodies[self.gen][0].clone()]
            },
            ShowTypes::Median => {
                self.custom_show = self.bodies[self.gen].len() / 2 + 1;
                self.rbodies = vec![self.bodies[self.gen][ self.bodies[self.gen].len() / 2].clone()]
            },
            ShowTypes::Worst => {
                self.custom_show = self.bodies[self.gen].len();
                self.rbodies = vec![self.bodies[self.gen].last().unwrap().clone()]
            },
            ShowTypes::Custom => {
                self.rbodies = vec![self.bodies[self.gen][self.custom_show - 1].clone()];
            },
            ShowTypes::All => {
                self.rbodies = self.bodies[self.gen].clone();
            },
            _ => println!("ERR update_rbodies(&mut self) in ecosystem.rs {:?}", self.show),
        }
    }
    
    pub fn calc_family_tree(&mut self)
    {
        let (mut gen, mut index) = (self.gen, self.custom_show - 1);
        (gen, index) = self.get_earliest(gen, index);

        let mut body = TreeBody::from(self.bodies[gen][index].clone(), (gen, index));
        
        let (mut pgen, mut pindex) = body.parent.unwrap();
        (pgen, pindex) = self.get_earliest(pgen, pindex);
        self.slide_index = self.bodies[pgen][pindex].children.iter().position(|&x| x == (gen, index)).unwrap();
        body.parent = Some((pgen, pindex));

        self.tree_bodies.push(body);

        //add parents
        let mut pos_y = -500.0;
        (gen, index) = (pgen, pindex);
        loop
        {
            // print!("{:?} ", (gen, index));
            let mut body = TreeBody::from(self.bodies[pgen][pindex].clone(), (pgen, pindex));
            if self.bodies[gen][index].parent.is_some()
            {
                (pgen, pindex) = self.bodies[gen][index].parent.unwrap();
                (pgen, pindex) = self.get_earliest(pgen, pindex);
                body.parent = Some((pgen, pindex));
            }
            body.pos.y = pos_y;
            pos_y -= 500.0;
            self.tree_bodies.insert(0, body);
            if gen == 0
            {
                break;
            }
            (gen, index) = (pgen, pindex);
        }

        //add children
        (gen, index) = (self.gen, self.custom_show - 1);
        (gen, index) = self.get_earliest(gen, index);
        if self.bodies[gen][index].children.len() == 0
        {
            return;
        }
        let (mut cgen, mut cindex);
        pos_y = 500.0;
        loop
        {
            (cgen, cindex) = self.bodies[gen][index].children[0];
            (cgen, cindex) = self.get_earliest(cgen, cindex);
            let mut body = TreeBody::from(self.bodies[cgen][cindex].clone(), (cgen, cindex));
            if self.bodies[gen][index].parent.is_some()
            {
                body.parent = Some((gen, index));
            }
            body.pos.y = pos_y;
            pos_y += 500.0;
            self.tree_bodies.push(body);
            if self.bodies[cgen][cindex].children.len() == 0
            {
                break;
            }
            (gen, index) = (cgen, cindex);
        }
    }

    pub fn recalc_family_tree(&mut self)
    {
        self.tree_bodies = self.tree_bodies[..self.tree_slide.unwrap() + 1].to_vec();
        let (mut gen, mut index) = self.tree_bodies[self.tree_bodies.len() - 1].body_array_index;
        (gen, index) = self.get_earliest(gen, index);
        if self.bodies[gen][index].children.len() == 0
        {
            return;
        }
        let mut pos_y = self.tree_bodies[self.tree_bodies.len() - 1].pos.y + 500.0;
        loop
        { 
            let (mut cgen, mut cindex) = self.bodies[gen][index].children[0];
            (cgen, cindex) = self.get_earliest(cgen, cindex);
            let mut body = TreeBody::from(self.bodies[cgen][cindex].clone(), (cgen, cindex));
            body.parent = Some((gen, index));
            body.pos.y = pos_y;
            pos_y += 500.0;
            self.tree_bodies.push(body);
            if self.bodies[cgen][cindex].children.len() == 0
            {
                break;
            }
            (gen, index) = (cgen, cindex);
        }
    }

    pub fn draw_family_tree(&mut self)
    {
        clear_background(Color::from_rgba(249, 251, 231, 255));
        //println!("tree_slide {:?}, slide index {:?}", self.tree_slide, self.slide_index);
        'outer : for i in 0..self.tree_bodies.len()
        {
            // if user has selected a boy in the family line, show siblings
            if self.tree_slide.is_some() && self.tree_slide == Some(i)
            {
                let (pgen, pindex) = self.tree_bodies[i].parent.unwrap();
                let num_before = self.slide_index;
                let mut x = -300.0 * num_before as f32;
                for j in 0..(num_before as usize)
                {
                    let (mut gen, mut index) = self.bodies[pgen][pindex].children[j];
                    (gen, index) = self.get_earliest(gen, index);

                    let mut body = TreeBody::from(self.bodies[gen][index].clone(), (gen, index));
                    body.parent = Some((pgen, pindex));
                    body.pos.x = x;
                    body.pos.y = self.tree_bodies[i].pos.y;
                    if (body.pos.x - self.tree_cam_pos.x).abs() < screen_width() * self.zoom && (body.pos.y - self.tree_cam_pos.y).abs() < screen_height() * self.zoom
                    {
                        body.draw(Point{x:0.0, y:0.0});
                    }
                    x += 300.0;
                    if body.mouse_on((Point::from(mouse_position()) - Point { x: screen_width() / 2.0, y: screen_height() / 2.0}) * Point {x : self.zoom, y : self.zoom} + self.tree_cam_pos)
                    {
                        if is_mouse_button_released(MouseButton::Left) && self.slide_index != j
                        {
                            self.slide_index = j;
                            self.tree_bodies[self.tree_slide.unwrap()] = body;
                            self.recalc_family_tree();
                            break 'outer;
                        }
                    }
                }

                for j in (num_before)..(self.bodies[pgen][pindex].children.len() as usize)
                {
                    let (mut gen, mut index) = self.bodies[pgen][pindex].children[j];
                    (gen, index) = self.get_earliest(gen, index);

                    let mut body = TreeBody::from(self.bodies[gen][index].clone(), (gen, index));
                    body.parent = Some((pgen, pindex));
                    body.pos.x = x;
                    body.pos.y = self.tree_bodies[i].pos.y;
                    if (body.pos.x - self.tree_cam_pos.x).abs() < screen_width() * self.zoom && (body.pos.y - self.tree_cam_pos.y).abs() < screen_height() * self.zoom
                    {
                        body.draw(Point{x:0.0, y:0.0});
                    }
                    x += 300.0;
                    //if a sibling is clicked, change the family line below the selected one
                    if body.mouse_on((Point::from(mouse_position()) - Point { x: screen_width() / 2.0, y: screen_height() / 2.0}) * Point {x : self.zoom, y : self.zoom} + self.tree_cam_pos)
                    {
                        if is_mouse_button_released(MouseButton::Left) && self.slide_index != j
                        {
                            self.slide_index = j;
                            self.tree_bodies[self.tree_slide.unwrap()] = body;
                            self.recalc_family_tree();
                            break 'outer;
                        }
                    }
                }
            }
            else
            {
                // draw family line
                if (self.tree_bodies[i].pos.x - self.tree_cam_pos.x).abs() < screen_width() * self.zoom && (self.tree_bodies[i].pos.y - self.tree_cam_pos.y).abs() < screen_height() * self.zoom
                {
                    self.tree_bodies[i].draw(Point{x:0.0, y:0.0});
                }
                
                //select a body to see its siblings
                
                if self.tree_bodies[i].parent.is_some()
                {
                    let (pgen, pindex) = self.tree_bodies[i].parent.unwrap();
                    if self.tree_slide != Some(i) && self.bodies[pgen][pindex].children.len() > 1 && self.tree_bodies[i].mouse_on((Point::from(mouse_position()) - Point { x: screen_width() / 2.0, y: screen_height() / 2.0}) * Point {x : self.zoom, y : self.zoom} + self.tree_cam_pos)
                    {
                        if is_mouse_button_released(MouseButton::Left)
                        {
                            for j in 0..self.tree_bodies.len()
                            {
                                self.tree_bodies[j].pos.x = 0.0;
                            }
                            self.tree_slide = Some(i);
                            self.slide_index = self.bodies[pgen][pindex].children.iter().position(|&x| x == self.tree_bodies[i].body_array_index).unwrap();
                        }
                    }
                }
                
            }
        }
        
        if is_key_pressed(KeyCode::Escape)
        {
            self.screen = Screens::Simulation;
        }
    }

    pub fn family_tree_cam(&mut self)
    {
        if (is_mouse_button_pressed(MouseButton::Middle) || is_mouse_button_pressed(MouseButton::Right)) && !self.mouse_on_ui
        {
            self.mouse_down_pos = Point {x : mouse_position().0, y : mouse_position().1} * Point {x : self.zoom, y : self.zoom} + self.tree_cam_pos;
        }
        if (is_mouse_button_down(MouseButton::Middle)  || is_mouse_button_down(MouseButton::Right)) && !self.mouse_on_ui
        {
            self.tree_cam_pos.x = self.mouse_down_pos.x - mouse_position().0 * self.zoom;
            self.tree_cam_pos.y = self.mouse_down_pos.y - mouse_position().1 * self.zoom;
        }

        if is_key_down(KeyCode::S)
        {
            self.tree_cam_pos.y += 10.0 * self.zoom;
        }
        if is_key_down(KeyCode::W)
        {
            self.tree_cam_pos.y -= 10.0 * self.zoom;
        }
        if is_key_down(KeyCode::A)
        {
            self.tree_cam_pos.x -= 10.0 * self.zoom;
        }
        if is_key_down(KeyCode::D)
        {
            self.tree_cam_pos.x += 10.0 * self.zoom;
        }
        if mouse_wheel().1 != 0.0
        {
            self.zoom -= mouse_wheel().1 * 0.002;
            if self.zoom > 4.0
            {
                self.zoom = 4.0;
            }
            else if self.zoom < 0.52
            {
                self.zoom = 0.52;
            }
        }

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width() * self.zoom, screen_height() * self.zoom));
        cam.target.x = self.tree_cam_pos.x;
        cam.target.y = self.tree_cam_pos.y;
        set_camera(&cam);
    }

    pub fn family_tree_gui(&mut self)
    {
        egui_macroquad::ui(|egui_ctx| {
            let window = egui::Window::new("dashboard").show(egui_ctx, |ui| {
                if ui.button("back").clicked()
                {
                    self.tree_bodies.clear();
                    self.screen = Screens::Simulation;
                }
                ui.horizontal(|ui| {
                    ui.label("Zoom".to_string());
                    ui.add(egui::Slider::new(&mut self.zoom, 0.52..=2.5).drag_value_speed(0.01).show_value(false));
                });
                ui.label("");
                ui.label("Left Click: See/Select Sibling");
                ui.label("Right Click / Middle Click / wasd: Pan");
                ui.label("Wheel: Zoom ");
            });
            if window.is_some() && window.unwrap().response.rect.contains(mouse_position().into())
            {
                self.mouse_on_ui = true;
            }
            else 
            {
                self.mouse_on_ui = false;
            }
        });
        egui_macroquad::draw();
    }

    pub fn calc_generation_display(&mut self,)
    {
        self.display_bodies.clear();
        for i in 0..self.bodies[self.display_gen].len()
        {
            self.display_bodies.push(DisplayBody::from(self.bodies[self.display_gen][i].clone(), (self.display_gen, i)));
        }
    }

    pub fn draw_generation_display(&mut self)
    {
        clear_background(Color::from_rgba(249, 251, 231, 255));
        let mut x = 0.0;
        let mut y = 0.0;
        //draw all bodies in a grid
        for i in 0..self.display_bodies.len()
        {
            if (x * 300.0 - self.display_cam_pos.x).abs() < screen_width() * self.zoom && (y - self.display_cam_pos.y).abs() < screen_height() * self.zoom
            {
                self.display_bodies[i].pos = Point {x : x * 300.0, y};
                self.display_bodies[i].draw(Point{x : 0.0, y : 0.0});

                //when you click on a body take you to it
                if !self.mouse_on_ui && self.display_bodies[i].mouse_on( (Point::from(mouse_position()) - Point { x: screen_width() / 2.0, y: screen_height() / 2.0}) * Point {x : self.zoom, y : self.zoom} + self.display_cam_pos )
                {
                    if is_mouse_button_released(MouseButton::Left)
                    {
                        self.gen = self.display_bodies[i].body_array_index.0;
                        self.show = ShowTypes::Custom;
                        self.custom_show = self.display_bodies[i].body_array_index.1 + 1;
                        self.update_rbodies();
                        self.screen = Screens::Simulation;
                    }
                }
            
            }
            x += 1.0;
            if x == 10.0
            {
                x = 0.0;
                y += 500.0;
            }
        }
        if is_key_pressed(KeyCode::Escape)
        {
            self.screen = Screens::Simulation;
        }
    }

    pub fn generation_display_cam(&mut self)
    {
        //drag camera using middle mouse button or right mouse button
        if (is_mouse_button_pressed(MouseButton::Middle) || is_mouse_button_pressed(MouseButton::Right)) && !self.mouse_on_ui
        {
            self.mouse_down_pos = Point {x : mouse_position().0, y : mouse_position().1} * Point {x : self.zoom, y : self.zoom} + self.display_cam_pos;
        }
        if (is_mouse_button_down(MouseButton::Middle)  || is_mouse_button_down(MouseButton::Right)) && !self.mouse_on_ui
        {
            self.display_cam_pos.x = self.mouse_down_pos.x - mouse_position().0 * self.zoom;
            self.display_cam_pos.y = self.mouse_down_pos.y - mouse_position().1 * self.zoom;
        }
        //can move cam with wasd
        if is_key_down(KeyCode::S)
        {
            self.display_cam_pos.y += 10.0 * self.zoom;
        }
        if is_key_down(KeyCode::W)
        {
            self.display_cam_pos.y -= 10.0 * self.zoom;
        }
        if is_key_down(KeyCode::A)
        {
            self.display_cam_pos.x -= 10.0 * self.zoom;
        }
        if is_key_down(KeyCode::D)
        {
            self.display_cam_pos.x += 10.0 * self.zoom;
        }
        if mouse_wheel().1 != 0.0
        {
            self.zoom -= mouse_wheel().1 * 0.002;
            if self.zoom > 2.5
            {
                self.zoom = 2.5;
            }
            else if self.zoom < 0.52
            {
                self.zoom = 0.52;
            }
        }

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width() * self.zoom, screen_height() * self.zoom));
        cam.target.x = self.display_cam_pos.x;
        cam.target.y = self.display_cam_pos.y;
        set_camera(&cam);
    }

    pub fn generation_display_gui(&mut self)
    {
        egui_macroquad::ui(|egui_ctx| {
            let window = egui::Window::new("dashboard").show(egui_ctx, |ui| {
                if ui.button("back").clicked()
                {
                    self.display_bodies.clear();
                    self.screen = Screens::Simulation;
                }
                ui.horizontal(|ui| {
                    ui.label("View Year".to_string());
                    if ui.add(egui::Slider::new(&mut self.display_gen, 0..=self.bodies.len() - 1)).changed()
                    {
                        self.calc_generation_display();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Zoom".to_string());
                    ui.add(egui::Slider::new(&mut self.zoom, 0.52..=2.5).drag_value_speed(0.01).show_value(false));
                });
                ui.label("");
                ui.label("Left Click: View Body running");
                ui.label("Right Click / Middle Click / wasd: Pan");
                ui.label("Wheel: Zoom ");

            });

            if window.is_some() && window.unwrap().response.rect.contains(mouse_position().into())
            {
                self.mouse_on_ui = true;
            }
            else 
            {
                self.mouse_on_ui = false;
            }
        });
        egui_macroquad::draw();
    }

    pub fn creation_gui(&mut self)
    {
        let defaults = Settings {..Default::default()};
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("creation screen").resizable(false).collapsible(false).fixed_pos((0.0, 0.0)).default_size((screen_width(), screen_height())).show(egui_ctx, |ui| {
                egui::ScrollArea::both().max_height(screen_height()).max_width(screen_width()).show(ui, |ui| {
                    egui::Grid::new("some_unique_id").striped(true).show(ui, |ui| {
                        ui.label("Seed");
                        if ui.button(if self.settings.random_seed {"random"} else {"custom"}).clicked()
                        {
                            self.settings.random_seed = !self.settings.random_seed;
                        }
                        ui.end_row();
                        

                        if !self.settings.random_seed
                        {
                            ui.label("");
                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut self.settings.seed));
                                if ui.button("📋").clicked()
                                {
                                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                    if ctx.get_contents().is_ok() && ctx.get_contents().unwrap().parse::<u64>().is_ok()
                                    {
                                        self.settings.seed = ctx.get_contents().unwrap().parse::<u64>().unwrap();
                                    }
                                }
                            });
                            ui.end_row();
                        }

                        ui.end_row();
                        ui.label("Physics");
                        ui.end_row();

                        ui.label("Gravity");
                        ui.horizontal(|ui|
                        {
                            ui.add(egui::DragValue::new(&mut self.settings.grav).speed(0.01));
                            if self.settings.grav != defaults.grav && ui.button("↺").clicked()
                            {
                                self.settings.grav = defaults.grav;
                            }
                        });
                        
                        ui.end_row();

                        ui.label("Air Slip");
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut self.settings.drag, 0.0..=1.0).clamp_to_range(false));
                            if self.settings.drag != defaults.drag && ui.button("↺").clicked()
                            {
                                self.settings.drag = defaults.drag;
                            }
                        });
                        
                        ui.end_row();

                        ui.end_row();
                        ui.label("Base Population Generation Settings").on_hover_text("settings for the first generation, used to direct the evolution");
                        ui.label("");
                        ui.label("Mutation limits").on_hover_text("limits for a body after mutation");
                        ui.end_row();

                        ui.label("Population Size");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.population_size).speed(10).clamp_range(1..=usize::MAX));
                            if self.settings.population_size != defaults.population_size && ui.button("↺").clicked()
                            {
                                self.settings.population_size = defaults.population_size;
                            }
                        });
                        ui.end_row();
                        
                        ui.label("Max Body Width");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.x_bound).speed(10).clamp_range(0.0..=f32::MAX));
                            if self.settings.x_bound != defaults.x_bound && ui.button("↺").clicked()
                            {
                                self.settings.x_bound = defaults.x_bound;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cx_bound).speed(10).clamp_range(0.0..=f32::MAX));
                            if self.settings.cx_bound != defaults.cx_bound && ui.button("↺").clicked()
                            {
                                self.settings.cx_bound = defaults.cx_bound;
                            }
                        });
                        ui.end_row();

                        ui.label("Max Body Height");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.y_bound).speed(10).clamp_range(0.0..=f32::MAX));
                            if self.settings.y_bound != defaults.y_bound && ui.button("↺").clicked()
                            {
                                self.settings.y_bound = defaults.y_bound;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cy_bound).speed(10).clamp_range(0.0..=f32::MAX));
                            if self.settings.cy_bound != defaults.cy_bound && ui.button("↺").clicked()
                            {
                                self.settings.cy_bound = defaults.cy_bound;
                            }
                        });
                        ui.end_row();

                        let min = self.settings.slip_min.clone();
                        ui.label("Node Slip Min");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.slip_min).speed(0.01).clamp_range(0.0..=1.0));
                            if self.settings.slip_min != defaults.slip_min && ui.button("↺").clicked()
                            {
                                self.settings.slip_min = defaults.slip_min;
                            }
                        });
                        let mut cmin = self.settings.cslip_min.clone();
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cslip_min).speed(0.01).clamp_range(0.0..=1.0));
                            if self.settings.cslip_min != defaults.cslip_min && ui.button("↺").clicked()
                            {
                                self.settings.cslip_min = defaults.cslip_min;
                            }
                        });
                        ui.end_row();

                        ui.label("Node Slip Max");
                        ui .horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.slip_max).speed(0.01).clamp_range(min..=1.0));
                            if self.settings.slip_max != defaults.slip_max && ui.button("↺").clicked()
                            {
                                self.settings.slip_max = defaults.slip_max;
                            }
                        });
                        ui .horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cslip_max).speed(0.01).clamp_range(cmin..=1.0));
                            if self.settings.cslip_max != defaults.cslip_max && ui.button("↺").clicked()
                            {
                                self.settings.cslip_max = defaults.cslip_max;
                            }
                        });
                        ui.end_row();


                        let mut min = self.settings.strength_min.clone();
                        ui.label("Muscle Strength Min");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.strength_min).speed(1.0).clamp_range(0.0..=f32::MAX));
                            if self.settings.strength_min != defaults.strength_min && ui.button("↺").clicked()
                            {
                                self.settings.strength_min = defaults.strength_min;
                            }
                        });
                        cmin = self.settings.cstrength_min.clone();
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cstrength_min).speed(1.0).clamp_range(0.0..=f32::MAX));
                            if self.settings.cstrength_min != defaults.cstrength_min && ui.button("↺").clicked()
                            {
                                self.settings.cstrength_min = defaults.cstrength_min;
                            }
                        });
                        ui.end_row();

                        ui.label("Muscle Strength Max");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.strength_max).speed(1.0).clamp_range(min..=f32::MAX).clamp_range(0.0..=f32::MAX));
                            if self.settings.strength_max != defaults.strength_max && ui.button("↺").clicked()
                            {
                                self.settings.strength_max = defaults.strength_max;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cstrength_max).speed(1.0).clamp_range(cmin..=f32::MAX).clamp_range(0.0..=f32::MAX));
                            if self.settings.cstrength_max != defaults.cstrength_max && ui.button("↺").clicked()
                            {
                                self.settings.cstrength_max = defaults.cstrength_max;
                            }
                        });
                        ui.end_row();


                        min = self.settings.contracted_time_min.clone();
                        ui.label("Contracted Time Min");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.contracted_time_min).speed(0.01).clamp_range(0.0..=f32::MAX));
                            if self.settings.contracted_time_min != defaults.contracted_time_min && ui.button("↺").clicked()
                            {
                                self.settings.contracted_time_min = defaults.contracted_time_min;
                            }
                        });
                        cmin = self.settings.ccontracted_time_min.clone();
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.ccontracted_time_min).speed(0.01).clamp_range(0.0..=f32::MAX));
                            if self.settings.ccontracted_time_min != defaults.ccontracted_time_min && ui.button("↺").clicked()
                            {
                                self.settings.ccontracted_time_min = defaults.ccontracted_time_min;
                            }
                        });
                        ui.end_row();

                        ui.label("Extended Time Max");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.contracted_time_max).speed(0.01).clamp_range(min..=f32::MAX));
                            if self.settings.contracted_time_max != defaults.contracted_time_max && ui.button("↺").clicked()
                            {
                                self.settings.contracted_time_max = defaults.contracted_time_max;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.ccontracted_time_max).speed(0.01).clamp_range(cmin..=f32::MAX));
                            if self.settings.ccontracted_time_max != defaults.ccontracted_time_max && ui.button("↺").clicked()
                            {
                                self.settings.ccontracted_time_max = defaults.ccontracted_time_max;
                            }
                        });
                        ui.end_row();


                        min = self.settings.contracted_len_min.clone();
                        ui.label("Contracted Len Min");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.contracted_len_min).speed(1.0).clamp_range(0.0..=f32::MAX));
                            if self.settings.contracted_len_min != defaults.contracted_len_min && ui.button("↺").clicked()
                            {
                                self.settings.contracted_len_min = defaults.contracted_len_min;
                            }
                        });
                        cmin = self.settings.ccontracted_len_min.clone();
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.ccontracted_len_min).speed(1.0).clamp_range(0.0..=f32::MAX));
                            if self.settings.ccontracted_len_min != defaults.ccontracted_len_min && ui.button("↺").clicked()
                            {
                                self.settings.ccontracted_len_min = defaults.ccontracted_len_min;
                            }
                        });
                        ui.end_row();

                        ui.label("Contracted Len Max");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.contracted_len_max).speed(1.0).clamp_range(min..=f32::MAX));
                            if self.settings.contracted_len_max != defaults.contracted_len_max && ui.button("↺").clicked()
                            {
                                self.settings.contracted_len_max = defaults.contracted_len_max;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.ccontracted_len_max).speed(1.0).clamp_range(cmin..=f32::MAX));
                            if self.settings.ccontracted_len_max != defaults.ccontracted_len_max && ui.button("↺").clicked()
                            {
                                self.settings.ccontracted_len_max = defaults.ccontracted_len_max;
                            }
                        });
                        ui.end_row();


                        let min = self.settings.min_circles.clone();
                        ui.label("Circle Number Min");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.min_circles).speed(1).clamp_range(0..=usize::MAX));
                            if self.settings.min_circles != defaults.min_circles && ui.button("↺").clicked()
                            {
                                self.settings.min_circles = defaults.min_circles;
                            }
                        });
                        let cmin = self.settings.cmin_circles.clone();
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cmin_circles).speed(1).clamp_range(0..=usize::MAX));
                            if self.settings.cmin_circles != defaults.cmin_circles && ui.button("↺").clicked()
                            {
                                self.settings.cmin_circles = defaults.cmin_circles;
                            }
                        });
                        ui.end_row();

                        ui.label("Circle Number Max");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.max_circles).speed(1).clamp_range(min..=usize::MAX));
                            if self.settings.max_circles != defaults.max_circles && ui.button("↺").clicked()
                            {
                                self.settings.max_circles = defaults.max_circles;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.settings.cmax_circles).speed(1).clamp_range(cmin..=usize::MAX));
                            if self.settings.cmax_circles != defaults.cmax_circles && ui.button("↺").clicked()
                            {
                                self.settings.cmax_circles = defaults.cmax_circles;
                            }
                        });
                        ui.end_row();

                        ui.label("Time Given");
                        ui.add(egui::DragValue::new(&mut self.settings.time_given).speed(0.01).clamp_range(0.0..=f32::MAX));
                        if self.settings.time_given != defaults.time_given && ui.button("↺").clicked()
                        {
                            self.settings.time_given = defaults.time_given;
                        }
                        ui.end_row();

                        // ui.label("Heuristic");
                        // if ui.button(if self.settings.heuristic == 0 {"Distance"} else {"Distance/Energy"}).clicked()
                        // {
                        //     self.settings.heuristic = 1 - self.settings.heuristic;
                        // }
                        ui.end_row();

                        ui.label("Distance Based On");
                        if ui.button(if self.settings.distance_based_on == 0 {"avg of nodes"} else {"furthest node"}).clicked()
                        {
                            self.settings.distance_based_on = 1 - self.settings.distance_based_on;
                        }
                        ui.end_row();

                        ui.label("hurdles");
                        if ui.button(if self.settings.hurdles {"on"} else {"off"}).clicked()
                        {
                            self.settings.hurdles = !self.settings.hurdles;
                            if self.settings.stairs
                            {
                                self.settings.stairs = false;
                            }
                        }
                        ui.end_row();

                        ui.label("stairs");
                        if ui.button(if self.settings.stairs {"on"} else {"off"}).clicked()
                        {
                            self.settings.stairs = !self.settings.stairs;
                            if self.settings.hurdles
                            {
                                self.settings.hurdles = false;
                            }
                        }
                        ui.end_row();

                        ui.label("ceiling (good for 0 gravity)");
                        if ui.button(if self.settings.ceiling {"on"} else {"off"}).clicked()
                        {
                            self.settings.ceiling = !self.settings.ceiling;
                        }
                        ui.end_row();

                        if ui.button("begin").clicked()
                        {
                            self.initialise();
                            self.screen = Screens::Simulation;
                        }
                        ui.end_row();
                        ui.end_row();
                        ui.end_row();
                        ui.end_row();
                        ui.label("");
                        
                    });

                //ui.allocate_space(egui::Vec2::new(screen_width(), screen_height()));
                });
            
            let mut visuals = egui::Visuals::light();
            visuals.window_shadow.extrusion = 0.0;

            let style = egui::Style {
                visuals,
                ..Default::default()
            };
            egui_ctx.set_style(style);
            });
        });
        
        egui_macroquad::draw();
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
#[derive(Debug, Clone, PartialEq)]
pub enum Screens
{
    Creation,
    Simulation,
    FamilyTree,
    GenerationDisplay,
    Stats,
}
