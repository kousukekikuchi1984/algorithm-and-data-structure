pub fn tribonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }
    return tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3);
}

pub fn memorized_tribonacci(n: u32) -> u32 {
    fn _tribonacci(n: u32, memo: &mut [Option<usize>]) -> usize {
        return memo[n as usize].unwrap_or_else(|| {
            let result = {
                if n == 0 || n == 1 {
                    0
                } else if n == 2 {
                    1
                } else {
                    _tribonacci(n - 1, memo) + _tribonacci(n - 2, memo) + _tribonacci(n - 3, memo)
                }
            };
            memo[n as usize] = Some(result);
            return result;
        });
    }

    let val = _tribonacci(n, &mut vec![None; (n + 1) as usize]);
    return val as u32;
}

#[cfg(test)]
mod tests {

    use super::tribonacci;

    #[test]
    fn test_tribonacci() {
        assert_eq!(tribonacci(4), 2);
        assert_eq!(tribonacci(9), 44);
        assert_eq!(tribonacci(19), 19513);
    }

    #[test]
    fn test_memorized_tribonacci() {
        assert_eq!(tribonacci(4), 2);
        assert_eq!(tribonacci(9), 44);
        assert_eq!(tribonacci(19), 19513);
    }
}
