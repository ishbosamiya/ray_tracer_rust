use ray_tracer_rust::random_in_unit_sphere;
use ray_tracer_rust::Camera;
use ray_tracer_rust::Hittable;
use ray_tracer_rust::HittableList;
use ray_tracer_rust::Image;
use ray_tracer_rust::Ray;
use ray_tracer_rust::Sphere;
use ray_tracer_rust::Vec3;
use ray_tracer_rust::PPM;

use rand::random;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1280;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let mut image = Image::new(width, height);
    let num_samples = 50;
    let max_depth = 50;

    let viewport_height = 2.0;
    let focal_length = 1.0;
    let camera = Camera::new(
        viewport_height,
        aspect_ratio,
        focal_length,
        Vec3::new(0.0, 0.0, 0.0),
    );

    let mut objects = HittableList::new();
    objects.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.add_object(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for (j, row) in image.get_mut_pixels().iter_mut().enumerate() {
        for (i, pixel) in row.iter_mut().enumerate() {
            let j = height - j - 1;

            *pixel = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let u = (i as f64 + random::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (height - 1) as f64;

                let ray = camera.get_ray(u, v);

                let color = ray_color(&ray, &objects, max_depth);

                *pixel += color;
            }

            *pixel /= num_samples as f64;
            *pixel = Vec3::new(pixel.x().sqrt(), pixel.y().sqrt(), pixel.z().sqrt());
        }
    }

    let ppm = PPM::new(&image);
    ppm.write_to_file("image.ppm").unwrap();
}

fn ray_color(ray: &Ray, objects: &HittableList, depth: usize) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    match objects.hit(ray, 0.001, 1000.0) {
        Some(record) => {
            let target = &record.p + record.get_normal() + random_in_unit_sphere();
            return 0.5
                * ray_color(
                    &Ray::new(&record.p, &(target - &record.p)),
                    objects,
                    depth - 1,
                );
        }
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}
