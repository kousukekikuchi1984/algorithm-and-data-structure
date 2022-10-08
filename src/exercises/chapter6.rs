use std::cmp::Ordering;

pub fn nth_rank(nums: Vec<u32>) -> Vec<usize> {
    fn _binary_search(nums: Vec<u32>, target_num: u32, left: isize, right: isize) -> Option<usize> {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;

        match target_num.cmp(&nums[mid as usize]) {
            Ordering::Equal => Some(mid as usize),
            Ordering::Less => _binary_search(nums, target_num, left, mid - 1),
            Ordering::Greater => _binary_search(nums, target_num, mid + 1, right),
        }
    }
    let mut copied = nums.to_vec();
    let mut results = vec![0; nums.len()];
    copied.sort();
    for i in 0..nums.len() {
        let key = _binary_search(copied.to_vec(), nums[i], 0, nums.len() as isize - 1);
        results[i] = key.unwrap();
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::nth_rank;

    #[test]
    fn test_nth_rank() {
        assert_eq!(nth_rank(vec![12, 43, 7, 15, 9]), vec![2, 4, 0, 3, 1]);
    }
}
