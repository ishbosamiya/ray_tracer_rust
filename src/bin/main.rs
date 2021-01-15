use ray_tracer_rust::Vec3;
use ray_tracer_rust::PPM;

fn main() {
    let height = 256;
    let width = 128;
    let mut vals: Vec<Vec<Vec3>> = Vec::with_capacity(height);
    let mut empty_row = Vec::with_capacity(width);
    empty_row.resize(width, Vec3::new(0.0, 0.0, 0.0));
    vals.resize(height, empty_row);
    for (j, row) in vals.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            let j = height - j - 1;
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            *cell = Vec3::new(u, v, 0.0);
        }
    }

    let ppm = PPM::new(&vals);
    ppm.write_to_file("image.ppm").unwrap();
}
