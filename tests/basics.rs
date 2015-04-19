extern crate zipWith;

#[cfg(test)]
mod tests {
    use zipWith::IntoZipWith;
    use std::iter::Iterator;

    #[test]
    fn zipping_two_lists_of_numbers_with_plus_returns_their_sum () {
        let left: Vec<u8> = vec![1, 2, 3];
        let right: Vec<u8> = vec![4, 5, 6];

        let result: Vec<u8> = left.zip_with(right, | l, r | l + r).collect();

        assert_eq!(vec![5, 7, 9], result);
    }
}
