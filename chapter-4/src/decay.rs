use csv::Writer;
use rand::{thread_rng, Rng};

pub fn main(mut n: i32, lambda: f64) {
    // data for saving
    let mut times = vec![];
    let mut deltas = vec![];
    let mut particles = vec![];

    // data writer
    let mut writer = Writer::from_path("./chapter-4/data/decay/data.csv").unwrap();

    // sim vars
    let mut rng = thread_rng();
    let mut t = 0;

    while n > 0 {
        let mut delta_n = 0;

        for _ in 0..n {
            let decays = rng.gen_bool(lambda);

            if decays {
                delta_n += 1
            };
        }

        t += 1;
        n -= delta_n;

        times.push(t);
        deltas.push(delta_n);
        particles.push(n);
    }

    let times = times.iter().map(|v| v.to_string());
    let deltas = deltas.iter().map(|v| v.to_string());
    let particles = particles.iter().map(|v| v.to_string());

    writer.write_record(times).unwrap();
    writer.write_record(deltas).unwrap();
    writer.write_record(particles).unwrap();

    writer.flush().expect("Cant write R data");
}
