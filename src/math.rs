extern crate overload;
use overload::overload;
use std::ops;

pub struct Vec3 {
    data: [f64; 3],
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        return Vec3 {
            data: self.data.clone(),
        };
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { data: [x, y, z] };
    }

    pub fn x(&self) -> f64 {
        return self.data[0];
    }
    pub fn y(&self) -> f64 {
        return self.data[1];
    }
    pub fn z(&self) -> f64 {
        return self.data[2];
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.data[0] * self.data[0]
            + self.data[1] * self.data[1]
            + self.data[2] * self.data[2];
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        return self.data[0] * rhs.data[0]
            + self.data[1] * rhs.data[1]
            + self.data[2] * rhs.data[2];
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        return Vec3::new(
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            -(self.data[0] * rhs.data[2] - self.data[2] * rhs.data[0]),
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        );
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self / self.length();
    }
}

// Neg
overload!(- (a: ?Vec3) -> Vec3 { Vec3::new(-a.data[0], -a.data[1], -a.data[2])});

// Add
overload!((a: ?Vec3) + (b: ?Vec3) -> Vec3 {Vec3::new(a.data[0] + b.data[0], a.data[1] + b.data[1], a.data[2] + b.data[2])});

// AddAssign
overload!((a: &mut Vec3) += (b: ?Vec3) { a.data[0] += b.data[0]; a.data[1] += b.data[1]; a.data[2] += b.data[2];});

// Sub
overload!((a: ?Vec3) - (b: ?Vec3) -> Vec3 {Vec3::new(a.data[0] - b.data[0], a.data[1] - b.data[1], a.data[2] - b.data[2])});

// SubAssign
overload!((a: &mut Vec3) -= (b: ?Vec3) { a.data[0] -= b.data[0]; a.data[1] -= b.data[1]; a.data[2] -= b.data[2];});

// Mul
overload!((a: ?Vec3) * (b: ?f64) -> Vec3 {Vec3::new(a.data[0] * b, a.data[1] * b, a.data[2] * b)});
overload!((a: ?f64) * (b: ?Vec3) -> Vec3 {Vec3::new(b.data[0] * a, b.data[1] * a, b.data[2] * a)});

// Div
overload!((a: ?Vec3) / (b: ?f64) -> Vec3 {Vec3::new(a.data[0] / b, a.data[1] / b, a.data[2] / b)});

// impl ops::Neg for Vec3 {
//     type Output = Vec3;

//     fn neg(self) -> Vec3 {
//         return Vec3::new(-self.data[0], -self.data[1], -self.data[2]);
//     }
// }

// impl ops::Add<Vec3> for Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: Vec3) -> Vec3 {
//         return Vec3::new(
//             self.data[0] + rhs.data[0],
//             self.data[1] + rhs.data[1],
//             self.data[2] + rhs.data[2],
//         );
//     }
// }

// impl ops::AddAssign<Vec3> for Vec3 {
//     fn add_assign(&mut self, rhs: Vec3) {
//         self.data[0] += rhs.data[0];
//         self.data[1] += rhs.data[1];
//         self.data[2] += rhs.data[2];
//     }
// }

// impl ops::Sub<Vec3> for Vec3 {
//     type Output = Vec3;

//     fn sub(self, rhs: Vec3) -> Vec3 {
//         return self + (-rhs);
//     }
// }

// impl ops::SubAssign<Vec3> for Vec3 {
//     fn sub_assign(&mut self, rhs: Vec3) {
//         *self += -rhs;
//     }
// }

// impl ops::Mul<f64> for Vec3 {
//     type Output = Vec3;

//     fn mul(self, rhs: f64) -> Vec3 {
//         return Vec3::new(self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs);
//     }
// }

// impl ops::MulAssign<f64> for Vec3 {
//     fn mul_assign(&mut self, rhs: f64) {
//         self.data[0] *= rhs;
//         self.data[1] *= rhs;
//         self.data[2] *= rhs;
//     }
// }

// impl ops::Div<f64> for Vec3 {
//     type Output = Vec3;
//     fn div(self, rhs: f64) -> Vec3 {
//         return self * (1.0 / rhs);
//     }
// }

// impl ops::DivAssign<f64> for Vec3 {
//     fn div_assign(&mut self, rhs: f64) {
//         *self *= 1.0 / rhs;
//     }
// }
