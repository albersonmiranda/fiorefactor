use extendr_api::prelude::*;
use rayon::prelude::*;
use crate::iom::Iom;

#[extendr]
impl Iom {
  /// Compute technical coefficients
  ///     - param: `self` A mutable reference to the Iom instance.
  ///     - return: A new instance of the Iom class with the technical coefficients matrix computed.
  pub fn compute_technical_coefficients(&mut self) {
      let n = (self.intermediate_transactions.len() as f64).sqrt() as usize;

      // divide each entry of intermediate_transactions by each column of total_production
      let a_matrix: Vec<f64> = self
          .intermediate_transactions
          .par_iter()
          .enumerate()
          .map(|(i, value)| value / self.total_production[i / n])
          .collect();

      self.technical_coefficients_matrix =
          RArray::new_matrix(n, n, |row, column| a_matrix[row + column * n]);

      // Print message to R console
      rprintln!("Successfully computed the technical coefficients matrix.");
  }
}

extendr_module! {
  mod leontief;
  impl Iom;
}