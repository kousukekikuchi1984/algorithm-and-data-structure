use std::cmp::Ordering;

fn binary_search(nums: Vec<u32>, target_num: u32) -> bool {
    if nums.len() == 1 {
        return nums[0] == target_num;
    }
    let middle = if nums.len() % 2 == 0 {
        nums.len() / 2
    } else {
        (nums.len() - 1) / 2
    };
    match target_num.cmp(&nums[middle]) {
        Ordering::Equal => return true,
        Ordering::Greater => return binary_search(nums[middle..nums.len()].to_vec(), target_num),
        Ordering::Less => return binary_search(nums[0..middle].to_vec(), target_num),
    }
}

fn binary_search_key(nums: Vec<u32>, target_num: u32) -> Option<usize> {
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
    let left = 0 as isize;
    let right = nums.len() as isize - 1;
    return _binary_search(nums, target_num, left, right);
}

fn sum_of_two_integers(nums1: Vec<u32>, nums2: Vec<u32>, k: u32) -> u32 {
    let mut results = vec![0; nums1.len()];
    for i in 0..nums1.len() {
        println!("{}, {:?}", i, nums1);
        if k > nums1[i] {
            // must implement lower_bound function
            let key = binary_search_key(nums2.to_vec(), k - nums1[i]);
            results[i] = match key {
                None => std::u32::MAX,
                _ => nums2[key.unwrap() as usize] + nums1[i],
            };
        } else {
            results[i] = std::u32::MAX;
        }
    }
    let val = *results.iter().min().unwrap();
    match val {
        std::u32::MAX => 0,
        _ => val,
    }
}

#[cfg(test)]
mod tests {

    use super::{binary_search, binary_search_key, sum_of_two_integers};

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(vec![1, 3, 5, 7, 9, 11, 13], 7), true);
        assert_eq!(binary_search(vec![1, 3, 5, 8, 9, 11, 13], 7), false);
        assert_eq!(binary_search(vec![1, 3, 5, 8, 9, 11], 7), false);
        assert_eq!(binary_search(vec![1, 3, 5, 7, 9, 13], 7), true);
    }

    #[test]
    fn test_binary_search_key() {
        assert_eq!(binary_search_key(vec![1, 3, 5, 7, 9, 11, 13], 7), Some(3));
        assert_eq!(binary_search_key(vec![1, 3, 5, 8, 9, 11, 13], 7), None);
        assert_eq!(binary_search_key(vec![1, 3, 5, 8, 9, 11], 7), None);
        assert_eq!(binary_search_key(vec![1, 3, 5, 7, 9, 13], 7), Some(3));
        assert_eq!(binary_search_key(vec![1, 4, 6, 7, 8], 2), None);
        assert_eq!(
            binary_search_key(vec![1, 3, 5, 7, 9, 11, 31, 43, 65], 7),
            Some(3)
        );
    }

    #[test]
    fn test_sum_of_two_intgers() {
        // assert_eq!(
        //     sum_of_two_integers(vec![1, 4, 6, 7, 8], vec![2, 6, 8, 14, 17], 20),
        //     20
        // );
        // assert_eq!(
        //     sum_of_two_integers(vec![1, 4, 6, 7, 8], vec![2, 6, 8, 14, 17], 21),
        //     21
        // );
        assert_eq!(
            sum_of_two_integers(vec![1, 4, 6, 7, 8], vec![2, 6, 8, 14, 17], 4),
            6
        );
    }
}
