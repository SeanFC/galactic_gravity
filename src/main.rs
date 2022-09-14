extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

struct Point2D {
    x : f64,
    y : f64,
}

struct Particle {
    position: Point2D,
    velocity : Point2D,
}

/// Initialise the state of several particles
fn initialise_particles(window_width: u32, window_height: u32) -> Vec<Particle> {
    let mut rng = thread_rng();
    const NUM_PARTICLES: usize = 6;
    const MAX_SINGLE_AXIS_VEL: f64 = 0.0001; //100.0;

    let mut galaxy: <Particle> = Vec::new();

    for i in 0..NUM_PARTICLES {
        galaxy.push(
            Particle(
                position:Point2D(
                    x:f64::from(rng.gen_range(100, window_width - 100)), 
                    y:f64::from(rng.gen_range(100, window_width - 100)),
                    ), 
                velocity:Point2D(
                    x:f64::from(rng.gen_range(-MAX_SINGLE_AXIS_VEL, MAX_SINGLE_AXIS_VEL)), 
                    y:f64::from(rng.gen_range(-MAX_SINGLE_AXIS_VEL, MAX_SINGLE_AXIS_VEL)),
                    )
                )
            )
            
    }

    galaxy
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width: u32 = 800;
    let window_height: u32 = 600;

    let window = video_subsystem
        .window("GALACTIC GRAVITY", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    const PARTICLE_SIZE: u32 = 4;
    const PARTICLE_COLOUR: Color = Color::RGB(255, 255, 255);
    const MAX_ABS_ACCEL: f64 = 200.0;
    let particle_mass: f64 = 1.0 * 10f64.powf(15.0);
    let gravitational_constant: f64 = 6.67430 * 10.0f64.powf(-11.0);

    let (mut particle_position, mut particle_velocity) =
        initialise_particles(window_width, window_height);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    const TARGET_FPS: u32 = 60;
    let time_step: f64 = 1.0 / f64::from(TARGET_FPS as f64);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    let half_num_to_add = 2;
                    for i in -half_num_to_add..half_num_to_add {
                        particle_position.push([f64::from(x + i), f64::from(y + i)]);
                        particle_velocity.push([0.0, 0.0]);
                    }
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let pre_loop_particle_position = particle_position.clone();

        // Update particle velocity as x_1 = x_0 + dt * (vel + dt* accel)
        for (cur_pos, cur_vel) in particle_position
            .iter_mut()
            .zip(particle_velocity.iter_mut())
        {
            let mut cur_acc_x: f64 = 0.0;
            let mut cur_acc_y: f64 = 0.0;

            for i in 0..pre_loop_particle_position.len() {
                let mut add_accel = calc_gravitational_force(
                    gravitational_constant,
                    *cur_pos,
                    pre_loop_particle_position[i],
                    particle_mass,
                );

                if add_accel[0].abs() > MAX_ABS_ACCEL {
                    add_accel[0] = add_accel[0] / add_accel[0].abs() * MAX_ABS_ACCEL
                }
                if add_accel[1].abs() > MAX_ABS_ACCEL {
                    add_accel[1] = add_accel[1] / add_accel[1].abs() * MAX_ABS_ACCEL
                }

                cur_acc_x += add_accel[0];
                cur_acc_y += add_accel[1];
            }

            cur_vel[0] += time_step * cur_acc_x;
            cur_vel[1] += time_step * cur_acc_y;

            let likely_x = cur_pos[0] + time_step * cur_vel[0];
            let likely_y = cur_pos[1] + time_step * cur_vel[1];

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

        // Draw
        draw_particles(
            &mut canvas,
            &particle_position,
            PARTICLE_SIZE,
            PARTICLE_COLOUR,
        );

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}

/// Draw a given set of particles onto a canvas
/// TODO: No idea about the mutability etc here
fn draw_particles(
    canvas: &mut Canvas<Window>,
    positions: &Vec<[f64; 2]>,
    particle_size: u32,
    colour: Color,
) {
    canvas.set_draw_color(colour);
    for cur_pos in positions {
        //TODO: These should be some sort of 2D object
        let _result = canvas.fill_rect(Rect::new(
            cur_pos[0] as i32,
            cur_pos[1] as i32,
            particle_size,
            particle_size,
        ));
    }
}

fn calc_gravitational_force(
    gravitational_constant: f64,
    pos_first: [f64; 2],
    pos_second: [f64; 2],
    mass: f64,
) -> [f64; 2] {
    let rel_x = pos_second[0] - pos_first[0];
    let rel_y = pos_second[1] - pos_first[1];
    let dist_sq = rel_x.powf(2.0) + rel_y.powf(2.0);
    let dist = dist_sq.sqrt();

    if dist_sq == 0.0 {
        return [0.0, 0.0];
    }
    let orth_force_magnitude = gravitational_constant * mass / dist_sq / dist;

    return [orth_force_magnitude * rel_x, orth_force_magnitude * rel_y];
}
