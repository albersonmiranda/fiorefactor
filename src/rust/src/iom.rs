use extendr_api::prelude::*;

/// Rust class for input-output matrix
/// @description
/// This class represents an input-output matrix, which is a representation of the transactions between different sectors of an economy.
/// It contains methods to compute the technical coefficients matrix and other related operations.
/// @usage NULL
/// @format NULL
/// @param name (`character`)\cr
/// A string representing the name of the input-output matrix.
/// @param intermediate_transactions (`matrix`)\cr
/// A matrix of intermediate transactions.
/// @param total_production (`matrix`)\cr A vector of total production.
/// @return A new instance of the `Iom` class.
/// 
/// @details
/// This paragraph of details is on struct-level.
/// 
/// @export
#[extendr]
pub struct Iom {
    name: String,
    pub intermediate_transactions: Vec<f64>,
    pub total_production: Vec<f64>,
    pub technical_coefficients_matrix: RArray<f64, 2>,
    pub leontief_inverse_matrix: RArray<f64, 2>,
}

/// @details
/// But hey! This impl-block adds stuff that requires clarification so this paragraph
/// is on impl-level docs but it's appended to struct docs! Isn't that cool?
#[extendr]
impl Iom {
    /// Instantiate a new Iom object
    /// @details
    /// This function creates a new instance of the Iom class.
    /// @param name (`character`)
    /// A string representing the name of the input-output matrix.
    /// @param intermediate_transactions (`matrix`)
    /// A matrix of intermediate transactions.
    /// @param total_production (`character`)
    /// A vector of total production.
    /// @return A new instance of the Iom class.
    /// @examples
    /// Iom$new(
    ///     name = "example",
    ///     intermediate_transactions = c(1, 2, 3, 4),
    ///     total_production = c(5, 6)
    /// )
    pub fn new(
        name: String,
        intermediate_transactions: Vec<f64>,
        total_production: Vec<f64>,
    ) -> Self {
        let n = (intermediate_transactions.len() as f64).sqrt() as usize;

        Self {
            name,
            intermediate_transactions,
            total_production,
            technical_coefficients_matrix: RArray::new_with_na(n, n),
            leontief_inverse_matrix: RArray::new_with_na(n, n),
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
            self.name,
            &self
                .intermediate_transactions
                .iter()
                .take(5)
                .collect::<Vec<&f64>>(),
            &self.total_production.iter().take(5).collect::<Vec<&f64>>(),
            &self.technical_coefficients_matrix.slice(0..=5).unwrap()
        );
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Getter for intermediate_transactions matrix.
    pub fn intermediate_transactions(&self) -> Vec<f64> {
        self.intermediate_transactions.clone()
    }

    /// Getter for total_production matrix.
    pub fn total_production(&self) -> Vec<f64> {
        self.total_production.clone()
    }

    /// Getter for technical_coefficients_matrix.
    pub fn technical_coefficients_matrix(&self) -> Robj {
        self.technical_coefficients_matrix.clone()
    }

    /// Getter for leontief_inverse_matrix.
    fn leontief_inverse_matrix(&self) -> Robj {
        self.leontief_inverse_matrix.clone()
    }
}

extendr_module! {
    mod iom;
    impl Iom;
}