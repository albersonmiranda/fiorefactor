use extendr_api::prelude::*;

#[extendr]
pub struct Iom {
    _name: String,
    pub intermediate_transactions: Vec<f64>,
    pub total_production: Vec<f64>,
    pub technical_coefficients_matrix: RArray<f64, 2>,
}

/// Rust class for input-output matrix
/// @description
/// This is the description
/// @export
#[extendr]
impl Iom {
    /// instantiate new iom class
    ///     - param: `_name` A string representing the name of the input-output matrix.
    ///     - param: `intermediate_transactions` A matrix of intermediate transactions.
    ///     - param: `total_production` A vector of total production.
    ///     - return: A new instance of the Iom class.
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

    /// Getter for intermediate_transactions matrix.
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