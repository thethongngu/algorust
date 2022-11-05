mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::merge_sort;
pub use self::quick_sort::quick_sort;
pub use self::selection_sort::selection_sort;

pub fn is_increasing<T>(arr: &[T]) -> bool
where
    T: Ord,
{
    if arr.len() <= 1 {
        return true;
    }

    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{
        bubble_sort::bubble_sort, heap_sort::heap_sort, insertion_sort, is_increasing, merge_sort,
        quick_sort, selection_sort,
    };

    #[test]
    fn utility_function() {
        assert!(is_increasing(&[1, 2, 3])); // normal
        assert!(is_increasing(&[1, 2, 2])); // equal
        assert!(is_increasing::<i32>(&[])); // empty
        assert!(is_increasing(&[-1000])); // negative
        assert!(!is_increasing(&[1, 2, 1])); // not increaseing
        assert!(!is_increasing(&[-1, -2])); // negative
    }

    fn normal_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![3, 7, 11, -4, 6, 1, 1];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    fn increasing_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![-3, 7, 9, 10, 20, 88];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    fn decreasing_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![88, 20, 10, 9, 7, -3];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    fn empty_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    fn one_element_only_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![4];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    fn two_element_only_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![4, -10];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    fn slice_case<F>(f: F)
    where
        F: FnOnce(&mut [i32]),
    {
        let mut arr = vec![4, 9, 0, 0, -2, -4, 5, 7, 9, 5, 2];
        f(&mut arr[2..10]);
        assert_eq!(arr[2..10], vec![-4, -2, 0, 0, 5, 5, 7, 9]);
    }

    fn string_type_case<F>(f: F)
    where
        F: FnOnce(&mut [&'static str]),
    {
        let mut arr = vec!["eond", "ase", "wowo", "", "taiwan"];
        f(&mut arr);
        assert!(is_increasing(&arr));
    }

    #[test]
    fn test_normal() {
        normal_case(bubble_sort);
        normal_case(insertion_sort);
        normal_case(selection_sort);
        normal_case(insertion_sort);
        normal_case(quick_sort);
        normal_case(merge_sort);
        normal_case(heap_sort);
    }

    #[test]
    fn test_increasing() {
        increasing_case(bubble_sort);
        increasing_case(insertion_sort);
        increasing_case(selection_sort);
        increasing_case(insertion_sort);
        increasing_case(quick_sort);
        increasing_case(merge_sort);
        increasing_case(heap_sort);
    }

    #[test]
    fn test_decreasing() {
        decreasing_case(bubble_sort);
        decreasing_case(insertion_sort);
        decreasing_case(selection_sort);
        decreasing_case(insertion_sort);
        decreasing_case(quick_sort);
        decreasing_case(merge_sort);
        decreasing_case(heap_sort);
    }

    #[test]
    fn test_empty() {
        empty_case(bubble_sort);
        empty_case(insertion_sort);
        empty_case(selection_sort);
        empty_case(insertion_sort);
        empty_case(quick_sort);
        empty_case(merge_sort);
        empty_case(heap_sort);
    }

    #[test]
    fn test_one_element_only() {
        one_element_only_case(bubble_sort);
        one_element_only_case(insertion_sort);
        one_element_only_case(selection_sort);
        one_element_only_case(insertion_sort);
        one_element_only_case(quick_sort);
        one_element_only_case(merge_sort);
        one_element_only_case(heap_sort);
    }

    #[test]
    fn test_two_element_only() {
        two_element_only_case(bubble_sort);
        two_element_only_case(insertion_sort);
        two_element_only_case(selection_sort);
        two_element_only_case(insertion_sort);
        two_element_only_case(quick_sort);
        two_element_only_case(merge_sort);
        two_element_only_case(heap_sort);
    }

    #[test]
    fn test_slice() {
        slice_case(bubble_sort);
        slice_case(insertion_sort);
        slice_case(selection_sort);
        slice_case(insertion_sort);
        slice_case(quick_sort);
        slice_case(merge_sort);
        slice_case(heap_sort);
    }

    #[test]
    fn test_string_type() {
        string_type_case(bubble_sort);
        string_type_case(insertion_sort);
        string_type_case(selection_sort);
        string_type_case(insertion_sort);
        string_type_case(quick_sort);
        string_type_case(merge_sort);
        string_type_case(heap_sort);
    }
}
