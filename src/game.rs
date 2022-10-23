extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
//use sdl2::render::TextureQuery;

use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use crate::scene::SDLDrawable;
use crate::scene::Tickable;

pub struct Galaxy {
    particles: Vec<Particle>,
    width: u32,
    height: u32,
}

impl Galaxy {
    /// Initialise the state of several particles
    pub fn new(width: u32, height: u32) -> Galaxy {
        let mut rng = thread_rng();
        const NUM_PARTICLES: usize = 6;
        const MAX_SINGLE_AXIS_VEL: f64 = 0.0001; //100.0;

        let mut particles: Vec<Particle> = Vec::new();

        for _ in 0..NUM_PARTICLES {
            particles.push(Particle {
                position: Point2D {
                    x: f64::from(rng.gen_range(100, width - 100)),
                    y: f64::from(rng.gen_range(100, height - 100)),
                },
                velocity: Point2D {
                    x: f64::from(rng.gen_range(-MAX_SINGLE_AXIS_VEL, MAX_SINGLE_AXIS_VEL)),
                    y: f64::from(rng.gen_range(-MAX_SINGLE_AXIS_VEL, MAX_SINGLE_AXIS_VEL)),
                },
            })
        }

        Self {
            particles,
            width,
            height,
        }
    }
}

impl SDLDrawable for Galaxy {
    /// Draw a given set of particles onto a canvas
    /// TODO: No idea about the mutability etc here
    fn draw(&self, canvas: &mut Canvas<Window>, particle_size: u32, colour: Color) {
        canvas.set_draw_color(colour);
        for cur_particle in &self.particles {
            //TODO: These should be some sort of 2D object
            let _result = canvas.fill_rect(Rect::new(
                cur_particle.position.x as i32,
                cur_particle.position.y as i32,
                particle_size,
                particle_size,
            ));
        }
    }
}

impl Tickable for Galaxy {
    /// Update particle velocity
    /// x_{t+1} = x_{t} + dt * (vel + dt* accel)
    fn push_forward(&mut self, time_step: f64, particle_mass: f64) {
        const MAX_ABS_ACCEL: f64 = 200.0;

        let pre_loop_galaxy = self.particles.clone();
        for particle in self.particles.iter_mut() {
            let mut cur_acc_x: f64 = 0.0;
            let mut cur_acc_y: f64 = 0.0;

            for i in 0..pre_loop_galaxy.len() {
                let mut add_accel = calc_gravitational_force(
                    particle.position,
                    pre_loop_galaxy[i].position,
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

            particle.velocity.x += time_step * cur_acc_x;
            particle.velocity.y += time_step * cur_acc_y;

            let likely_x = particle.position.x + time_step * particle.velocity.x;
            let likely_y = particle.position.y + time_step * particle.velocity.y;

            // Bounce off walls
            if likely_x < 0.0 || likely_x > f64::from(self.width) {
                particle.velocity.x = -particle.velocity.x
            } else {
                particle.position.x = likely_x
            }

            if likely_y < 0.0 || likely_y > f64::from(self.height) {
                particle.velocity.y = -particle.velocity.y
            } else {
                particle.position.y = likely_y
            }
        }
    }
}

/// Calculate the gravitational force from the first to the second of two particles
fn calc_gravitational_force(pos_first: Point2D, pos_second: Point2D, mass: f64) -> [f64; 2] {
    let gravitational_constant: f64 = 6.67430 * 10.0f64.powf(-11.0);

    let rel_x = pos_second.x - pos_first.x;
    let rel_y = pos_second.y - pos_first.y;
    let dist_sq = rel_x.powf(2.0) + rel_y.powf(2.0);
    let dist = dist_sq.sqrt();

    if dist_sq == 0.0 {
        return [0.0, 0.0];
    }
    let orth_force_magnitude = gravitational_constant * mass.powf(2.0) / dist_sq / dist;

    return [orth_force_magnitude * rel_x, orth_force_magnitude * rel_y];
}

pub struct Game {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
    galaxy: Galaxy,
}

impl Game {
    pub fn new() -> Result<Self, anyhow::Error> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Figure out the maximum screen size (note that on web this will be the screen, no the
        // viewport of the browser
        let cur_disp_mode = video_subsystem.current_display_mode(0).unwrap();

        let window = video_subsystem
            .window("Galactic Gravity", (cur_disp_mode.w as u32)*7/10, (cur_disp_mode.h as u32)*7/10)
            .position_centered()
            .build()
            .unwrap();
        let (window_width, window_height) = window.size();
       
        // Make a canvas to draw to
        let canvas = window.into_canvas().build().unwrap();

        // Make a galaxy the size of the window
        let galaxy = Galaxy::new(window_width, window_height);

        Ok(Self {
            canvas,
            galaxy,
            sdl_context,
        })
    }
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        const PARTICLE_COLOUR: Color = Color::RGB(255, 255, 255);
        const PARTICLE_SIZE: u32 = 2;
        let particle_mass: f64 = 1.0 * 10f64.powf(7.5);

        const TARGET_FPS: u32 = 60;
        let time_step: f64 = 1.0 / f64::from(TARGET_FPS as f64);

        // Process any events (keyboard etc.)
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    let half_num_to_add = 2;
                    for i in -half_num_to_add..half_num_to_add {
                        self.galaxy.particles.push(Particle {
                            position: Point2D {
                                x: f64::from(x + i),
                                y: f64::from(y + i),
                            },
                            velocity: Point2D { x: 0.0, y: 0.0 },
                        })
                    }
                }
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(x,y)  => {
                            println!("Resize {} {}", x, y);
                        }
                        _ => {}
                    }
                }
                
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return emscripten_main_loop::MainLoopEvent::Terminate,
                _ => {}
            }
        }

        // Push elements forwards in time
        //TODO: mass definitely shouldn't be here
        self.galaxy.push_forward(time_step, particle_mass);

        // Clear the background
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Draw scene objects
        self.galaxy.draw(&mut self.canvas, PARTICLE_SIZE, PARTICLE_COLOUR);
        //self.canvas.present();


        //const FONT_PATH: &str = "./Swansea-q3pd.ttf";
        //const FONT_PATH: &str = "/usr/share/fonts/TTF/DejaVuSans.ttf";
        //let texture_creator = self.canvas.texture_creator();
        //let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        //let font = ttf_context.load_font(FONT_PATH, 30).unwrap();
        ////font.set_style(sdl2::ttf::FontStyle::BOLD);

        //// render a surface, and convert it to a texture bound to the canvas
        //let surface = font
        //    .render("Hello Rust!")
        //    .blended(Color::RGBA(255, 0, 0, 255))
        //    .map_err(|e| e.to_string())
        //    .unwrap();
        //let texture = texture_creator
        //    .create_texture_from_surface(&surface)
        //    .map_err(|e| e.to_string())
        //    .unwrap();

        //self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        ////self.canvas.clear();

        //let TextureQuery { width, height, .. } = texture.query();

        //let target = Rect::new(100, 100, width, height);

        //let _result = self.canvas.copy(&texture, None, Some(target));

        self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        let (width, height) = self.canvas.output_size().unwrap();
        //let _result = self.canvas.fill_rect(Rect::new(
        //        100,
        //        100,
        //        width/10,
        //        height/10,
        //    ));

        let _result = self.canvas.draw_rect(Rect::new(
                0,
                0,
                width,
                height,
            ));
        
        self.canvas.present();



        // Wait for next tick
        // TODO: This isn't true FPS since we're not taking into account calculation time
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
        emscripten_main_loop::MainLoopEvent::Continue
    }
}

//TODO: Don't think I really want to be using these copies and clones
#[derive(Clone, Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Particle {
    position: Point2D,
    velocity: Point2D,
}
