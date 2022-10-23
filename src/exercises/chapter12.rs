use std::fmt::Debug;

pub fn insert_sort(mut v: Vec<u32>) -> Vec<u32> {
    println!("{:?}", v);
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
    v
}

fn merge_sort(arr: Vec<u32>) -> Vec<u32> {
    fn _merge(mut arr: Vec<u32>, left: usize, mid: usize, right: usize) -> Vec<u32> {
        let n1 = mid - left;
        let n2 = right - mid;
        let l1 = arr.clone();
        let r1 = arr.clone();
        let l = &l1[left..mid];
        let r = &r1[mid..right];

        let mut i = 0;
        let mut j = 0;
        let mut k = left;
        while i < n1 && j < n2 {
            if l[i] < r[j] {
                arr[k] = l[i];
                i += 1;
            } else {
                arr[k] = r[j];
                j += 1;
            }
            k += 1;
        }
        while i < n1 {
            arr[k] = l[i];
            i += 1;
            k += 1;
        }
        while j < n2 {
            arr[k] = r[j];
            j += 1;
            k += 1;
        }
        arr
    }
    fn _merge_sort(mut arr: Vec<u32>, left: usize, right: usize) -> Vec<u32> {
        if right - 1 > left {
            let mid = left + (right - left) / 2;
            arr = _merge_sort(arr, left, mid);
            arr = _merge_sort(arr, mid, right);
            arr = _merge(arr, left, mid, right);
        }
        arr
    }

    let size = arr.len();
    _merge_sort(arr, 0, size)
}

#[cfg(test)]
mod test {

    use super::{insert_sort, merge_sort};

    #[test]
    fn test_insert_sort() {
        assert_eq!(insert_sort(vec![1, 5, 2, 4, 3]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        assert_eq!(
            merge_sort(vec![64, 34, 25, 8, 22, 11, 9]),
            vec![8, 9, 11, 22, 25, 34, 64]
        );
    }
}
