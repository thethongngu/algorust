pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    for i in 0..arr.len() {
        let mut sorted = true;
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::bubble_sort::*;
    use crate::sorting::*;

    #[test]
    fn normal() {
        let mut arr = vec![3, 7, 11, -4, 6, 1, 1];
        bubble_sort(&mut arr);
        assert!(is_increasing(&arr));
    }

    #[test]
    fn increasing() {
        let mut arr = vec![-3, 7, 9, 10, 20, 88];
        bubble_sort(&mut arr);
        assert!(is_increasing(&arr));
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        bubble_sort(&mut arr);
        assert!(is_increasing(&arr));
    }

    #[test]
    fn one_element_only() {
        let mut arr = vec![-4];
        bubble_sort(&mut arr);
        assert!(is_increasing(&arr));
    }
}
