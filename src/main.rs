extern crate colored;

mod accuracy;
mod calculations;
mod console;


use std::time::{Instant};
use crate::calculations::get_calculations;
use crate::accuracy::calculate_accuracy;

use console::print_all;


fn main() {

    let start = Instant::now();
    let coefficients_total_cost = get_calculations(1, 19).unwrap();
    let coefficients_offer = get_calculations(1, 22).unwrap();

    let cloned_coefficient_total_cost = coefficients_total_cost.clone();
    let accuracy_total_cost = calculate_accuracy(cloned_coefficient_total_cost, 1 , 19).unwrap();

    let cloned_coefficient_offer = coefficients_offer.clone();
    let accuracy_offer = calculate_accuracy(cloned_coefficient_offer, 1 , 22).unwrap();
    
    // transform intercept to a string so i can print it

    let intercept_total_cost = coefficients_total_cost.intercept_value.to_string();
    let intercept_offer = coefficients_offer.intercept_value.to_string();

    let duration = start.elapsed();

    // format it to a string but its very important to get all the values down to mikroseconds
    let duration_text = format!("{:.5?}", duration);

    println!("");
    println!("Time elapsed: {}", duration_text);

    print_all("Total cost", accuracy_total_cost, 99.0, intercept_total_cost, coefficients_total_cost.regressor_values[0].to_string());
    print_all("Offer", accuracy_offer, 97.0, intercept_offer, coefficients_offer.regressor_values[0].to_string());
}

