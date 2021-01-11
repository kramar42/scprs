#![windows_subsystem = "windows"]
#![allow(dead_code)]

use nannou::prelude::*;

mod r201;
use self::r201::R201;

mod ga;
mod draw;

const MIN_SIZE: f64 = 3.;
const MOUSE_SCALE: f64 = 100.;
const COLOR_SCALE: f64 = 100.;

fn main() {
    nannou::app(model)
        .event(event)
        .run();
}

pub struct Model {
    o: R201,
    a: R201,
    m: R201,
    s: f64
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1280, 720)
        .view(view)
        .build()
        .unwrap();
    // our center
    let o = ga::point(0., -120.);
    // top point
    let a = ga::point(0., 380.);

    let mut res = Model {
        o: o,
        a: a,
        m: R201::zero(),
        s: 0.
    };
    update_model(&mut res);
    res
}

/// update derived values
fn update_model(m: &mut Model) {
    if m.s < 0. {
        m.s = 0.;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    app.set_loop_mode(LoopMode::wait());

    let draw = app.draw();
    draw.background().color(BLACK);

    let y = &model.a - &model.m;
    draw::triangle(&draw, model, &model.m, &y, model.s / MOUSE_SCALE);
    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    //println!("got event: {:?}", &event);
    match event {
        Event::WindowEvent { simple: Some(event), .. } => match event {
            MouseMoved(coords) => {
                model.m = ga::point(coords.x.into(), coords.y.into());
            }
            MouseWheel(delta, _) => {
                match delta {
                    MouseScrollDelta::LineDelta(_h, v) => {
                        model.s += v as f64;
                    }
                    MouseScrollDelta::PixelDelta(p) => {
                        model.s += p.y as f64;
                    }
                }
            }
            _ => () } _ => ()
    }
    update_model(model);
}
