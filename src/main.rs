use std::fs::File;
use std::io::prelude::*;

struct PPM {
    data: Vec<Vec<Vec3>>,
}

#[derive(Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }
}

impl PPM {
    fn new(data: &Vec<Vec<Vec3>>) -> PPM {
        assert!(data.len() > 0);
        assert!(data[0].len() > 0);

        let cloned_data = data.clone();

        return PPM { data: cloned_data };
    }

    fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut string_data = String::new();

        let header = "P3\n";
        string_data.push_str(header);

        let sizing = format!("{} {}\n", self.data[0].len(), self.data.len());
        string_data.push_str(&sizing);

        let max_val = "255\n";
        string_data.push_str(max_val);

        for i in &self.data {
            for j in i {
                string_data.push_str(&((j.x * 255.0) as i64 % 256).to_string());
                string_data.push_str(" ");
                string_data.push_str(&((j.y * 255.0) as i64 % 256).to_string());
                string_data.push_str(" ");
                string_data.push_str(&((j.z * 255.0) as i64 % 256).to_string());
                string_data.push_str(" ");
            }
            string_data.push_str("\n");
        }

        print!("{}", string_data);

        let mut fout = File::create(path).unwrap();
        fout.write_all(string_data.as_bytes())?;

        return Ok(());
    }
}

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
