extern crate calamine;
extern crate linregress;


use calamine::{open_workbook, Xlsx, Reader};
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use std::collections::HashMap;
use std::error::Error;


pub fn get_calculations(col1: usize, col2: usize) -> Result<linregress::RegressionParameters, Box<dyn Error>> {
    let path = "./src/assets/excel.xlsx";
    let mut excel: Xlsx<_> = open_workbook(path)?;

    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();

    if let Some(Ok(range)) = excel.worksheet_range("Kalkyle WEB") {
        for row in range.rows().skip(1) {  // Skip the first row
            let antall_panel = row[col1].get_float().unwrap_or(0.0);  // Column 0 (A)
            let tot_kost = row[col2].get_float().unwrap_or(0.0);     // Column 19 (T)
            x_vals.push(antall_panel);
            y_vals.push(tot_kost);
        }
    }

    let mut data = HashMap::new();
    data.insert("y".to_string(), y_vals.clone());
    data.insert("x".to_string(), x_vals.clone());

    let regression_data = RegressionDataBuilder::new()
        .build_from(data)?;

    let formula = "y ~ x";

    let model = FormulaRegressionBuilder::new()
        .formula(formula)
        .data(&regression_data)
        .fit()?;

    let coefficients = model.parameters;

    Ok(coefficients)
}
