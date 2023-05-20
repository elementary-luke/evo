use crate::body::*;
use crate::point::*;
use crate::settings::*;
use crate::tree_body::*;
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
    paused : bool,
    paused_before : bool,
    bodies : Vec<Vec<Body>>,
    rbodies : Vec<Body>,
    gen : usize,
    add_gens_text : String,
    show : ShowTypes,
    custom_show : usize,
    tree_cam_pos : Point,
    tree_bodies : Vec<Vec<TreeBody>>,
    pub screen : Screens,
}

impl Ecosystem
{
    pub fn new() -> Ecosystem
    {
        Ecosystem {
            seed : 0,
            time : 0.0,
            paused : false,
            paused_before : false,
            bodies : Vec::new(),
            rbodies : Vec::new(),
            tree_bodies : Vec::new(),
            gen : 0,
            add_gens_text : "1".to_string(),
            show : ShowTypes::Best,
            custom_show : 1,
            screen : Screens::Simulation,
            tree_cam_pos : Point { x: 0.0, y: 0.0 },
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
        self.rbodies = vec![self.bodies[self.gen][0].clone()];
    }

    pub fn create (&mut self, n : usize)
    {
        let bodies = &mut self.bodies[self.gen];
        for _ in 0..n
        {
            bodies.push(Body::new_random(Settings::X_BOUND, Settings::Y_BOUND));
        }
    }

    pub fn kill(&mut self)
    {
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
        let mut new_bodies : Vec<Body> = Vec::new();
        for i in 0..self.bodies[self.gen + 1].len()
        {
            let mut new_body = self.bodies[self.gen + 1][i].clone();
            new_body.parent = Some((self.gen, self.bodies[self.gen + 1][i].previous.unwrap()));
            new_body.previous = None;
            new_body.age = 0;
            new_body.distance = None;
            new_body.mutate();
            new_bodies.push(new_body);
        }
        self.bodies[self.gen + 1].append(&mut new_bodies);
    }

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
            // if dist < 0.0
            // {
            //     bodies[i].flip(); // TODO FIX THIS AS IF BEST IS FLIPPED DOESNT DO THE SAME THING
            // }
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
        
        self.draw_sky();
        self.draw_signs();
        self.draw_ground();

        
        for i in 0..self.rbodies.len()
        {
            if !self.paused
            {
                self.rbodies[i].update(self.time);

                if self.time > Settings::TIME_GIVEN || self.rbodies.len() == 1 && self.rbodies[0].circles.iter().all(|c| c.on_floor)
                {
                    if !self.paused_before
                    {
                        self.paused_before = true;
                        self.paused = true;
                    }
                }
            }

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
            let y = Settings::FLOOR_Y - 250.0;
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
            draw_line(x + w / 2.0, Settings::FLOOR_Y, x + w / 2.0, y, 2.0, Color {r : 1.0, g : 1.0, b : 1.0, a : 0.4});
            draw_text(&(i * 200).to_string(), x + 10.0, y + 30.0, 40.0, BLACK);
        }
        for i in 0..(((self.bodies[self.gen][0].distance.unwrap() + screen_width() / 2.0) / 200.0).ceil() as usize + 1)
        {
            let y = Settings::FLOOR_Y - 250.0;
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
            draw_line(x + w / 2.0, Settings::FLOOR_Y, x + w / 2.0, y, 2.0, Color {r : 1.0, g : 1.0, b : 1.0, a : 0.4});
            draw_text(&(i * 200).to_string(), x + 10.0, y + 30.0, 40.0, BLACK);
        }
    }

    pub fn draw_ground(&mut self)
    {
        draw_rectangle(-screen_width() * 5.0 + self.bodies[self.gen][0].distance.unwrap(), 
            Settings::FLOOR_Y, 
            (screen_width() * 5.0 + self.bodies[self.gen][0].distance.unwrap()) * 2.0, 
            screen_height() - Settings::FLOOR_Y, 
            color_u8!(192.0, 255.0, 133.0, 255.0)
        );
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
                                ui.output_mut(|o| o.copied_text = self.seed.to_string());
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
                            }
                        }
                        
                        if self.show != show_before
                        {
                            self.update_rbodies();
                        }
                    });
                    
                    ui.collapsing("Year Info", |ui| {
                        
                        ui.label("Best dist ".to_string() + &self.bodies[self.gen][0].distance.unwrap().to_string());
                        ui.label("Mean dist ".to_string() + &(self.bodies[self.gen].iter().map(|i| i.distance.unwrap()).sum::<f32>() / self.bodies[self.gen].len() as f32).to_string());
                        ui.label("Median dist ".to_string() + &self.bodies[self.gen][self.bodies[self.gen].len() / 2].distance.unwrap().to_string());
                        ui.label("Worst dist ".to_string() + &self.bodies[self.gen].last().unwrap().distance.unwrap().to_string());
                    });
                    if self.show != ShowTypes::All
                    {
                        ui.collapsing("Viewing Creature Info", |ui| {
                            ui.label("Distance In Time ".to_string() + &self.rbodies[0].distance.unwrap().to_string());
                            ui.label("Time ".to_string() + &self.time.to_string());
                            ui.label("Distance ".to_string() + &self.rbodies[0].get_average_distance().to_string());
                            if ui.button("see family tree").clicked()
                            {
                                self.tree_bodies.clear();
                                self.screen = Screens::FamilyTree;
                                self.calc_family_tree()
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

    pub fn run_cam(&mut self)
    {
        let zoom = 0.0;
        let mut cam = Camera2D::from_display_rect(Rect::new(zoom, zoom, screen_width() - zoom, screen_height() - zoom));
        cam.target.x = self.rbodies[0].pos.x + self.rbodies[0].get_average_distance();
        cam.target.y = Settings::FLOOR_Y - 100.0;
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

    fn get_base_parent(&mut self, gen : usize, index : usize) -> (usize, usize)
    {
        if self.bodies[gen][index].parent.is_none()
        {
            return (gen, index);
        }
        return self.get_base_parent(self.bodies[gen][index].parent.unwrap().0, self.bodies[gen][index].parent.unwrap().1);
    }
    
    pub fn calc_family_tree(&mut self)
    {
        let base = self.get_base_parent(self.gen, self.custom_show - 1).1;
        // let mut current_row : Vec<(usize, usize)> = vec![];
        // current_row.push((0, base));
        self.tree_bodies.push(vec![]);
        let mut row_index = 0; // FROM THE TOP
        self.tree_bodies[row_index].push(TreeBody::from(self.bodies[0][base].clone(), (0, base)));

        self.tree_bodies.push(vec![]);
        for i in 0..self.tree_bodies[row_index].len()
        {
            if self.tree_bodies[row_index][i].children.len() == 0
            {
                continue;
            }
            let mut x = 0.0;

            for (gen, index) in self.tree_bodies[row_index][i].children.clone()
            {
                let mut body = TreeBody::from(self.bodies[gen][index].clone(), (gen, index));
                body.pos.x = x;
                body.pos.y = 400.0;
                let (pgen, pindex) = body.parent.unwrap();
                body.parent = Some(self.get_earliest(pgen, pindex));
                body.parent_tree_body = Some((row_index, i));
                self.tree_bodies[row_index + 1].push(body);
                x += 200.0;
            }
        }
        row_index += 1;

        loop
        {
            self.tree_bodies.push(vec![]);
            for i in 0..self.tree_bodies[row_index].len()
            {
                if self.tree_bodies[row_index][i].children.len() == 0
                {
                    continue;
                }

                let mut x = 0.0;

                for (gen, index) in self.tree_bodies[row_index][i].children.clone()
                {
                    let mut body = TreeBody::from(self.bodies[gen][index].clone(), (gen, index));
                    body.pos.x = x;
                    body.pos.y = 400.0;
                    let (pgen, pindex) = body.parent.unwrap();
                    body.parent = Some(self.get_earliest(pgen, pindex));
                    body.parent_tree_body = Some((row_index, i));
                    self.tree_bodies[row_index + 1].push(body);
                    //println!("{:?}", (gen, index));
                    if (gen, index) == (4, 29)
                    {
                        println!("AAAAA {x}");
                    }
                    x += 200.0;
                }
                for i in 0..self.tree_bodies[row_index].len()
                {
                    if i != self.tree_bodies[row_index].len() - 1
                    {
                        self.tree_bodies[row_index][i + 1].pos.x += x;
                    }
                }
            }
            if self.tree_bodies[row_index].len() == 0
            {
                break;
            }
            
            row_index += 1;
        }
        println!("{:?}", self.tree_bodies);

        
        // if row_index > 1
        // {
        //     self.tree_bodies[row_index].sort_by_key(|item| (item.parent.unwrap().0, item.parent.unwrap().1) );
        // }

        
    }

    fn get_parent_poses(&mut self, gen : usize, index : usize, pos : Point) -> Point
    {
        if self.tree_bodies[gen][index].parent_tree_body.is_none()
        {
            return pos
        }
        let (pgen, pindex) = self.tree_bodies[gen][index].parent_tree_body.unwrap();
        return self.get_parent_poses(pgen, pindex, pos + self.tree_bodies[pgen][pindex].pos);
    }

    pub fn draw_family_tree(&mut self)
    {
        clear_background(Color::from_rgba(249, 251, 231, 255));
        for i in 0..self.tree_bodies.len()
        {
            for j in 0..self.tree_bodies[i].len()
            {
                if self.tree_bodies[i][j].parent_tree_body.is_some()
                {
                    let (pgen, pindex) = self.tree_bodies[i][j].parent_tree_body.unwrap();
                    let par_pos = self.get_parent_poses(pgen, pindex, self.tree_bodies[pgen][pindex].pos);
                    self.tree_bodies[i][j].draw(par_pos);
                }
                else 
                {
                    self.tree_bodies[i][j].draw(Point {x : 0.0, y : 0.0});
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
        if is_key_down(KeyCode::S)
        {
            self.tree_cam_pos.y += 10.0;
        }
        if is_key_down(KeyCode::W)
        {
            self.tree_cam_pos.y -= 10.0;
        }
        if is_key_down(KeyCode::A)
        {
            self.tree_cam_pos.x -= 10.0;
        }
        if is_key_down(KeyCode::D)
        {
            self.tree_cam_pos.x += 10.0;
        }
        let zoom = -400.0;
        let mut cam = Camera2D::from_display_rect(Rect::new(zoom, zoom, screen_width() - zoom, screen_height() - zoom));
        cam.target.x = self.tree_cam_pos.x;
        cam.target.y = self.tree_cam_pos.y;
        set_camera(&cam);
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
    Stats,
}
