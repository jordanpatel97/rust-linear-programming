fn gradient_descent<F>(mut x: f64, learning_rate: f64, tolerance: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let derivative = |x: f64| -> f64 { f(x + tolerance) - f(x) };

    loop {
        let x_new = x - learning_rate * derivative(x);
        if (x - x_new).abs() < tolerance {
            break;
        }
        x = x_new;
    }

    x
}

fn main() {
    // Define the function the minimise
    let function = |x| (x - 5.0).powf(2.0);

    let x = 0.0; // initial guess
    let learning_rate = 0.1;
    let tolerance = 0.01;
    let min = gradient_descent(x, learning_rate, tolerance, function);

    println!("Minimum found at x = {}", min);
}
