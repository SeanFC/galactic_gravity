extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width: u32 = 800;
    let window_height: u32 = 600;

    let window = video_subsystem.window("GALACTIC GRAVITY", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut rng = thread_rng();

    const NUM_PARTICLES: usize = 200;
    const MAX_SINGLE_AXIS_VEL: f64 = 100.0;
    let mut particle_position: [[f64; 2]; NUM_PARTICLES] = [[0.0,0.0]; 200];
    let mut particle_velocity: [[f64; 2]; NUM_PARTICLES] = [[0.0,0.0]; 200];
    let mut particle_acceleration: [[f64; 2]; NUM_PARTICLES] = [[0.0,0.0]; 200];
    for i in 0..NUM_PARTICLES {
        particle_position[i] = [f64::from(rng.gen_range(0, window_width)), f64::from(rng.gen_range(0, window_width))];
        particle_velocity[i] = [f64::from(rng.gen_range(-MAX_SINGLE_AXIS_VEL, MAX_SINGLE_AXIS_VEL)), f64::from(rng.gen_range(-MAX_SINGLE_AXIS_VEL, MAX_SINGLE_AXIS_VEL))];
    }

    const PARTICLE_SIZE: u32 = 4;
    const PARTICLE_COLOUR: Color = Color::RGB(255, 255, 255);
       
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    const TARGET_FPS: u32 = 60;
    let time_step: f64 = 1.0/f64::from(TARGET_FPS as f64);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Update particle velocity as x_1 = x_0 + dt * (vel + dt* accel)
        for (cur_pos, cur_vel) in particle_position.iter_mut().zip(particle_velocity.iter_mut()) {
            

            let likely_x = cur_pos[0] + time_step *cur_vel[0];
            let likely_y = cur_pos[1] + time_step *cur_vel[1];

            // Bounce off walls
            if likely_x < 0.0 || likely_x > f64::from(window_width) {
                cur_vel[0] = -cur_vel[0]
            } else {
                cur_pos[0] = likely_x
            }

            if likely_y < 0.0 || likely_y > f64::from(window_height) {
                cur_vel[1] = -cur_vel[1]
            } else {
                cur_pos[1] = likely_y
            }
        }

        // Clear the frame
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the particles
        canvas.set_draw_color(PARTICLE_COLOUR);
        for cur_pos in particle_position {
            //TODO: These should be some sort of 2D object
            let x = cur_pos[0];
            let y = cur_pos[1];
            let _result = canvas.fill_rect(Rect::new(x as i32, y as i32, PARTICLE_SIZE, PARTICLE_SIZE));
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
