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
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn main() -> Result<(), String> {
    // size of the window
    let window_dim = 400;


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
    'game_loop: loop {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
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
