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
    count: u32,
    stepsize: usize,
}

fn model(app: &App) -> Model {
    let _win = app
        .new_window()
        .size(300, 300)
        .view(record)
        //.view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        seed: 42,
        count: 10,
        stepsize: 70,
    }
}

fn from_polar(angle: f32, radius: f32) -> Vec2 {
    let angle = deg_to_rad(angle);
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    pt2(x, y)
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
    let win = app.window_rect();

    draw.background().color(BLACK);

    let noise = OpenSimplex::new().set_seed(model.seed);

    for radius in (20..win.w() as i64 / 2).step_by(model.stepsize) {
        let mut points: Vec<_> = (0..model.count)
            .map(|i| {
                let angle = map_range(i, 0, model.count, 0, 360);

                let frequency = app.mouse.y as f64 / 20.;

                let random_distance = noise.get([
                    frequency * map_range(deg_to_rad(angle as f64).sin(), -1., 1., 0., 1.),
                    frequency * map_range(deg_to_rad(angle as f64).cos(), -1., 1., 0., 1.),
                    app.elapsed_frames() as f64 / 10. / (radius as f64).sqrt().sqrt(),
                ]) as f32;

                let distance = radius as f32
                    + (2. * app.mouse.x / win.w()) * random_distance * model.stepsize as f32
                        / (radius as f32).sqrt().sqrt().sqrt()
                        * 5.;

                let xy = from_polar(angle as f32, distance);

                (
                    xy,
                    hsv(map_range(distance, 0., win.right(), 0., 1.), 1., 1.),
                )
            })
            .collect();
        points.push(points[0]);

        draw.polyline()
            .rotate(app.elapsed_frames() as f32 / radius as f32)
            .weight(5.)
            .points_colored(points);
    }

    draw.to_frame(app, &frame).unwrap()
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Up => model.stepsize += 10,
        Key::Down => model.stepsize -= 10,
        Key::Right => model.count += 1,
        Key::Left => model.count -= 1,
        Key::S => unsafe {
            RECORDING = true;
        },
        _ => (),
    }
}
