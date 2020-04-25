#![windows_subsystem = "windows"]
#![allow(dead_code)]

/// в центре статическая картинка
/// передвижение мышки по четырем углам окна передвигает нас по плоскости параметров
/// зажатие и движение меняет обзорный вектор

use nannou::prelude::*;
use nannou::draw::Draw;
use nannou::rand::{Rng, SeedableRng, rngs::StdRng};

mod r201;
use self::r201::R201;

mod ga;
mod draw;

//const POINT_SIZE: f32 = 2.;
//const TRIANGLE_SIZE: f64 = 250.;
//const MIN_SIZE: f64 = 3.; //0.7;
const MOUSE_SCALE: f64 = 100.;
const COLOR_SCALE: f64 = 100.;

fn main() {
    nannou::app(model)
        .event(event)
        .run();
}

struct Model {
    a: R201,
    b: R201,
    c: R201,
    m: R201,
    s: f64,
    t: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1280, 720)
        .view(view)
        .build()
        .unwrap();
    // our center
    let o = ga::point(0., -120.);
    // clockwise rotation
    let r = ga::rotator(&o, (PI/3.).into());
    // top point
    let a = ga::point(0., 380.);
    // left point
    let b = ga::mot(&r, &a);
    // right point
    let c = ga::mot(&r, &b);
    let mut res = Model {
        a: a,
        b: b,
        c: c,
        m: R201::zero(),
        s: 0.,
        t: 0.,
    };
    update_model(&mut res);
    res
}

/// update derived values
fn update_model(m: &mut Model) {
    if m.s < 0. {
        m.s = 0.;
    }
    m.t = m.s * MOUSE_SCALE;
}

fn triangle(draw: &Draw, m: &Model,
            x: &R201, t: f64, f: &mut impl FnMut(&R201) -> R201) {
    let mut x: R201 = x.clone();
    let mut t = t;
    while t > 0. {
        draw::point(draw, &x, (m.t - t) / COLOR_SCALE, 2.);
        x = f(&x);
        t = t - 1.;
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let s = [42; 32];
    let mut rng = StdRng::from_seed(s);
    let mut f = |x: &R201| {
        let i  = (rng.gen::<f64>() * 3.) as u32;
        let f1 = &|x: &R201| (x + &m.a).normalized();
        let f2 = &|x: &R201| (x + &m.b).normalized();
        let f3 = &|x: &R201| (x + &m.c).normalized();

        match i {
            0 => f1(x),
            1 => f2(x),
            2 => f3(x),
            _ => f3(x)
        }
    };

    println!("drawing {} points", m.t as u64);
    triangle(&draw, m, &m.m, m.t, &mut f);
    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: Event) {
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
            KeyReleased(key) => {
                match key {
                    Key::S => {
                        app.main_window()
                            .capture_frame(app.exe_name().unwrap() + ".png");
                    } _ => () } } _ => () } _ => ()
    }
    update_model(model);
}
