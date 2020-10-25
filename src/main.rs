extern crate sdl2; 

mod player;
mod velocity;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::gfx::framerate::FPSManager;
use std::collections::HashMap;
use player::Player;
use velocity::Velocity;

// Constants
const SCREEN_WIDTH:u32 = 800;
const SCREEN_HEIGHT:u32 = 600;
const FPS:u32 = 60;
const MOVE_SPEED:f32 = 5.0;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    // Main Window
    let window = video_subsystem.window("Rust SDL2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    // Canvas of Main Window
    let mut canvas = window.into_canvas().build().unwrap();

    // FPS manager Initializing
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(FPS).unwrap();

    // Texture Creator
    let player_image = include_bytes!("../cuddlyferris-in-game.png");
    let texture_creator = canvas.texture_creator();

    // Create Texture and Send to Player Struct
    let player_texture = texture_creator.load_texture_bytes(player_image).unwrap();
    let mut player: Player<'_> = Player{
        src_rect: None,
        dst_rect: sdl2::rect::Rect::new(0, 0, 0, 0),
        texture: player_texture
    };

    // Set Player Object Size
    player.dst_rect = Player::set_dst_rect(
        (SCREEN_WIDTH / 2 - 100 / 2) as i32,
        (SCREEN_HEIGHT / 2 - 74 / 2) as i32,
        100,
         74
    );

    // Movements Velocity
    let mut velocity = Velocity{ x: 0.0, y: 0.0 };

    let mut keypressed = HashMap::new();
    keypressed.insert("Right", false);
    keypressed.insert("Left", false);
    keypressed.insert("Up", false);
    keypressed.insert("Down", false);

    // Init Window with White Screen;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Clearing Screens
        canvas.clear();

        // Polling Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // TODO: Smoother way to move Crab
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { keypressed.insert("Right", true); },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => { keypressed.insert("Right", false); }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => { keypressed.insert("Left", true); },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => { keypressed.insert("Left", false); }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { keypressed.insert("Up", true); },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => { keypressed.insert("Up", false); }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { keypressed.insert("Down", true); },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => { keypressed.insert("Down", false); }
                _ => {}
            }
        }
        movements(&mut keypressed, &mut velocity, &mut player);

        // The rest of the game loop goes here...

        // Draw Texture
        canvas.copy(&player.texture, player.src_rect, player.dst_rect).unwrap();

        canvas.present();

        fps_manager.delay();
    }
}

fn movements(keypressed: &mut HashMap<&str, bool>, velocity: &mut Velocity, player: &mut Player) {
    let right_key = Some(keypressed.get("Right"));
    let left_key = Some(keypressed.get("Left"));
    let up_key = Some(keypressed.get("Up"));
    let down_key = Some(keypressed.get("Down"));

    // Horizontal Key Down Events
    if right_key.unwrap() == Some(&true) {
        velocity.x = MOVE_SPEED;
    } else if left_key.unwrap() == Some(&true) {
        velocity.x = -MOVE_SPEED;
    }else if right_key.unwrap() == Some(&false) {
        velocity.x = 0.0;
    } else if left_key.unwrap() == Some(&false) {
        velocity.x = 0.0;
    }

    // Vertical Key Down Events
    if up_key.unwrap() == Some(&true) {
        velocity.y = -MOVE_SPEED;
    } else if down_key.unwrap() == Some(&true) {
        velocity.y = MOVE_SPEED;
    } else if up_key.unwrap() == Some(&false) {
        velocity.y = 0.0;
    } else if down_key.unwrap() == Some(&false) {
        velocity.y = 0.0;
    }

    if velocity.x != 0.0 && velocity.y != 0.0 {
        player.dst_rect.x += (velocity.x / 1.414213) as i32;
        player.dst_rect.y += (velocity.y / 1.414213) as i32;
    } else {
        player.dst_rect.x += velocity.x as i32;
        player.dst_rect.y += velocity.y as i32;
    }
}