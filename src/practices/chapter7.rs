pub fn least_coin_usage(mut total: u32) -> u32 {
    let mut coins = vec![1, 5, 10, 50, 100, 500];
    coins.reverse();
    let mut result = 0;

    for coin in coins {
        let used = total / coin;
        total -= used * coin;
        result += used;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::least_coin_usage;
    #[test]
    fn test_least_coin_usage() {
        assert_eq!(least_coin_usage(666), 6);
        assert_eq!(least_coin_usage(363), 8);
    }
}
