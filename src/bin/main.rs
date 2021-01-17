use ray_tracer_rust::Image;
use ray_tracer_rust::Ray;
use ray_tracer_rust::Vec3;
use ray_tracer_rust::PPM;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1280;
    let height: usize = (width as f64 / aspect_ratio) as usize;
    let mut image = Image::new(width, height);

    let viewport_height = 2.0;
    let viewport_width = viewport_height as f64 * aspect_ratio;
    let focal_length = 1.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for (j, row) in image.get_mut_pixels().iter_mut().enumerate() {
        for (i, pixel) in row.iter_mut().enumerate() {
            let j = height - j - 1;
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let ray = Ray::new(
                &origin,
                &(&lower_left_corner + u * &horizontal + v * &vertical - &origin),
            );

            let color = ray_color(&ray);

            *pixel = color;
        }
    }

    let ppm = PPM::new(&image);
    ppm.write_to_file("image.ppm").unwrap();
}

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}
