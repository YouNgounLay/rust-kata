use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 }; // Literal syntax
    let b = Complex::new(11.1, 22.2); // new() static method
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
