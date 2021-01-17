use crate::Ray;
use crate::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub t: f64,

    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
    }

    /// Sets the normal and whether or not the hit was on the front
    /// face based on the true normal given and the ray's direction
    pub fn set_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if !self.front_face {
            self.normal = -outward_normal.clone();
        } else {
            self.normal = outward_normal.clone();
        }
    }

    pub fn get_normal(&self) -> &Vec3 {
        return &self.normal;
    }

    pub fn get_front_face(&self) -> &bool {
        return &self.front_face;
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList {
            objects: Vec::new(),
        };
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result_record = None;
        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    result_record = Some(record);
                }
                None => continue,
            }
        }

        return result_record;
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        return Sphere { center, radius };
    }

    pub fn center(&self) -> &Vec3 {
        return &self.center;
    }

    pub fn radius(&self) -> f64 {
        return self.radius;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut record = HitRecord::new();
        record.t = root;
        record.p = ray.at(record.t);
        let outward_normal = (&record.p - self.center()) / self.radius();
        record.set_normal(ray, &outward_normal);

        return Some(record);
    }
}
