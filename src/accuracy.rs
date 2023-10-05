extern crate calamine;
extern crate linregress;


use calamine::{open_workbook, Xlsx, Reader};


pub fn calculate_accuracy(coefficients: linregress::RegressionParameters, col1: usize, col2: usize) -> Result<f64, String> {
    // Load your data and perform the accuracy calculation

    let path = "./src/assets/excel.xlsx";
    let mut excel: Xlsx<_> = open_workbook(path).expect("Error opening workbook.");

    // Initialize variables to store errors
    let mut absolute_errors = Vec::new();
    let mut percentage_errors = Vec::new();

    if let Some(Ok(range)) = excel.worksheet_range("Kalkyle WEB") {
        for row in range.rows().skip(1) {
            let antall_panel = row[col1].get_float().unwrap_or(0.0);  // Column 0 (A)

            // Parse the actual value as a floating-point number (handling the comma)
            let actual_value_str = row[col2].get_float().unwrap_or(0.0).to_string();
            let actual_value = actual_value_str.replace(",", ".").parse::<f64>().unwrap_or(0.0);

            // Calculate predicted value using the coefficients
            let intercept = coefficients.intercept_value;
            let slope = coefficients.regressor_values[0]; // Assuming only one regressor ("x")
            let predicted_value = intercept + slope * antall_panel;

            // Calculate absolute error
            let absolute_error = (actual_value - predicted_value).abs();

            // Calculate percentage error
            let percentage_error = (absolute_error / actual_value) * 100.0;

            absolute_errors.push(absolute_error);
            percentage_errors.push(percentage_error);
        }
    }

    // Calculate the average percentage error
    let average_percentage_error = percentage_errors.iter().sum::<f64>() / percentage_errors.len() as f64;

    let accuracy = 100.0 - average_percentage_error;

    Ok(accuracy)

}
