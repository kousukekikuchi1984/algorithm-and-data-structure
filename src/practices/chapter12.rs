use superslice::Ext;

pub fn rank(v: Vec<u32>, i: usize) -> usize {
    let mut vec = v.to_vec();
    let search = vec[i - 1];
    vec.sort();
    vec.upper_bound(&search)
}

#[cfg(test)]
mod test {

    use super::rank;
    #[test]
    fn test_rank() {
        assert_eq!(rank(vec![1, 5, 4, 3, 2], 2), 5);
        assert_eq!(rank(vec![1, 5, 4, 3, 2], 1), 1);
        assert_eq!(rank(vec![1, 5, 4, 3, 2], 3), 4);
    }
}
