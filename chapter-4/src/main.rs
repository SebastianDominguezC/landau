extern crate csv;
mod decay;
mod protein;
mod protein3d;
mod pseudo;
mod walk;

// uncomment desired simulation
fn main() {
    pseudo::test();
    walk::walk();
    protein::fold(1.0, 5);
    protein3d::fold(1.0, 12);
    decay::main(100_000, 0.3);
}
