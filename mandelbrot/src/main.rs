use num::complex::Complex;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64, x_max: f64,
    y_min: f64, y_max: f64,
    width: usize, height: usize
    
) -> Vec<Vec<usize>> {
}

fn mandelbrot_at_point(
    cx: f64, cy: f64,
    max_iters: usize
) -> usize {
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
}


fn main() {
  let mandelbrot = calculate_mandelbrot( 
      1000, 2.0,
      1.0, -1.0, 
      1.0, 200, 24
  );
  
  render_mandelbrot(mandelbrot);
}
