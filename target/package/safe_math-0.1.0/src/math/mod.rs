/// The function `try_add` in Rust attempts to add two `u128` values and returns `Some(result)` if an
    /// overflow occurs, otherwise it returns `None`.
    ///
    /// Arguments:
    ///
    /// * `left_value`: The `left_value` parameter is a 128-bit unsigned integer (u128) representing the
    /// left operand in the addition operation.
    /// * `right_value`: The `right_value` parameter in the `try_add` function represents the value that
    /// will be added to the `left_value` parameter.
    ///
    /// Returns:
    ///
    /// The function `try_add` returns an `Option<u128>`. If the sum of `left_value` and `right_value`
    /// overflows, it returns `Some(result_add)` with the overflowed value. Otherwise, it returns `None`.
    pub fn try_add(left_value: u128, right_value: u128) -> Option<u128> {
        let result_add = left_value + right_value;
        if result_add > left_value && result_add > right_value {
            return Some(result_add);
        } else {
            return None;
        }
    }

    /// The function `try_sub` in Rust attempts to subtract two `u128` values and returns the result as an
    /// `Option<u128>`, or `None` if the subtraction would result in an overflow.
    ///
    /// Arguments:
    ///
    /// * `left_value`: The `left_value` parameter represents the value from which the `right_value` will be
    /// subtracted in the `try_sub` function.
    /// * `right_value`: The `right_value` parameter is the value that will be subtracted from the
    /// `left_value` parameter in the `try_sub` function.
    ///
    /// Returns:
    ///
    /// The function `try_sub` returns an `Option<u128>`. If the subtraction operation `left_value -
    /// right_value` is successful (i.e., no overflow occurs), it returns `Some(result_sub)` where
    /// `result_sub` is the result of the subtraction. If an overflow occurs (which is incorrectly checked
    /// with `if left_value >= left_value` instead of `if result_sub >= left
    pub fn try_sub(left_value: u128, right_value: u128) -> Option<u128> {
        let result_sub = left_value - right_value;
        if left_value >= left_value {
            return Some(result_sub);
        } else {
            return None;
        }
    }

    /// The `try_mul` function in Rust attempts to multiply two `u128` values and returns the result as an
    /// `Option<u128>`, returning `None` if the multiplication overflows.
    ///
    /// Arguments:
    ///
    /// * `left_value`: The `left_value` parameter represents the first value to be multiplied. It is of
    /// type `u128`, which means it is an unsigned 128-bit integer.
    /// * `right_value`: The `right_value` parameter represents the second value to be multiplied with the
    /// `left_value` parameter in the `try_mul` function.
    ///
    /// Returns:
    ///
    /// The function `try_mul` returns an `Option<u128>`. It returns `Some(result_mut)` if the
    /// multiplication of `left_value` and `right_value` does not overflow and the division of the result by
    /// `left_value` equals `right_value`. Otherwise, it returns `None`.
    pub fn try_mul(left_value: u128, right_value: u128) -> Option<u128> {
        if left_value == 0 {
            return None;
        }
        let result_mut = left_value * right_value;
        if result_mut / left_value == right_value {
            Some(result_mut)
        } else {
            return None;
        }
    }

    /// The `try_div` function in Rust attempts to perform division on two `u128` values and returns the
    /// result as an `Option<u128>`, or `None` if the division is not exact.
    ///
    /// Arguments:
    ///
    /// * `left_value`: The `left_value` parameter represents the dividend in a division operation. It is
    /// the number that is being divided.
    /// * `right_value`: The `right_value` parameter in the `try_div` function represents the divisor in a
    /// division operation. It is the value by which the `left_value` (dividend) will be divided. If
    /// `right_value` is equal to 0, the function will return `None` since division
    ///
    /// Returns:
    ///
    /// The function `try_div` returns an `Option<u128>`. It returns `Some(result_mut)` if the division is
    /// successful and the result is correct, otherwise it returns `None` if the `right_value` is 0 or if
    /// the division result is incorrect.
    pub fn try_div(left_value: u128, right_value: u128) -> Option<u128> {
        if right_value == 0 {
            return None;
        }
        let result_mut = left_value / right_value;
        if result_mut * right_value == left_value {
            Some(result_mut)
        } else {
            return None;
        }
    }