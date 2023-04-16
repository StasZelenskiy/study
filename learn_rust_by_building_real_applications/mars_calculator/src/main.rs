use std::io;

fn main() {
    println!("Enter weight (kg) on Earth:");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).unwrap();

    let weight: f32 = weight_input.trim().parse().unwrap();

    let weight_on_mars = calc_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", weight_on_mars);
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}