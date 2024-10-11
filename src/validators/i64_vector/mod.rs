/// A trait responsible for validating a vector of `f64` values.
///
/// This trait defines a method to check the validity of a vector containing
/// `f64` values. It ensures that the vector is not empty and that all
/// contained values are non-negative (i.e., greater than or equal to 0.0).
/// If either condition fails, the method will return `false`.
///
/// ## Implementation Details
///
/// The trait includes the following method:
///
/// - `is_valid_f64_vector(&self, f64_vector: &Vec<f64>) -> bool`:
///   - Checks if the provided vector is empty. If it is, returns `false`.
///   - Iterates through each value in the vector and checks if it is a positive
///     `f64`. If any value is found to be negative, the method returns `false`.
///   - If the vector is non-empty and all values are non-negative, it returns `true`.
///
/// ## Performance Considerations
///
/// This method is designed to efficiently validate the vector in a single pass,
/// leveraging the iterator's `all` method. For large vectors, performance will be
/// linear in relation to the number of elements, as each element must be checked.
///
/// ## Trait Summary
///
/// - **Trait Name**: `NenyrF64Validator`
/// - **Method**: `is_valid_f64_vector(&self, f64_vector: &Vec<f64>) -> bool`
/// - **Returns**: `true` if the vector is valid (non-empty and all non-negative), `false` otherwise.
pub trait NenyrF64Validator {
    fn is_valid_f64_vector(&self, f64_vector: &Vec<f64>) -> bool {
        if f64_vector.is_empty() {
            return false;
        }

        f64_vector.iter().all(|&x| x >= 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrF64Validator;

    struct F64 {}

    impl F64 {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrF64Validator for F64 {}

    #[test]
    fn all_i64_are_valid() {
        let f_64 = F64::new();
        let valid_f64_vector: Vec<f64> = vec![
            64.0,
            78.0,
            98.0,
            54.0,
            0.0,
            65.0,
            654.0,
            6545.0,
            852.0,
            66445.05,
            7998.021,
            21.015,
            845.01,
            51.0511,
            5.0147,
            9.06,
            0.85,
            35.0,
            5.085,
            58.05,
            1.25545,
            2.98788,
            3.55555555,
            5.555555555555555555558784544,
        ];

        assert!(f_64.is_valid_f64_vector(&valid_f64_vector));
    }

    #[test]
    fn all_f64_are_valid_with_zeros() {
        let f_64 = F64::new();
        let valid_f64_vector: Vec<f64> = vec![0.0, 0.0, 0.0];

        assert!(f_64.is_valid_f64_vector(&valid_f64_vector));
    }

    #[test]
    fn all_f64_are_valid_with_large_numbers() {
        let f_64 = F64::new();
        let valid_f64_vector: Vec<f64> = vec![1e10, 2e20, 3e30];

        assert!(f_64.is_valid_f64_vector(&valid_f64_vector));
    }

    #[test]
    fn empty_vector_is_invalid() {
        let f_64 = F64::new();
        let empty_vector: Vec<f64> = vec![];

        assert!(!f_64.is_valid_f64_vector(&empty_vector));
    }

    #[test]
    fn vector_with_negative_numbers_is_invalid() {
        let f_64 = F64::new();
        let invalid_f64_vector: Vec<f64> = vec![1.0, -2.0, 3.0];

        assert!(!f_64.is_valid_f64_vector(&invalid_f64_vector));
    }

    #[test]
    fn single_positive_value_is_valid() {
        let f_64 = F64::new();
        let single_positive: Vec<f64> = vec![5.0];

        assert!(f_64.is_valid_f64_vector(&single_positive));
    }

    #[test]
    fn single_zero_value_is_valid() {
        let f_64 = F64::new();
        let single_zero: Vec<f64> = vec![0.0];

        assert!(f_64.is_valid_f64_vector(&single_zero));
    }

    #[test]
    fn single_negative_value_is_invalid() {
        let f_64 = F64::new();
        let single_negative: Vec<f64> = vec![-1.0];

        assert!(!f_64.is_valid_f64_vector(&single_negative));
    }

    #[test]
    fn performance_test_large_vector() {
        let f_64 = F64::new();
        let large_vector: Vec<f64> = (0..1_000_000).map(|x| x as f64).collect();

        assert!(f_64.is_valid_f64_vector(&large_vector));
    }

    #[test]
    fn large_negative_vector_is_invalid() {
        let f_64 = F64::new();
        let large_negative_vector: Vec<f64> = vec![-1.0; 1_000_000];

        assert!(!f_64.is_valid_f64_vector(&large_negative_vector));
    }

    #[test]
    fn mixed_large_vector_is_invalid() {
        let f_64 = F64::new();
        let mixed_large_vector: Vec<f64> = vec![1.0; 500_000]
            .into_iter()
            .chain(vec![-1.0].into_iter())
            .collect();

        assert!(!f_64.is_valid_f64_vector(&mixed_large_vector));
    }
}
