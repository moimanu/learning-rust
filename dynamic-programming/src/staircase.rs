pub fn count_ways(steps: usize) -> u64 {
    match steps {
        0 | 1 => 1,
        2 => 2,
        _ => {
            let mut prev3 = 1;
            let mut prev2 = 1;
            let mut prev1 = 2;

            for _ in 3..=steps {
                let current = prev1 + prev2 + prev3;
                prev3 = prev2;
                prev2 = prev1;
                prev1 = current;
            }

            prev1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_handle_base_cases() {
        assert_eq!(count_ways(0), 1);
        assert_eq!(count_ways(1), 1);
        assert_eq!(count_ways(2), 2);
    }

    #[test]
    fn should_calculate_four_steps_correctly() {
        assert_eq!(count_ways(4), 7);
    }
}