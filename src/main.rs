use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::atomic::{AtomicU32, Ordering};

// Pixels per frame tick (TODO: Make independent of monitor refresh rate)
const SPEED: i32 = 4;

// Milliseconds
const FPS_COUNTER_INTERVAL: u32 = 5000;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PLAYER_WIDTH: u32 = 32;
const PLAYER_HEIGHT: u32 = 32;

fn wrap_player(player: &mut Rect) {
    let sw = SCREEN_WIDTH as i32;
    let sh = SCREEN_HEIGHT as i32;
    player.x = if player.x < -player.w {
        player.x + player.w + sw
    } else if player.x > sw {
        -player.w
    } else {
        player.x
    };
    player.y = if player.y < -player.h {
        player.y + player.h + sh
    } else if player.y > sh {
        -player.h
    } else {
        player.y
    };
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let win = video_subsystem
        .window("Hello SDL2 from Rust", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = win
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let fps_count = AtomicU32::new(0);

    let timer_subsystem = sdl.timer().unwrap();
    let _fps_timer = timer_subsystem.add_timer(
        FPS_COUNTER_INTERVAL,
        Box::new(|| {
            let seconds = FPS_COUNTER_INTERVAL as f32 / 1000_f32;
            println!(
                "{:.2} FPS",
                fps_count.swap(0, Ordering::Acquire) as f32 / seconds
            );
            FPS_COUNTER_INTERVAL
        }),
    );

    let mut player = Rect::new(
        (SCREEN_WIDTH / 2 - PLAYER_WIDTH / 2) as i32,
        (SCREEN_HEIGHT / 2 - PLAYER_HEIGHT / 2) as i32,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    );

    let mut event_pump = sdl.event_pump().unwrap();
    'outer: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'outer,
                _ => (),
            }
        }

        for key in KeyboardState::new(&event_pump).pressed_scancodes() {
            match key {
                Scancode::Up => player.y -= SPEED,
                Scancode::Down => player.y += SPEED,
                Scancode::Left => player.x -= SPEED,
                Scancode::Right => player.x += SPEED,
                _ => (),
            }
        }

        wrap_player(&mut player);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.fill_rect(player);
        canvas.present();

        fps_count.fetch_add(1, Ordering::Release);

        // Needs `&mut timer_subsystem` to work but unable to because of `_fps_timer`
        //timer_subsystem.delay(16);
    }
}
