use csv::Writer;
use rand::{thread_rng, Rng};

pub fn walk() {
    let mut rng = thread_rng();
    let mut writer = Writer::from_path("./chapter-4/data/walk/walks.csv").unwrap();

    let walks = 7;
    let n = 1000;

    for _ in 0..walks {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut x_data: Vec<f64> = vec![0.0];
        let mut y_data: Vec<f64> = vec![0.0];
        let mut z_data: Vec<f64> = vec![0.0];

        for _ in 0..n {
            let dx: f64 = rng.gen_range(-1.0..=1.0);
            let dy: f64 = rng.gen_range(-1.0..=1.0);
            let dz: f64 = rng.gen_range(-1.0..=1.0);
            let l = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();

            let dx = dx / l;
            let dy = dy / l;
            let dz = dz / l;

            x += dx;
            y += dy;
            z += dz;

            x_data.push(x);
            y_data.push(y);
            z_data.push(z);
        }

        let x_data = x_data.iter().map(|v| v.to_string());
        let y_data = y_data.iter().map(|v| v.to_string());
        let z_data = z_data.iter().map(|v| v.to_string());

        writer.write_record(x_data).unwrap();
        writer.write_record(y_data).unwrap();
        writer.write_record(z_data).unwrap();

        writer.flush().expect("Cant write R data");
    }
}
