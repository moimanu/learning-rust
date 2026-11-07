pub fn max_sum(values: &[u32]) -> u32 {
    match values.len() {
        0 => 0,
        1 => values[0],
        _ => {
            let mut prev2 = values[0];
            let mut prev1 = values[0].max(values[1]);

            for &current_val in &values[2..] {
                let current_max = prev1.max(prev2 + current_val);
                prev2 = prev1;
                prev1 = current_max;
            }

            prev1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_zero_for_empty_slice() {
        assert_eq!(max_sum(&[]), 0);
    }

    #[test]
    fn should_solve_example_case() {
        assert_eq!(max_sum(&[3, 2, 7, 10]), 13);
    }

    #[test]
    fn should_handle_alternating_high_values() {
        assert_eq!(max_sum(&[2, 1, 4, 9]), 11);
    }
}