pub fn found(nums: Vec<u32>, target_num: u32) -> usize {
    let mut found_id = 1;
    for (i, num) in nums.iter().enumerate() {
        if num == &target_num {
            found_id = i;
        }
    }
    return found_id;
}

pub fn found_counter(nums: Vec<u32>, target_num: u32) -> u32 {
    let mut count = 0;
    for num in nums {
        if num == target_num {
            count += 1
        }
    }
    return count;
}

pub fn second_min(nums: Vec<u32>) -> u32 {
    let mut first = std::u32::MAX;
    let mut second = std::u32::MAX;

    for num in nums {
        if num < first {
            second = first;
            first = num;
        } else if num < second {
            second = num
        }
    }
    return second;
}

pub fn max_distance(nums: Vec<u32>) -> u32 {
    let mut max = std::u32::MIN;
    let mut min = std::u32::MAX;

    for num in nums {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
    }
    return max.abs_diff(min);
}

pub fn devide_two(nums: &mut Vec<u32>) -> u32 {
    let mut j = 0;
    loop {
        for (i, num) in nums.iter().enumerate() {
            if num % 2 == 0 {
                std::mem::replace(&mut nums[i], num / 2);
            } else {
                return j;
            }
        }
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{found, found_counter, max_distance, second_min};
    #[test]
    fn test_found() {
        assert_eq!(found(vec![1, 2, 3, 1, 2], 1), 3);
        assert_eq!(found(vec![1, 2, 3, 1, 2], 2), 4);
    }

    #[test]
    fn test_found_counter() {
        assert_eq!(found_counter(vec![1, 2, 3, 1, 2], 1), 2);
        assert_eq!(found_counter(vec![1, 2, 3, 1, 2], 3), 1);
    }

    #[test]
    fn test_second_min() {
        assert_eq!(second_min(vec![1, 2, 3, 4, 5, 6]), 2);
        assert_eq!(second_min(vec![1, 3, 4, 2, 6, 5]), 2);
    }

    #[test]
    fn test_max_distance() {
        assert_eq!(max_distance(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_distance(vec![5, 0, 3, 4, 2]), 5);
    }
}
