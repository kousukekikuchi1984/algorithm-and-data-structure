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

fn merge_sort<T: Ord + Copy + Debug>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let mut right = data.len();
    let middle = match right % 2 {
        0 => right / 2,
        1 => (right - 1) / 2,
        _ => panic!(),
    };
    let mut left = 0;

    merge_sort(&mut data[left..middle]);
    merge_sort(&mut data[middle..right]);

    let mut v = data.to_vec();
    v[middle..right].reverse();

    println!("{:?}", v);

    for i in 0..right - 1 {
        if v[left] > v[right] {
            data[i] = v[right];
            right -= 1;
        } else {
            data[i] = v[left];
            left += 1;
        }
    }
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
        let mut v = vec![1, 5, 2, 4, 3];
        merge_sort(&mut v);
        assert_eq!(&mut v, &mut vec![1, 2, 3, 4, 5]);
    }
}
