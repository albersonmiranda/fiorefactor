use extendr_api::prelude::*;

/// @title Rust class for input-output matrix.
/// @description
/// This class represents an input-output matrix.
/// @section Fields:
/// - `name`: A string representing the name of the input-output matrix.
/// - `intermediate_transactions`: A matrix of intermediate transactions.
/// - `total_production`: A vector of total production.
/// - `technical_coefficients_matrix`: A matrix of technical coefficients.
/// @section Methods:
/// - `compute_technical_coefficients`: Computes the technical coefficients matrix.
/// - `compute_inplace`: Computes the technical coefficients matrix in place.
/// - `name`: Getter for the name field.
/// - `intermediate_transactions`: Getter for the intermediate_transactions field.
/// - `total_production`: Getter for the total_production field.
/// @examples
/// intermediate_transactions <- matrix(c(1, 2, 3, 4, 5, 6, 7, 8, 9), 3, 3)
/// total_production <- matrix(c(100, 200, 300), 1, 3)
/// # instantiate iom object
/// my_iom <- fiorefactor::iom$new("test", intermediate_transactions, total_production)
/// # Calculate the technical coefficients
/// my_iom$compute_technical_coefficients()
/// # show the technical coefficients
/// my_iom$technical_coefficients_matrix
/// @export
/// @rdname iom
pub struct Iom {
    _name: String,
    pub intermediate_transactions: Vec<f64>,
    pub total_production: Vec<f64>,
    pub technical_coefficients_matrix: RArray<f64, [usize; 2]>,
}

#[extendr]
impl Iom {
    pub fn new(
        _name: String,
        intermediate_transactions: Vec<f64>,
        total_production: Vec<f64>,
    ) -> Self {
        let n = (intermediate_transactions.len() as f64).sqrt() as usize;

        Self {
            _name,
            intermediate_transactions,
            total_production,
            technical_coefficients_matrix: RArray::new_with_na(n, n),
        }
    }

    pub fn print(&self) {
        println!(
            "<Iom>
Name: {:?}
Intermediate Transactions: {:?}...
Total Production: {:?}...
Technical Coefficients Matrix: {:?}...
",
            self._name,
            &self
                .intermediate_transactions
                .iter()
                .take(5)
                .collect::<Vec<&f64>>(),
            &self.total_production.iter().take(5).collect::<Vec<&f64>>(),
            &self.technical_coefficients_matrix.slice(0..=5).unwrap()
        );
    }

    pub fn _name(&self) -> &str {
        &self._name
    }

    pub fn intermediate_transactions(&self) -> Vec<f64> {
        self.intermediate_transactions.clone()
    }

    pub fn total_production(&self) -> Vec<f64> {
        self.total_production.clone()
    }

    pub fn technical_coefficients_matrix(&self) -> Robj {
        self.technical_coefficients_matrix.clone()
    }
}

extendr_module! {
    mod iom;
    impl Iom;
}