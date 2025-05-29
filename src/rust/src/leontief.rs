use extendr_api::prelude::*;
use rayon::prelude::*;
use faer::{Mat, linalg::solvers::Solve};
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
          .par_iter()
          .enumerate()
          .map(|(i, value)| value / self.total_production[i / n])
          .collect();

      self.technical_coefficients_matrix =
          RArray::new_matrix(n, n, |row, column| a_matrix[row + column * n]);

      // Print message to R console
      rprintln!("Successfully computed the technical coefficients matrix.");
  }

  /// Compute the Leontief inverse matrix and populate the `leontief_inverse_matrix` field.
  /// @usage NULL
  /// @details
  /// It computes the Leontief inverse matrix, a nxn matrix, known as `L` matrix, which is the inverse of the identity matrix minus the technical coefficients matrix.
  /// @return Self (invisibly)
  /// @examples
  /// iom <- Iom$new(
  ///   name = "example",
  ///   intermediate_transactions = c(1, 2, 3, 4),
  ///   total_production = c(5, 6)
  /// )
  /// iom$compute_leontief_inverse()
  /// iom$leontief_inverse_matrix
  pub fn compute_leontief_inverse(&mut self) {
      let n = (self.intermediate_transactions.len() as f64).sqrt() as usize;

      // create faer matrix
      let tech_coeff_matrix = Mat::from_fn(n, n, |row, col| self.intermediate_transactions[col * n + row]);

      // calculate Leontief matrix
      let identity_matrix: Mat<f64> = Mat::identity(n, n);
      let leontief_matrix = &identity_matrix - tech_coeff_matrix;

      // calculate Leontief inverse
      let leontief_inverse = leontief_matrix.partial_piv_lu().solve(identity_matrix);

      // convert to R matrix
      self.leontief_inverse_matrix = RArray::new_matrix(n, n, |row, col| leontief_inverse[(row, col)]);
    }
  }

extendr_module! {
  mod leontief;
  impl Iom;
}