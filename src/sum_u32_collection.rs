fn sum_u32_collection(numbers: &[u32]) -> Option<u32> {
    let result = numbers.iter().try_fold(0u32, |acc, &x| acc.checked_add(x));
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_u32_collection() {
        let numbers1 = &[1, 2, 3, 4, 5];
        assert_eq!(Some(15), sum_u32_collection(numbers1));

        let numbers2 = &[u32::MAX, 2];
        assert_eq!(None, sum_u32_collection(numbers2));
    }
}