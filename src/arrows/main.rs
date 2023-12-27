extern crate nannou;

use std::process::exit;

use nannou::{
    noise::{NoiseFn, OpenSimplex, Seedable},
    prelude::*,
};

fn main() {
    nannou::app(model).run();
}

struct Model {
    seed: u32,
    grid_step_size: usize,
    noise_scale: f64
}

fn model(app: &App) -> Model {
    let _win = app
        .new_window()
        .size(300, 300)
        .key_pressed(key_pressed)
        .view(record)
        .build()
        .unwrap();

    Model {
        seed: 42,
        grid_step_size: 30,
        noise_scale: 1.,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Up => model.noise_scale += 0.1,
        Key::Down => model.noise_scale -= 0.1,
        Key::Left => model.grid_step_size -= 1,
        Key::Right => model.grid_step_size += 1,
        Key::S => unsafe {RECORDING = true},
        _ => (),
    }
    
}

static mut RECORDING: bool = false;
static mut FRAME_COUNT: i32 = 500;

fn record(app: &App, model: &Model, frame: Frame) {
    view(app, model, frame);
    unsafe {
        if RECORDING {
            app.main_window()
                .capture_frame(format!("{FRAME_COUNT:03}") + ".png");
            if FRAME_COUNT < 0 {
                exit(0);
            }
            FRAME_COUNT -= 1;
        }
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect().pad(20.);

    draw.background().color(BLACK);

    let noise = OpenSimplex::new().set_seed(model.seed);

    for x in (win.left() as i32 ..= win.right() as i32).step_by(model.grid_step_size) {
        for y in (win.bottom() as i32 ..= win.top() as i32).step_by(model.grid_step_size) {
            let nx = map_range(x as f64, win.left() as f64, win.right() as f64, 0., 1.);
            let ny = map_range(y as f64, win.bottom() as f64, win.top() as f64, 0., 1.);
            let n = noise.get([nx * model.noise_scale, ny * model.noise_scale, app.elapsed_frames() as f64 / 100.]) as f32 * 8.;
            draw.translate([x as f32,y as f32,0.].into())
                .arrow()
                .start([model.grid_step_size as f32 * -0.5,0.].into())
                .end([model.grid_step_size as f32 * 0.5, 0.].into())
                .rotate(n)
                .gray(1.0);
        }
    }
    draw.to_frame(app, &frame).unwrap()
}


