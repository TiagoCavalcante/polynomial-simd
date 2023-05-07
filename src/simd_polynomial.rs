use std::simd::SimdFloat;

const SIZE: usize = 16;

pub struct Polynomial {
  coefficients: std::vec::Vec<std::simd::f32x16>,
}

impl Polynomial {
  pub fn new(coff: std::vec::Vec<f32>) -> Self {
    let mut coefficients = coff.clone();

    let new_len = (coefficients.len() + SIZE) / SIZE * SIZE;

    // Pad the coefficients with zeros.
    coefficients.resize(new_len, 0.0);

    Polynomial {
      coefficients: coefficients
        .chunks_exact(SIZE)
        .map(std::simd::f32x16::from_slice)
        .collect(),
    }
  }

  pub fn at(&self, x: f32) -> f32 {
    let mut sum = std::simd::f32x16::splat(0.0);

    let mut xs = std::simd::f32x16::from([
      1.0,
      x,
      x * x,
      x * x * x,
      x * x * x * x,
      x * x * x * x * x,
      x * x * x * x * x * x,
      x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x * x * x * x * x * x,
      x * x * x * x * x * x * x * x * x * x * x * x * x * x * x,
    ]);

    let x_size = std::simd::f32x16::splat(
      // x to the power of SIZE
      x * x * x * x * x * x * x * x * x * x * x * x * x * x * x * x,
    );

    for chunk in &self.coefficients {
      sum += chunk * xs;
      xs *= x_size;
    }

    sum.reduce_sum()
  }
}

mod tests {
  #[test]
  fn evaluate() {
    let polynomial =
      super::Polynomial::new(vec![-1.0, 1.0, -1.0, 1.0]);
    assert_eq!(polynomial.at(1.0), 0.0);
  }
}
