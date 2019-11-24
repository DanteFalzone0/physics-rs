/*          main.rs file of physics-rs
    Copyright (C) 2019  Dante Falzone

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::mouse::Cursor;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod particle;
use particle::Particle;

fn main() -> Result<(), String> {
    // size of the window
    let window_dim = 400;

    // number of paritcles
    let particle_ct = 256;

    let sdl_context = sdl2::init()?; // initialize the SDL stuff
    let video_subsystem = sdl_context.video()?;

    // Create an SDL window
    let window = video_subsystem.window(
        "Physics Simulation in Rust",
        window_dim,
        window_dim
    ).position_centered().opengl().build()
        .map_err(|e| e.to_string())?; // catch any errors

    // create a new SDL rendering context
    let mut renderer = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    // Game event loop
    let mut event_pump = sdl_context.event_pump()?;

    // Vectors of Particles
    let mut particles = vec![particle::new(); particle_ct];
    let mut prev_particles = vec![particle::new(); particle_ct];
    let mut mouse_particle = particle::new();

    for i in 0..particle_ct {
        particles[i].x_pos =
            window_dim as f32 / 2.0 + 20.0 * (i as f32).sin();
        particles[i].y_pos = 
            window_dim as f32 / 2.0 + 20.0 * (i as f32).cos();
    }

    'game_loop: loop {
        sdl_context.mouse().show_cursor(false);

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        for i in 0..particle_ct {
            prev_particles[i] = particles[i];
        }

        let mouse_info = event_pump.mouse_state();
        let prev_mouse_x = mouse_particle.x_pos;
        let prev_mouse_y = mouse_particle.y_pos;
        mouse_particle.x_pos = mouse_info.x() as f32;
        mouse_particle.y_pos = mouse_info.y() as f32;
        mouse_particle.x_momentum =
            mouse_particle.x_pos - prev_mouse_x;
        mouse_particle.y_momentum =
            mouse_particle.y_pos - prev_mouse_y;

        /* update the state of the particles */
        for i in 0..particle_ct {
            for j in 0..particle_ct {
                if i != j {
                    let x_diff =
                        particles[i].x_pos
                      - prev_particles[j].x_pos;

                    let y_diff =
                        particles[i].y_pos
                      - prev_particles[j].y_pos;

                    if x_diff.hypot(y_diff) <= 1.5 {
                        particles[i].collide_with(
                            prev_particles[j]
                        );
                    }
                }
            }

            /* collide with the mouse */
            let mouse_x_dist =
                particles[i].x_pos - mouse_particle.x_pos;
            let mouse_y_dist =
                particles[i].y_pos - mouse_particle.y_pos;
            if mouse_x_dist.hypot(mouse_y_dist) < 3.0 {
                particles[i].collide_with(mouse_particle);
            }

            /* do the physics lol */
            particles[i].update(
                mouse_particle.x_pos as i32,
                mouse_particle.y_pos as i32,
                0.002, // space drag
                0.2, // wall bounciness
                window_dim,
                window_dim,
                0.2 // gravity
            );
        }

        renderer.set_draw_color(Color::RGB(0, 255, 50));

        for i in 0..particle_ct {
            renderer.draw_point(
                Point::new(
                    particles[i].x_pos as i32,
                    particles[i].y_pos as i32
                )
            ).map_err(|e| e.to_string())?;
        }

        renderer.set_draw_color(Color::RGB(255, 10, 0));
        for x in -2_i32..3_i32 {
            for y in -2_i32..3_i32 {
                if x.abs() != y.abs() {
                    renderer.draw_point(
                        Point::new(
                            mouse_particle.x_pos as i32 + x,
                            mouse_particle.y_pos as i32 + y
                        )
                    ).map_err(|e| e.to_string())?;
                }
            }
        }

        renderer.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game_loop,

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if keycode == Keycode::X {
                        println!("Closing the program.");
                        break 'game_loop;
                    } else if keycode == Keycode::A {
                        println!("You pressed A!");
                    }
                },

                _ => {}
            }
        }
        ::std::thread::sleep(
            Duration::new(
                0,
                1_000_000_000u32 / 60
            )
        );
    }

    Ok(())
}
