use nannou::prelude::*;
use nannou::draw::Draw;

use super::r201::R201;
use super::ga;

use super::Model;
use super::MIN_SIZE;

pub fn axis(draw: &Draw) {
    // TODO: improve: draw in geom terms
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
    draw.ellipse()
        .resolution(3)
        .x_y(x as f32, y as f32)
        .hsla(h as f32, 1., 0.5, 0.5)
        .radius(r as f32);
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

pub fn triangle(draw: &Draw, model: &Model, x: &R201, y: &R201, s: f64) {
    // calc the size of the triangle
    let d = x.distance(y);

    // if it's small enought - print the dot
    if d < MIN_SIZE {
        let hue = x.angle() / (PI) as f64;

        // how far we are from center
        let f = x.distance(&model.o);
        // make a spiral rotation out of scroll
        let dx = model.o.rotator(f * s);
        // division is not defined in r201
        let mx = dx.rotate(x) * 0.5;

        point(draw, &mx, hue, 3.);
        return;
    }

    // else recur to 3 triangles
    let r = x.rotator((PI/3.).into());

    // x is center of new triangles, y defines far vertex
    let p1 = x + y;
    let p2 = r.rotate(&p1);
    let p3 = r.rotate(&p2);

    // scale down by the factor of 2
    let y2 = y * 0.5;
    let s2 = s * 0.5;

    triangle(draw, model, &p1, &y2, s2);
    triangle(draw, model, &p2, &y2, s2);
    triangle(draw, model, &p3, &y2, s2);
}
