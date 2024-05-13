// Define function to minimise
fn func(x: f64) -> f64 {
    (x - 5.0).powf(2.0) - 3.0
}

// ...it's derivative
fn derivative(x: f64) -> f64 {
    let h = 0.00001;
    (func(x+h) - func(x-h)) / (2.0*h)  // finite diff approximation
}

// ... and it's 2nd derivative
fn second_derivative(x: f64) -> f64 {
    let h = 0.00001;
    (func(x+h) - 2.0*func(x) + func(x-h)) / h.powi(2)  // Second derivative approximation
}

// newton's method
fn optimize_newton_method(x0: f64, e: f64, max_iter: u32) -> f64 {
    let mut x = x0;
    let mut iter = 0;

    while derivative(x).abs() > e && iter < max_iter {
        x = x - derivative(x) / second_derivative(x); // xn+1 = xn - (f(xn) / f'(xn))
        iter += 1;
    }
    return x;
}

fn main() {
    let x0 = 0.0; // initial guess
    let e = 0.00001; // tolerance
    let max_iter = 1000; // ToDo: Dynamically handle non-convergence cases

    let opt_x = optimize_newton_method(x0, e, max_iter);

    println!("{:.10}", opt_x);
}
