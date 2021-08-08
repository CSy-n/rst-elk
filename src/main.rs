extern crate sdl2;
extern crate lua53_sys;
extern crate libc;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;

use sdl2::pixels::{Color, PixelFormatEnum};

use lua53_sys::{lauxlib, lua};

pub const HELLO: &'static [u8] = b"print(\"Hello\")\0";


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Lite", 640, 480)
        .resizable()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
//       .accelerated() // -- enables render-acceleration
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 0, 255, 255));

    let timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;

      unsafe {
        let state = lauxlib::luaL_newstate();
        lauxlib::luaL_openlibs(state);
        lauxlib::luaL_dostring(state, HELLO.as_ptr() as *const libc::c_char);
        lua::lua_close(state);
    }

//     // animation sheet and extras are available from
//     // https://opengameart.org/content/a-platformer-in-the-forest
//     let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
//     let texture = texture_creator
//         .create_texture_from_surface(&temp_surface)
//         .map_err(|e| e.to_string())?;

//     let frames_per_anim = 4;
//     let sprite_tile_size = (32, 32);

//     // Baby - walk animation
//     let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
//     let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
//     dest_rect_0.center_on(Point::new(-64, 120));

//     // King - walk animation
//     let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
//     let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
//     dest_rect_1.center_on(Point::new(0, 240));

//     // Soldier - walk animation
//     let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
//     let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
//     dest_rect_2.center_on(Point::new(440, 360));
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .map_err(|e| e.to_string())?;

  let mut tick = 0;
  let mut angle = 0.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!(
                "Window - pos({}x{}), size({}x{}): {}",
                position.0, position.1, size.0, size.1, tick
            );
            window.set_title(&title).map_err(|e| e.to_string())?;

            tick += 1;
        }
        angle = (angle + 0.5) % 360.;
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                texture_canvas
                    .fill_rect(Rect::new(0, 0, 400, 300))
                    .expect("could not fill rect");
            })
            .map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, 400, 300));


        canvas.clear();
        canvas.copy_ex(
            &texture,
            None,
            dst,
            angle,
            Some(Point::new(400, 300)),
            false,
            false,
        )?;

        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.present();
    }






//     let mut running = true;
//     while running {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => {
//                     running = false;
//                 }
//                 _ => {}
//             }
//         }
//     }
//         let ticks = timer.ticks() as i32;

//         // set the current frame for time
//         source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
//         dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

//         source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
//         dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

//         source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
//         dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

//         canvas.clear();
//         // copy the frame to the canvas
//         canvas.copy_ex(
//             &texture,
//             Some(source_rect_0),
//             Some(dest_rect_0),
//             0.0,
//             None,
//             false,
//             false,
//         )?;
//         canvas.copy_ex(
//             &texture,
//             Some(source_rect_1),
//             Some(dest_rect_1),
//             0.0,
//             None,
//             true,
//             false,
//         )?;
//         canvas.copy_ex(
//             &texture,
//             Some(source_rect_2),
//             Some(dest_rect_2),
//             0.0,
//             None,
//             false,
//             false,
//         )?;
//         canvas.present();

//         std::thread::sleep(Duration::from_millis(100));
//     }

    Ok(())
}


// static fn get_exe_filename(char *buf, int sz) {


// }

