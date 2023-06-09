pub struct Polynomial {
  coefficients: std::vec::Vec<f32>,
}

impl Polynomial {
  pub fn new(coefficients: std::vec::Vec<f32>) -> Self {
    Polynomial { coefficients }
  }

  pub fn at(&self, x: f32) -> f32 {
    self
      .coefficients
      .iter()
      .fold((0.0, 1.0), |(r, xs), e| (r + e * xs, xs * x))
      .0
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn evaluate() {
    let polynomial =
      super::Polynomial::new(vec![-1.0, 1.0, -1.0, 1.0]);
    assert_eq!(polynomial.at(1.0), 0.0);
  }
}
