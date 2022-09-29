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

pub fn number_753(k: u32) -> u32 {
    let mut count: u32 = 0;
    fn _number_753(k: u32, prev_num: u32, count: &mut u32) {
        let prev_str = prev_num.to_string();
        println!("{}", prev_str);
        if prev_num > k {
            return; // stop iteration
        }
        if prev_str.contains("3") && prev_str.contains("5") && prev_str.contains("7") {
            *count += 1;
        }
        _number_753(k, prev_num * 10 + 3, count);
        _number_753(k, prev_num * 10 + 5, count);
        _number_753(k, prev_num * 10 + 7, count);
    }

    _number_753(k, 0, &mut count);
    return count as u32;
}

#[cfg(test)]
mod tests {

    use crate::chapter4::recursive::memorized_tribonacci;

    use super::{number_753, tribonacci};

    #[test]
    fn test_tribonacci() {
        assert_eq!(tribonacci(4), 2);
        assert_eq!(tribonacci(9), 44);
        assert_eq!(tribonacci(19), 19513);
    }

    #[test]
    fn test_memorized_tribonacci() {
        assert_eq!(memorized_tribonacci(4), 2);
        assert_eq!(memorized_tribonacci(9), 44);
        assert_eq!(memorized_tribonacci(19), 19513);
    }

    #[test]
    fn test_number_753() {
        assert_eq!(number_753(575), 4);
        assert_eq!(number_753(3600), 13);
    }
}
