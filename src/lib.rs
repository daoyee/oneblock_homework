mod bubble_sort;



#[cfg(test)]
mod tests {
    use super::bubble_sort::*;

    #[test]
    fn test_bubble_sort() {
        let mut vec1 = vec![6,5,3,200];
        bubble_sort(&mut vec1, SortDirection::ASC);
        assert_eq!(vec1,[3,5,6,200]);

        let mut vec2 = vec![6,5,3,200];
        bubble_sort(&mut vec2, SortDirection::DES);
        assert_eq!(vec2,[200,6,5,3]);
    }
}
