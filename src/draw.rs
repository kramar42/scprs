use nannou::prelude::*;
use nannou::draw::Draw;

use super::r201::R201;
use super::ga;

pub fn axis(draw: &Draw) {
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

pub fn point(draw: &Draw, p: &R201, h: f64, r: f64) {
    let (x, y) = p.to_xy();
    //println!("r: {}", r);
    draw.ellipse()
        .radius(r as f32)
        //.resolution((40. / r) as usize)
        .resolution(3)
        .hsla(h as f32, 1., 0.5, 0.5)
        .x_y(x as f32, y as f32);
    //draw.text(t)
        //.color(BLACK)
        //.up(5.)
        //.left(5.);
}

pub fn line(draw: &Draw, l: &R201) {
    let xi = ga::ipoint(1., 0.);
    let alpha = (l.normalized() ^ xi)[7].asin();
    //println!("alpha: {}", alpha);

    let yi = ga::ipoint(0., 1.);
    let beta = (l.normalized() ^ yi)[7].asin();
    //println!("beta: {}", beta);

    let sign = beta / beta.abs();

    let o = ga::point(0., 0.);
    let dist = (o & l.normalized())[0];
    //println!("dist: {:?}", dist);

    draw.rect()
        .w(2000.)
        .h(4.)
        .color(GREEN)
        .x_y(0. as f32, dist as f32)
        .rotate((alpha * sign) as f32);
}

