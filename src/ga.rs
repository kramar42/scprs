use super::r201::R201;

pub fn point(x: f64, y: f64) -> R201 {
    R201::e12() - x * R201::e02() + y * R201::e01()
}

pub fn ipoint(x: f64, y: f64) -> R201 {
    -x * R201::e02() + y * R201::e01()
}

pub fn line(a: f64, b: f64, c: f64) -> R201 {
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

    pub fn angle(self: &Self) -> f64 {
        let xi = ipoint(1., 0.);
        let o = point(0., 0.);
        ((&o & self).normalized() ^ &xi)[7].asin()
    }

    pub fn length(self: &Self) -> f64 {
        (self * !self)[0].abs().sqrt()
    }

    pub fn to_xy(self: &Self) -> (f64, f64) {
        (-self[5], self[4])
    }

    pub fn to_center(self: &Self) -> (f64, f64) {
        (-self[2], self[1])
    }

    pub fn distance(self: &Self, dest: &Self) -> f64 {
        (self & dest).norm()
    }

    pub fn rotator(self: &Self, a: f64) -> R201 {
        a.cos() + a.sin() as f64 * self
    }

    pub fn rotate(self: &Self, x: &Self) -> Self {
         self * x * self.Reverse()
    }
}
