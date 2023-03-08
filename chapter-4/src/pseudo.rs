use csv::Writer;
use rand::{thread_rng, Rng};
use std::vec;

use std::ops::Rem;

/// PowerResidue
/// Defines a power residue struct with base parameters
pub struct PowerResidue {
    a: i64,
    ri: i64,
    c: i64,
    m: i64,
}

impl PowerResidue {
    /// Creates a new power residue prng
    pub fn new(a: i64, c: i64, m: i64, r0: i64) -> Self {
        Self { a, ri: r0, c, m }
    }
    /// Updates current random number and returns
    pub fn next(&mut self) -> i64 {
        let next_r = (self.a * self.ri + self.c).rem(self.m);
        self.ri = next_r;
        next_r
    }
}

// r is for lazg prng
// s is for better prng
// i is for iters
// f is for rust's prng
pub fn test() {
    // setup rust prng
    let mut rng = thread_rng();

    // output data files
    let mut r_writer = Writer::from_path("./chapter-4/data/prng/r.csv").unwrap();
    let mut i_writer = Writer::from_path("./chapter-4/data/prng/i.csv").unwrap();
    let mut f_writer = Writer::from_path("./chapter-4/data/prng/f.csv").unwrap();
    let mut s_writer = Writer::from_path("./chapter-4/data/prng/s.csv").unwrap();

    // power residue instances
    let r0 = 10;
    let m = 256;
    let mut prng = PowerResidue::new(57, 1, m, r0);
    let mut serious = PowerResidue::new(455, 12, 9810, r0);

    // temporal data storage
    let mut i = 0;
    let mut i_data = vec![1];

    let mut r = vec![r0];
    let mut f = vec![];
    let mut s = vec![];

    // generate numbers until r series repeats
    let mut next;
    while {
        next = prng.next();
        let val = rng.gen_range(0..=m);
        f.push(val);
        s.push(serious.next());
        next != r0
    } {
        i += 1;
        r.push(next);
        i_data.push(i);
        println!("r-{} = {}", i, next);
    }

    println!("Sequence repeats at: {}", i + 1);

    // save data for matlab plots
    let r = r.iter().map(|v| v.to_string());
    let i_data = i_data.iter().map(|v| v.to_string());
    let f = f.iter().map(|v| v.to_string());
    let s = s.iter().map(|v| v.to_string());

    r_writer.write_record(r).unwrap();
    i_writer.write_record(i_data).unwrap();
    f_writer.write_record(f).unwrap();
    s_writer.write_record(s).unwrap();

    r_writer.flush().expect("Cant write R data");
    i_writer.flush().expect("Cant write i data");
    f_writer.flush().expect("Cant write f data");
    s_writer.flush().expect("Cant write s data");
}
