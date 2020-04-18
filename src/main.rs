#![windows_subsystem = "windows"]

/// в центре статическая картинка
/// передвижение мышки по четырем углам окна передвигает нас по плоскости параметров
/// зажатие и движение меняет обзорный вектор

use nannou::prelude::*;
use nannou::draw::Draw;
use nannou::app::LoopMode;

mod r201;
use r201::R201;

const POINT_SIZE: f32 = 1.;
const TRIANGLE_SIZE: f64 = 250.;
const MIN_SIZE: f64 = 0.7;

fn point(x: f64, y: f64) -> R201 {
    R201::e12() - x * R201::e02() + y * R201::e01()
}

fn ipoint(x: f64, y: f64) -> R201 {
    -x * R201::e02() + y * R201::e01()
}

fn _line(a: f64, b: f64, c: f64) -> R201 {
    a * R201::e1() + b * R201::e2() + c * R201::e0()
}

impl R201 {
    pub fn scale(self: &Self, s: f64) -> R201 {
        let mut res = R201::zero();
        let a = self;
        res[0] = a[0] * s;
        res[1] = a[1] * s;
        res[2] = a[2] * s;
        res[3] = a[3] * s;
        res[4] = a[4] * s;
        res[5] = a[5] * s;
        res[6] = a[6] * s;
        res[7] = a[7] * s;
        res
    }

    pub fn length(self: &Self) -> f64 {
        //(self * !self).abs().sqrt()
        1.0
    }

    pub fn to_xy(self: &Self) -> (f64, f64) {
        (-self[5], self[4])
    }

    pub fn to_center(self: &Self) -> (f64, f64) {
        (-self[2], self[1])
    }
}

/*
impl Iter {
    pub fn partition(self: &Self, n: usize) -> (self.Item, self.Item) {
    }
}
*/

fn _coeff(_coefs: &[f64]) -> R201 {
    let res = R201::zero();
    //for (i, c) in coefs.iter().partition(2) {
        //res[i] = c;
    //}
    res
}

fn _pr(p: &R201) {
    println!("{:}", p);
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    x: R201,
    a: R201,
    o: R201,
    m: R201,
    l: R201,
}

fn model(_app: &App) -> Model {
    let o = point(0., 0.);
    let x = point(0., -100.);
    let mut res = Model {
        x: x,
        a: o,
        o: point(0., 0.),
        l: R201::zero(),
        m: R201::zero(),
    };
    update_model(&mut res);
    res
}

fn update_model(model: &mut Model) {
    model.m = (&model.o + &model.a).normalized();
    model.l = &model.o & &model.a;
}

fn _update(_app: &App, _model: &mut Model, _update: Update) {
}

fn draw_axis(draw: &Draw) {
    // improve: draw in geom terms
    draw.rect()
        .w(2000.)
        .h(1.3)
        .color(BLACK);
    draw.rect()
        .w(1.3)
        .h(2000.)
        .color(BLACK);
}

fn triangle(draw: &Draw, x: &R201, y: f64) {
    if y < MIN_SIZE {
        draw_p(draw, &x, "1");
        return;
    }

    let angle = PI/3.;
    let dy = angle.sin() as f64 * y * R201::e01();
    let dx = -y * R201::e02();
    let p1 = x + &dy;
    let p2 = x - &dy * angle.cos() as f64 + &dx;
    let p3 = x - &dy * angle.cos() as f64 - &dx;
    //* sin(PI/3) - y * R201::e02();
    triangle(draw, &p1, y / 2.);
    triangle(draw, &p2, y / 2.);
    triangle(draw, &p3, y / 2.);
}

fn view(app: &App, model: &Model, frame: Frame) {
    app.set_loop_mode(LoopMode::loop_once());
    //app.set_loop_mode(LoopMode::wait());
    let draw = app.draw();

    draw.background().color(BLACK);
        //Rgba::new(0.7, 0.7, 0.7, 0.7));
    //draw_axis(&draw);

    //draw_p(&draw, &model.x, "o");
    triangle(&draw, &model.x, TRIANGLE_SIZE);

    //draw_l(&draw, &model.l);
    //draw_p(&draw, &model.a, "A");
    //draw_p(&draw, &model.o, "0");
    //draw_p(&draw, &model.m, "M");

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    //println!("got event: {:?}", &event);
    match event {
        Event::WindowEvent { simple: Some(event), .. } => match event {
            MouseMoved(coords) => {
                model.a = point(coords.x.into(), coords.y.into());
            }
            _other => (),
        },
        _ => (),
    }
    update_model(model);
}

fn draw_p(draw: &Draw, p: &R201, t: &str) {
    let (x, y) = p.to_xy();
    draw.ellipse()
        .radius(POINT_SIZE)
        .color(RED)
        .x_y(x as f32, y as f32);
    //draw.text(t)
        //.color(BLACK)
        //.up(5.)
        //.left(5.);
}

fn draw_l(draw: &Draw, l: &R201) {
    let xi = ipoint(1., 0.);
    let alpha = (l.normalized() ^ xi)[7].asin();
    //println!("alpha: {}", alpha);

    let yi = ipoint(0., 1.);
    let beta = (l.normalized() ^ yi)[7].asin();
    //println!("beta: {}", beta);

    let sign = beta / beta.abs();

    let o = point(0., 0.);
    let dist = (o & l.normalized())[0];
    //println!("dist: {:?}", dist);

    draw.rect()
        .w(2000.)
        .h(4.)
        .color(GREEN)
        .x_y(0. as f32, dist as f32)
        .rotate((alpha * sign) as f32);
}

fn _sketch(_app: &App, _frame: Frame) {
}
