mod ymath;

fn main() {
    let q = ymath::numbers::yq32::YQ32::new(1.0, 2.0, 3.0, 4.0);
    println!("{}", q);
}
