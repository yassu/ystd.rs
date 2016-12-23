mod ymath;

fn main() {
    let q = ymath::numbers::q32::Q32::new(1.0, 2.0, 3.0, 4.0);
    println!("{}", q);
}
