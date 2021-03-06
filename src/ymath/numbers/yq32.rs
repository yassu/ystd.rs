use std;
use std::ops::{Neg, Add, Mul, Sub, Div};


fn sgnf32(f: f32) -> String {
    if f > 0.0 {
        return "+".to_string();
    } else if f == 0.0 {
        return "0".to_string();
    } else {
        return "-".to_string();
    }
}

pub struct YQ32 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}


impl YQ32 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> YQ32 {
        YQ32 {x: x, y: y, z: z, w: w}
    }

    pub fn norm(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn to_tuple(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

impl std::clone::Clone for YQ32 {
    fn clone(&self) -> Self {
        YQ32::new(self.x.clone(), self.y.clone(),
                  self.z.clone(), self.w.clone())
    }
}


impl std::fmt::Display for YQ32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s:String = String::new();

        let sgn1 = sgnf32(self.x);
        if sgn1 != "0".to_string() {
            s += &sgn1;
            s += &self.x.abs().to_string();
        }

        let sgn2 = sgnf32(self.y);
        if sgn2 != "0".to_string() {
            s += &sgn2;
            s += &self.y.abs().to_string();
            s += "i";
        }

        let sgn3 = sgnf32(self.z);
        if sgn3 != "0".to_string() {
            s += &sgn3;
            s += &self.z.abs().to_string();
            s += "j";
        }

        let sgn4 = sgnf32(self.w);
        if sgn4 != "0".to_string() {
            s += &sgn4;
            s += &self.w.abs().to_string();
            s += "k";
        }

        if s == "".to_string() {
            s = "0".to_string();
        }

        // if s.starts_with("+") {
        //     s = s.to_string()[1..];
        // }
        write!(f, "{}", s)
    }
}


impl Neg for YQ32 {
    type Output = Self;
    fn neg(self) -> YQ32 {
        let x = self.x.clone();
        let y = self.y.clone();
        let z = self.z.clone();
        let w = self.w.clone();
        YQ32::new(-x, -y, -z, -w)
    }
}


impl Add for YQ32 {
    type Output = Self;
    fn add(self, rhs: Self) -> YQ32 {
        YQ32::new (self.x.clone() + rhs.x.clone(),
                   self.y.clone() + rhs.y.clone(),
                   self.z.clone() + rhs.z.clone(),
                   self.w.clone() + rhs.w.clone())
    }
}


impl Sub for YQ32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> YQ32 {
        YQ32::new (self.x.clone() - rhs.x.clone(),
                   self.y.clone() - rhs.y.clone(),
                   self.z.clone() - rhs.z.clone(),
                   self.w.clone() - rhs.w.clone())
    }
}


impl Mul for YQ32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> YQ32 {
        let x1 = self.x.clone();
        let y1 = self.y.clone();
        let z1 = self.z.clone();
        let w1 = self.w.clone();
        let x2 = rhs.x.clone();
        let y2 = rhs.y.clone();
        let z2 = rhs.z.clone();
        let w2 = rhs.w.clone();

        YQ32::new(
            x1*x2 - y1*y2 - z1*z2 - w1*w2,
            x1*y2 + x2*y1 + z1*w2 - w1*z2,
            x1*z2 - y1*w2 + x2*z1 + w1*y2,
            x1*w2 + y1*z2 - z1*y2 + w1*x2
       )
    }
}


impl Div for YQ32 {
    type Output = Self;
    fn div(self, rhs: Self) -> YQ32 {
        let norm = rhs.clone().norm();

        let x1 = self.x.clone();
        let y1 = self.y.clone();
        let z1 = self.z.clone();
        let w1 = self.w.clone();
        let x2 = rhs.x.clone();
        let y2 = rhs.y.clone();
        let z2 = rhs.z.clone();
        let w2 = rhs.w.clone();

        YQ32::new(
            (x1*x2 - y1*y2 - z1*z2 - w1*w2)/(norm * norm),
            (x1*y2 + x2*y1 + z1*w2 - w1*z2)/(norm * norm),
            (x1*z2 - y1*w2 + x2*z1 + w1*y2)/(norm * norm),
            (x1*w2 + y1*z2 - z1*y2 + w1*x2)/(norm * norm)
       )
    }
}
