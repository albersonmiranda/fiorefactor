use extendr_api::prelude::*;

/// A test class to demonstrate the use of extendr.
/// @description
/// This is a test struct to check if it's working with multiple structs.
/// @usage
/// Test$new(name, value)
/// @format NULL
/// @param name (`character`)\cr
/// A string representing the name of the test.
/// @param value (`numeric`)\cr
/// A numeric value representing the value of the test.
/// @return A new instance of the `Test` class.
#[extendr]
pub struct Test {
    pub name: String,
    pub value: f64,

}

#[extendr]
impl Test {
    /// Instantiate a new Test object
    /// @details
    /// This function creates a new instance of the Test class.
    /// @param name (`character`)\cr
    /// A string representing the name of the test.
    /// @param value (`numeric`)\cr
    /// A numeric value representing the value of the test.
    /// @return A new instance of the Test class.
    /// @examples
    /// Test$new(name = "example", value = 42)
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
    /// @details
    /// This function returns the name of the test.
    /// @return The name of the test.
    /// @examples
    /// test <- Test$new(name = "example", value = 42)
    /// test$get_name()
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// @details
    /// This function returns the value of the test.
    /// @return The value of the test.
    /// @examples
    /// test <- Test$new(name = "example", value = 42)
    /// test$get_value()
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

extendr_module! {
    mod example;
    impl Test;
}