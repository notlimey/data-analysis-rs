
use colored::*;


pub fn print_accuracy(acc: f64, threshold: f64) {
    println!("Accuracy: {}", {
        if acc > threshold {
            let str = acc.to_string() + "%" + " (OK)";
            str.green().bold()
        }  else {
            let str = acc.to_string() + "%" + " (Sub optimal)";
            str.red().bold()
        }
    });
}

pub fn print_intercept(intercept: String) {
    println!("Intercept: {}", {
        intercept.blue().bold()
    });
}

pub fn print_slope(slope: String) {
    println!("Slope: {}", {
        let mut slope = slope;
        slope.truncate(10);
        slope.blue().bold()
    });
}

pub fn print_all (title: &str, acc: f64, threshold: f64, intercept: String, slope: String) {
    println!("");
    println!("{}", title);
    print_accuracy(acc, threshold);
    print_intercept(intercept);
    print_slope(slope);
    println!("");
}