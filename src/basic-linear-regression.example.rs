// # Add this to your Cargo.toml file
// [dependencies]
// linregress = "0.1.1"

use linregress::linear_regression;

fn main() {
    // Sample data
    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data = vec![2.0, 3.0, 4.0, 3.5, 5.0];

    // Perform linear regression
    let (slope, intercept, _r_value, _p_value, _std_err) = linear_regression(&x_data, &y_data).unwrap();

    // Print results
    println!("Slope: {}", slope);
    println!("Intercept: {}", intercept);
}





