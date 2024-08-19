use std::io;
use std::io::Write;

fn main() {
    let a = get_coefficient("a");
    let b = get_coefficient("c");
    let c = get_coefficient("c");

    find_roots(a, b, c);
}

fn get_coefficient(name: &str) -> f64 {
    println!("Enter coefficient {}:", name);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a number")
}

fn find_roots(a: f64, b: f64, c: f64) {

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: root1 = {}, root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The roots are real and equal: root = {}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (discriminant.abs().sqrt()) / (2.0 * a);
        println!("The roots are complex: root1 = {} + {}i, root2 = {} - {}i",
                 real_part, imaginary_part, real_part, imaginary_part);
    }
}
