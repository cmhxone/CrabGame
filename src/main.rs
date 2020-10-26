extern crate sdl2; 

mod entity;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::gfx::framerate::FPSManager;
use sdl2::rect::Rect;
use std::collections::HashMap;
use entity::player::Player;
use entity::velocity::Velocity;
use entity::item::Item;

// Constants
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;
const PLAYER_WIDTH: u32 = 160;
const PLAYER_HEIGHT: u32 = 84;
const FPS: u32 = 60;
const MOVE_SPEED: f32 = 5.0;

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
    let coin_image = include_bytes!("../coin.png");
    let texture_creator = canvas.texture_creator();

    // Create Texture and Send to Player Struct
    let player_texture = texture_creator.load_texture_bytes(player_image).unwrap();
    let mut player: Player<'_> = Player{
        src_rect: None,
        dst_rect: Some(Rect::new(0, 0, 0, 0)),
        texture: Some(player_texture),
        velocity: Some(Velocity{x: 0.0, y: 0.0}),
    };

    // Set Player Sprite Size
    player.src_rect = Some(Player::set_rect(
        PLAYER_WIDTH as i32 * 0,
        PLAYER_HEIGHT as i32 * 0,
        PLAYER_WIDTH,
        PLAYER_HEIGHT
    ));

    // Set Player Object Size
    player.dst_rect = Some(Player::set_rect(
        (SCREEN_WIDTH / 2 - PLAYER_WIDTH / 2) as i32,
        (SCREEN_HEIGHT / 2 - PLAYER_HEIGHT / 2) as i32,
        PLAYER_WIDTH,
        PLAYER_HEIGHT
    ));

    // Init Coin
    let mut coin_count = 20;
    let mut coins: Vec<Item<'_>> = Vec::with_capacity(coin_count);
    for i in 0 .. coin_count {
        coins.insert(i, 
            Item{
                src_rect: Some(Rect::new(0, 0, 50, 50)),
                dst_rect: Some(Rect::new(rand::thread_rng().gen_range(0, SCREEN_WIDTH - 50) as i32, rand::thread_rng().gen_range(0, 720 - 50) as i32, 50, 50)),
                texture: Some(texture_creator.load_texture_bytes(coin_image).unwrap())
            }
        );
    }

    // Init Keypresses
    let mut keypressed = HashMap::new();
    keypressed.insert("Right", false);
    keypressed.insert("Left", false);
    keypressed.insert("Up", false);
    keypressed.insert("Down", false);

    // Init Window with White Screen;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut frame = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        frame += 1;

        // Clearing Screens
        canvas.clear();

        // Polling Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
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
        // Move Player
        player_movements(&mut keypressed, &mut player);

        // The rest of the game loop goes here...
        for coin in &mut coins {
            coin.src_rect = Some(Rect::new((frame / 2 % 3) * 50, 0, 50, 50));
            canvas.copy(coin.texture.as_ref().unwrap(), coin.src_rect, coin.dst_rect).unwrap();
        }

        if (coin_count > 0) {
            coins.get_mut(coin_count - 1).unwrap().src_rect = Some(Rect::new((frame / 2 % 3) * 50, 50, 50, 50));
            canvas.copy(coins.get(coin_count -1).unwrap().texture.as_ref().unwrap(), coins.get(coin_count-1).unwrap().src_rect, coins.get(coin_count-1).unwrap().dst_rect).unwrap();

            if player.is_collide(&coins.get(coin_count - 1).unwrap()) {
                coins.pop();
                coin_count -= 1;
            }
        }

        // Draw Texture
        canvas.copy(&player.texture.as_ref().unwrap(), player.src_rect, player.dst_rect).unwrap();

        canvas.present();

        fps_manager.delay();
    }
}

// Player Movements
fn player_movements(keypressed: &HashMap<&str, bool>, player: &mut Player) {
    let right_key = Some(keypressed.get("Right"));
    let left_key = Some(keypressed.get("Left"));
    let up_key = Some(keypressed.get("Up"));
    let down_key = Some(keypressed.get("Down"));

    // Horizontal Key Down Events
    if right_key.unwrap() == Some(&true) {
        player.velocity = Some(Velocity::new(MOVE_SPEED, player.velocity.as_ref().unwrap().y));
        player.src_rect = Some(Player::set_rect(
            PLAYER_WIDTH as i32 * 0,
            PLAYER_HEIGHT as i32 * 0,
            PLAYER_WIDTH,
            PLAYER_HEIGHT
        ));
    } else if left_key.unwrap() == Some(&true) {
        player.velocity = Some(Velocity::new(-MOVE_SPEED, player.velocity.as_ref().unwrap().y));
        player.src_rect = Some(Player::set_rect(
            PLAYER_WIDTH as i32 * 1,
            PLAYER_HEIGHT as i32 * 0,
            PLAYER_WIDTH,
            PLAYER_HEIGHT
        ));
    }else if right_key.unwrap() == Some(&false) {
        player.velocity = Some(Velocity::new(0.0, player.velocity.as_ref().unwrap().y));
    } else if left_key.unwrap() == Some(&false) {
        player.velocity = Some(Velocity::new(0.0, player.velocity.as_ref().unwrap().y));
    }

    // Vertical Key Down Events
    if up_key.unwrap() == Some(&true) {
        player.velocity = Some(Velocity::new(player.velocity.as_ref().unwrap().x, -MOVE_SPEED));
    } else if down_key.unwrap() == Some(&true) {
        player.velocity = Some(Velocity::new(player.velocity.as_ref().unwrap().x, MOVE_SPEED));
    }else if up_key.unwrap() == Some(&false) {
        player.velocity = Some(Velocity::new(player.velocity.as_ref().unwrap().x, 0.0));
    } else if down_key.unwrap() == Some(&false) {
        player.velocity = Some(Velocity::new(player.velocity.as_ref().unwrap().x, 0.0));
    }

    // Movements Edit
    if player.velocity.as_ref().unwrap().x != 0.0 && player.velocity.as_ref().unwrap().y != 0.0 {
        player.dst_rect = Some(Player::set_rect(
            player.dst_rect.unwrap().x() + (player.velocity.as_ref().unwrap().x / 1.414213) as i32,
            player.dst_rect.unwrap().y() + (player.velocity.as_ref().unwrap().y / 1.414213) as i32,
            player.dst_rect.unwrap().width(),
            player.dst_rect.unwrap().height(),
        ));
    } else {
        player.dst_rect = Some(Player::set_rect(
            player.dst_rect.unwrap().x() + player.velocity.as_ref().unwrap().x as i32,
            player.dst_rect.unwrap().y() + player.velocity.as_ref().unwrap().y as i32,
            player.dst_rect.unwrap().width(),
            player.dst_rect.unwrap().height(),
        ));
    }    
}
