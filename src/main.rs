#![feature(portable_simd)]

mod polynomial;
mod simd_polynomial;

fn main() {
  let mut buffer = String::new();
  match std::io::stdin().read_line(&mut buffer) {
    Ok(_) => {}
    Err(error) => {
      println!("error: {error}");
      std::process::exit(1);
    }
  }

  let coefficients: Vec<_> = buffer
    .split_whitespace()
    // Remove non-number parts.
    .filter_map(|s| s.parse().ok())
    .collect();

  let polynomial = polynomial::Polynomial::new(coefficients.clone());

  let simd_polynomial =
    simd_polynomial::Polynomial::new(coefficients);

  // Reset the buffer.
  buffer.clear();

  // Input a value for x.
  match std::io::stdin().read_line(&mut buffer) {
    Ok(_) => {}
    Err(error) => {
      println!("error: {error}");
      std::process::exit(1);
    }
  }

  let x = match buffer.trim().parse() {
    Ok(x) => x,
    Err(error) => {
      println!("error: {error}");
      std::process::exit(1);
    }
  };

  // Benchmark at.
  let start = std::time::Instant::now();
  for _ in 0..1000000 {
    polynomial.at(x);
  }
  let value = polynomial.at(x);
  let duration = start.elapsed();
  println!("at: {} ({} ns)", value, duration.as_nanos());

  // Benchmark at_simd.
  let start = std::time::Instant::now();
  for _ in 0..1000000 {
    simd_polynomial.at(x);
  }
  let value = simd_polynomial.at(x);
  let duration = start.elapsed();
  println!("at_simd: {} ({} ns)", value, duration.as_nanos());
}
