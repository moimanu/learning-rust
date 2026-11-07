pub fn min_coins(coins: &[usize], target: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; target + 1];
    dp[0] = 0;

    for amount in 1..=target {
        for &coin in coins {
            if coin <= amount && dp[amount - coin] != usize::MAX {
                dp[amount] = dp[amount].min(dp[amount - coin] + 1);
            }
        }
    }

    match dp[target] {
        usize::MAX => None,
        count => Some(count),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_greedy_trap_example() {
        assert_eq!(min_coins(&[1, 3, 4], 6), Some(2));
    }

    #[test]
    fn should_return_none_when_impossible() {
        assert_eq!(min_coins(&[2, 4], 7), None);
    }

    #[test]
    fn should_return_zero_for_zero_target() {
        assert_eq!(min_coins(&[1, 3, 4], 0), Some(0));
    }
}