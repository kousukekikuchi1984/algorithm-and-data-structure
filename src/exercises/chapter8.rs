use std::ptr::NonNull;

pub fn vector() {
    let mut vector = vec![4, 3, 12, 7, 11, 1, 9, 8, 14, 6];
    println!("{}", vector[0]);
    println!("{}", vector[2]);
    vector[2] = 5;
    println!("{}", vector[2]);
}

#[cfg(test)]
mod test {
    use super::vector;

    #[test]
    fn test_vector() {
        vector();
    }
}
