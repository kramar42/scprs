//#![windows_subsystem = "windows"]
#[link(name = "vcruntime")]

/// в центре статическая картинка
/// передвижение мышки по четырем углам окна передвигает нас по плоскости параметров
/// зажатие и движение меняет обзорный вектор
///

use nannou::prelude::*;
use nannou::draw::Draw;
use nannou::app::LoopMode;

mod r201;
use r201::R201;

fn point(x: f64, y: f64) -> R201 {
    R201::e12() - x * R201::e02() + y * R201::e01()
}

fn ipoint(x: f64, y: f64) -> R201 {
    -x * R201::e02() + y * R201::e01()
}

fn line(a: f64, b: f64, c: f64) -> R201 {
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
        (-self[5] * 50., self[4] * 50.)
    }

    pub fn to_center(self: &Self) -> (f64, f64) {
        (-self[2] * 50., self[1] * 50.)
    }
}

/*
impl Iter {
    pub fn partition(self: &Self, n: usize) -> (self.Item, self.Item) {
    }
}
*/

fn coeff(_coefs: &[f64]) -> R201 {
    let res = R201::zero();
    //for (i, c) in coefs.iter().partition(2) {
        //res[i] = c;
    //}
    res
}

fn pr(p: &R201) {
    println!("{:}", p);
}

fn main() {
    let p = ipoint(0.0, 0.0);
    pr(&p);
    let a = point(-1.0, 1.0);
    let b = point(1.0, 1.0);
    //let c = (&a + &b).normalized();
    let l = &a & &b;

    let x = line(0.0, 1.0, 0.0);
    let y = line(1.0, 0.0, -10.0);

    pr(&l);
    //(&x, &y) = l.to_center();
    println!("x: {}, y: {}", x, y);

    /*
    let no = coeff(0.5, -0.5); // some coeff
    let ni = coeff(1.0, -1.0);

    let ni_part = &p | (no.scale(-1.0)); // O_i + n_o O_oi
    let no_part = ni.scale(-1.0) | &p;   // O_o + O_oi n_i

    let no_ni_part = &no_part | no.scale(-1.0); // O_oi
    let no_only_part = ni ^ no_part | no.scale(-1.0); // O_oi

    let direction = no_ni_part;
    //let p = p.scale(1 / dl);
    */

    //let intersect = (&l ^ &y).normalized();
    //println!("{:?}", intersect.to_xy());

    /*
    let minus_no = no.Scale -1;

    let directior = no_ni_part;
    let dl = direction.length;
    o = o.scale(1/dl);
    let lx = o.e1;
    let ly = -o.e2;
    */

    let app = nannou::app(model)
        //.update(update)
        //.event(event)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn _update(_app: &App, _model: &mut Model, _update: Update) {
}

fn draw_axis(draw: &Draw) {
    // improve: draw in geom terms
    draw.rect()
        .w(2000.)
        .h(2.)
        .color(BLACK);
    draw.rect()
        .w(2.)
        .h(2000.)
        .color(BLACK);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    app.set_loop_mode(LoopMode::loop_once());

    let a = point(-3., 0.);
    let b = point( 3., 2.);
    let c = (&a + &b).normalized();
    let l = &b & &a;

    let draw = app.draw();
    draw.background().color(BLUE);
    draw_p(&draw, &a);
    draw_p(&draw, &b);
    draw_p(&draw, &c);
    draw_l(&draw, &l);
    draw_axis(&draw);

    //frame.clear(PURPLE);
    draw.to_frame(app, &frame).unwrap();
}

fn _event(_app: &App, _model: &mut Model, event: Event) {
    println!("got event: {:?}", event);
}

fn draw_p(draw: &Draw, p: &R201) {
    //pr(&p);

    let (x, y) = p.to_xy();

    draw.ellipse()
        .radius(5.0)
        .color(RED)
        .x_y(x as f32, y as f32);
}

fn draw_l(draw: &Draw, l: &R201) {
    //pr(l);
    //let no = ipoint(0.5, 0.5);
    //let minus_no = no.scale(-1.);
    //let loc = &minus_no | l;
    //pr(&l);
    let (x, y) = l.normalized().to_center();
    //println!("x: {}, y: {}", x, y);
    let start_point = pt2(-100.0 + x as f32, y as f32);
    let end_point   = pt2(100.0 + x as f32, y as f32);
    //println!("start: {:?}, end: {:?}", start_point, end_point);

    let xl = line(0., -1., 0.);
    let lr = (l.normalized() | xl.normalized())[0].acos() as f32;
    //println!("angle: {}", lr);

    draw.rect()
        //.weight(2.0)
        .w(2000.)
        .h(4.)
        .color(GREEN)
        .x_y(x as f32, y as f32)
        //.start(start_point)
        //.end(end_point);
        .rotate(lr);
}

fn _sketch(app: &App, _frame: Frame) {
}
