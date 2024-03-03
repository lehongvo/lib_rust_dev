/// This Rust code snippet is defining a module named `math` and a submodule named `tests`. Here's a
/// breakdown of what each part of the code is doing:
pub mod math;

#[cfg(test)]
pub mod tests {  
    /// The line `use crate::math::{try_add, try_div, try_mul, try_sub};` is a Rust statement that is
    /// used to bring specific functions (`try_add`, `try_div`, `try_mul`, `try_sub`) from the `math`
    /// module into the current scope for easier access within the `tests` module.
    use crate::math::{try_add, try_div, try_mul, try_sub};

    /// The test function `tes_try_add` checks if the `try_add` function correctly adds two numbers and
    /// returns the expected result.
    #[test]
    fn tes_try_add() {
        let add_value = try_add(10, 20).unwrap_or_default();
        assert_eq!(add_value, 30, "<====Wrong value at try add modules====>");
    }

    /// The test function `tes_try_sub` checks the result of subtracting 10 from 20 using the `try_sub`
    /// function in Rust.
    #[test]
    fn tes_try_sub() {
        let sub_value = try_sub(20, 10).unwrap_or_default();
        assert_eq!(sub_value, 2, "<====Wrong value at try sub modules====>");
    }

    /// The function `tes_try_mul` tests the `try_mul` function by checking if the result of multiplying
    /// 0 and 10 is 0.
    #[test]
    fn tes_try_mul() {
        let mul_value = try_mul(0, 10).unwrap_or_default();
        assert_eq!(mul_value, 0, "<====Wrong value at try sub modules====>");
    }

    /// The function `tes_try_div` tests the `try_div` function by attempting to divide 100 by 0 and
    /// checking if the result is 0.
    #[test]
    fn tes_try_div() {
        let div_value = try_div(100, 0).unwrap_or_default();
        assert_eq!(div_value, 0, "<====Wrong value at try sub modules====>");
    }
}
