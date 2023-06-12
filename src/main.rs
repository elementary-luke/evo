//use normal_rand as rand;
mod muscle;
mod circle;
mod point;
mod force;
mod body;
mod settings;
mod ecosystem;
mod tree_body;
mod display_body;

use crate::ecosystem::*;
use macroquad::color::*;
use macroquad::color_u8;
use macroquad::window::*;
use egui_macroquad::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "My game".to_owned(),
        //fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
    
}

#[macroquad::main(window_conf())]
async fn main() {
   let mut sys = Ecosystem::new();
    sys.initialise();
    loop {
        match sys.screen
        {
            Screens::Simulation => {
                sys.run_view();
                sys.run_gui();
                sys.run_cam();
            },
            Screens::FamilyTree => {
                sys.draw_family_tree();
                sys.family_tree_gui();
                sys.family_tree_cam();
            },
            Screens::GenerationDisplay => {
                sys.draw_generation_display();
                sys.generation_display_gui();
                sys.generation_display_cam();
            },
            _ => {
                println!("sum ting wong 46 main.rs");
            }
        }
        
        next_frame().await
    }
}