
use std::f64::consts::PI;
use std::io;

fn main() {
    println!("=== MTH 101 Shape Area & Volume Calculator ===");
    println!("Select a shape/calculation:");
    println!("1. Trapezium (Area)");
    println!("2. Rhombus (Area)");
    println!("3. Parallelogram (Area)");
    println!("4. Cube (Surface Area)");
    println!("5. Cylinder (Volume)");
    println!("\nEnter your choice (1-5): ");
    

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => calculate_trapezium_area(),
        "2" => calculate_rhombus_area(),
        "3" => calculate_parallelogram_area(),
        "4" => calculate_cube_surface_area(),
        "5" => calculate_cylinder_volume(),
        _ => println!("Invalid selection! Please enter a number between 1 and 5."),
    };
}

// Helper function to read a floating point number from standard input
fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

// 1. Trapezium, area = height / 2 * (base1 + base2)
fn calculate_trapezium_area() {
    println!("\n-- Trapezium Area Calculation --");
    let height = read_input("Enter height: ");
    let base1 = read_input("Enter base1: ");
    let base2 = read_input("Enter base2: ");

    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium = {:.2}", area);
}

// 2. Rhombus, area = 1/2 * diagonal1 * diagonal2
fn calculate_rhombus_area() {
    println!("\n-- Rhombus Area Calculation --");
    let diagonal1 = read_input("Enter diagonal1: ");
    let diagonal2 = read_input("Enter diagonal2: ");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {:.2}", area);
}

// 3. Parallelogram, area = base * altitude
fn calculate_parallelogram_area() {
    println!("\n-- Parallelogram Area Calculation --");
    let base = read_input("Enter base: ");
    let altitude = read_input("Enter altitude: ");

    let area = base * altitude;
    println!("Area of Parallelogram = {:.2}", area);
}

// 4. Cube, surface area = 6 * side * side
fn calculate_cube_surface_area() {
    println!("\n-- Cube Surface Area Calculation --");
    let side = read_input("Enter side length: ");

    let surface_area = 6.0 * side * side;
    println!("Surface Area of Cube = {:.2}", surface_area);
}

// 5. Cylinder, volume = pi * radius * radius * height
fn calculate_cylinder_volume() {
    println!("\n-- Cylinder Volume Calculation --");
    let radius = read_input("Enter radius: ");
    let height = read_input("Enter height: ");

    let volume = PI * radius * radius * height;
    println!("Volume of Cylinder = {:.2}", volume);
}
