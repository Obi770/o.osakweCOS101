use std::io;


fn main() {
    println!("P - Poundo Yam / Edinkaiko Soup - N3200");
    println!("F - Fried Rice & Chicken - N3000");
    println!("A - Amala & Ewedu Soup - N2500");
    println!("E - Eba & Egusi Soup - N2000");
    println!("W - White Rice & Stew - N2500");


    println!("Enter food type:");
    let mut food = String::new();
    io::stdin().read_line(&mut food).unwrap();


    println!("Enter quantity:");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).unwrap();
    let qty: f64 = qty.trim().parse().unwrap();


    let price = match food.trim() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        _ => 2500.0, // W
    };


    let mut total = price * qty;


    if total > 10000.0 {
        total = total * 0.95;
    }


    println!("Total: N{}", total);
}