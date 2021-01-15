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
}
