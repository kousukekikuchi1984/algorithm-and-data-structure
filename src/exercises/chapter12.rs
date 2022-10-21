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

#[cfg(test)]
mod test {

    use super::insert_sort;

    #[test]
    fn test_insert_sort() {
        assert_eq!(insert_sort(vec![1, 5, 2, 4, 3]), vec![1, 2, 3, 4, 5]);
    }
}
