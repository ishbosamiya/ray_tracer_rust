extern crate overload;
use overload::overload;
use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    data: [f64; 3],
}

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
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

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        return Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        };
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return &self.origin + t * &self.direction;
    }

    pub fn origin(&self) -> &Vec3 {
        return &self.origin;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.direction;
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

// MulAssign
overload!((a: &mut Vec3) *= (b: ?f64) { a.data[0] *= b; a.data[1] *= b; a.data[2] *= b;});

// Div
overload!((a: ?Vec3) / (b: ?f64) -> Vec3 {Vec3::new(a.data[0] / b, a.data[1] / b, a.data[2] / b)});

// DivAssign
overload!((a: &mut Vec3) /= (b: ?f64) { a.data[0] /= b; a.data[1] /= b; a.data[2] /= b;});

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn length_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.length(), (14.0_f64).sqrt());
    }

    #[test]
    fn length_squared_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.length_squared(), 14.0);
    }

    #[test]
    fn dot_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = 38.0;
        assert_eq!(a.dot(&b), res);
    }

    #[test]
    fn cross_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, 8.0, -4.0);
        assert_eq!(a.cross(&b), res);
    }

    #[test]
    fn unit_vector_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let res = Vec3::new(
            1.0 / (14.0_f64).sqrt(),
            2.0 / (14.0_f64).sqrt(),
            3.0 / (14.0_f64).sqrt(),
        );
        assert_eq!(a.unit_vector(), res);
    }

    #[test]
    fn neg_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let res = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(-a, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let res = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(-&a, res);
    }

    #[test]
    fn add_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(6.0, 8.0, 10.0);
        assert_eq!(a + b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(6.0, 8.0, 10.0);
        assert_eq!(&a + b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(6.0, 8.0, 10.0);
        assert_eq!(a + &b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(6.0, 8.0, 10.0);
        assert_eq!(&a + &b, res);
    }

    #[test]
    fn add_assign_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(6.0, 8.0, 10.0);
        a += b;
        assert_eq!(a, res);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(6.0, 8.0, 10.0);
        a += &b;
        assert_eq!(a, res);
    }

    #[test]
    fn sub_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, -4.0, -4.0);
        assert_eq!(a - b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, -4.0, -4.0);
        assert_eq!(&a - b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, -4.0, -4.0);
        assert_eq!(a - &b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, -4.0, -4.0);
        assert_eq!(&a - &b, res);
    }

    #[test]
    fn sub_assign_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, -4.0, -4.0);
        a -= b;
        assert_eq!(a, res);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let res = Vec3::new(-4.0, -4.0, -4.0);
        a -= &b;
        assert_eq!(a, res);
    }

    #[test]
    fn mul_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(a * b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(&a * b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(a * &b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(&a * &b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(b * a, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(&b * a, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(b * &a, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        assert_eq!(&b * &a, res);
    }

    #[test]
    fn mul_assign_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        a *= b;
        assert_eq!(a, res);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(3.5, 7.0, 10.5);
        a *= &b;
        assert_eq!(a, res);
    }

    #[test]
    fn div_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(1.0 / 3.5, 2.0 / 3.5, 3.0 / 3.5);
        assert_eq!(a / b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(1.0 / 3.5, 2.0 / 3.5, 3.0 / 3.5);
        assert_eq!(&a / b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(1.0 / 3.5, 2.0 / 3.5, 3.0 / 3.5);
        assert_eq!(a / &b, res);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(1.0 / 3.5, 2.0 / 3.5, 3.0 / 3.5);
        assert_eq!(&a / &b, res);
    }

    #[test]
    fn div_assign_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(1.0 / 3.5, 2.0 / 3.5, 3.0 / 3.5);
        a /= b;
        assert_eq!(a, res);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.5;
        let res = Vec3::new(1.0 / 3.5, 2.0 / 3.5, 3.0 / 3.5);
        a /= &b;
        assert_eq!(a, res);
    }
}
