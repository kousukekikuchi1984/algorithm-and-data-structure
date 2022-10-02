fn summer_holiday_happiness(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
    fn relaxzation(a: &mut i32, b: i32) {
        if *a < b {
            *a = b;
        }
    }
    // a -> happiness of swimming
    // b -> happiness of bug catching
    // c -> happiness of doing homework
    // n -> n days
    let mut dp = vec![std::i32::MIN; a.len()];
    let first_values = vec![a[0], b[0], c[0]];
    let first_happiness = first_values.iter().max().unwrap();
    dp[0] = *first_happiness;

    for n in 1..a.len() {
        let hp = vec![a[n], b[n], c[n]];
        let today_happiness = hp.iter().max().unwrap();
        let total_happiness: i32;
        if n == 1 {
            total_happiness = dp[0] + today_happiness;
        } else {
            total_happiness = dp[n - 1] + today_happiness;
        }
        relaxzation(&mut dp[n], total_happiness);
    }

    return dp[a.len() - 1];
}

#[cfg(test)]
mod tests {
    use super::summer_holiday_happiness;

    #[test]
    fn test_summer_holiday_happiness() {
        let bugs = vec![1, 2, 3];
        let swimming = vec![2, 3, 1];
        let homework = vec![1, 1, 1];
        assert_eq!(summer_holiday_happiness(swimming, bugs, homework), 8);
    }
}
