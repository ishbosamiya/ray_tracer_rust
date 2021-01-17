use ray_tracer_rust::Image;
use ray_tracer_rust::Vec3;
use ray_tracer_rust::PPM;

fn main() {
    let height = 256;
    let width = 128;
    let mut image = Image::new(width, height);
    for (j, row) in image.get_mut_pixels().iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            let j = height - j - 1;
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            *cell = Vec3::new(u, v, 0.0);
        }
    }

    let ppm = PPM::new(&image);
    ppm.write_to_file("image.ppm").unwrap();
}
