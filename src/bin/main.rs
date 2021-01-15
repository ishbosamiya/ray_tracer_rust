use ray_tracer_rust::Vec3;
use ray_tracer_rust::PPM;

fn main() {
    let vals: Vec<Vec<Vec3>> = vec![
        vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ],
        vec![
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
        ],
    ];

    let ppm = PPM::new(&vals);
    ppm.write_to_file("image.ppm").unwrap();
}
