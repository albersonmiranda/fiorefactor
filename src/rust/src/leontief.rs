use extendr_api::prelude::*;
use crate::iom::Iom;

#[extendr]
impl Iom {
  /// Compute the technical coefficients matrix and populate the `technical_coefficients_matrix` field.
  /// @details
  /// It computes the technical coefficientex matrix, a nxn matrix, known as `A` matrix, which is the column-wise ratio of intermediate transactions to total production.
  /// @return Self (invisibly)
  /// @examples
  /// iom <- Iom$new(
  ///   name = "example",
  ///   intermediate_transactions = c(1, 2, 3, 4),
  ///   total_production = c(5, 6)
  /// )
  /// iom$compute_technical_coefficients()
  /// iom$technical_coefficients_matrix
  pub fn compute_technical_coefficients(&mut self) {
      let n = (self.intermediate_transactions.len() as f64).sqrt() as usize;

      // divide each entry of intermediate_transactions by each column of total_production
      let a_matrix: Vec<f64> = self
          .intermediate_transactions
          .iter()
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