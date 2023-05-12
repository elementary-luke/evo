//use normal_rand as rand;
mod muscle;
mod circle;
mod point;
mod force;
mod body;
mod settings;
mod ecosystem;

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
        clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));
        sys.run_view();
        sys.run_gui();
        sys.cam();
        next_frame().await
    }
}