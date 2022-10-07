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
    fn _binary_search(nums: Vec<u32>, target_num: u32, left: usize, right: usize) -> Option<usize> {
        while left <= right {
            let mid = (left + right) / 2 as usize;
            if nums[mid] == target_num {
                return Some(mid);
            } else if nums[mid] > target_num {
                return _binary_search(nums, target_num, left, mid - 1);
            } else if nums[mid] < target_num {
                return _binary_search(nums, target_num, mid + 1, right);
            }
        }
        return None;
    }
    let left = 0 as usize;
    let right = nums.len() - 1;
    return _binary_search(nums, target_num, left, right);
}

#[cfg(test)]
mod tests {
    use super::{binary_search, binary_search_key};

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
    }
}
