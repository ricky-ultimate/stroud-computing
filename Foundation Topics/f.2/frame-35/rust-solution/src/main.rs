use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter coefficient a:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a number");
    input.clear();

    println!("Enter coefficient b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a number");
    input.clear();

    println!("Enter coefficient c:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: f64 = input.trim().parse().expect("Please enter a number");

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
