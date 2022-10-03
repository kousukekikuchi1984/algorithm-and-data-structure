fn summer_holiday_happiness(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
    // a -> happiness of swimming
    // b -> happiness of bug catching
    // c -> happiness of doing homework
    // n -> n days
    // same category cannot be used consequcutive days
    let values = vec![&a, &b, &c];
    let mut dp = vec![vec![std::i32::MIN; 3]; a.len()];
    for i in 0..a.len() {
        for j in 0..3 {
            if i == 0 {
                dp[i][j] = values[j][i];
            } else {
                // remove same index;
                let mut val = values[j].to_vec();
                val.remove(i);
                let happiness = val.iter().max().unwrap();
                dp[i][j] = dp[i - 1][j] + happiness;
            }
        }
    }
    return *dp[a.len() - 1].iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::summer_holiday_happiness;

    #[test]
    fn test_summer_holiday_happiness() {
        let bugs = vec![1, 2, 3];
        let swimming = vec![2, 3, 1];
        let homework = vec![1, 1, 1];
        assert_eq!(summer_holiday_happiness(swimming, bugs, homework), 7);
    }
}
