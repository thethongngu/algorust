pub mod bubble_sort;

pub fn is_increasing<T>(arr: &[T]) -> bool
where
    T: PartialOrd,
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
    use super::is_increasing;

    #[test]
    fn all_cases() {
        assert!(is_increasing(&[1, 2, 3])); // normal
        assert!(is_increasing(&[1, 2, 2])); // equal
        assert!(is_increasing::<i32>(&[])); // empty
        assert!(is_increasing(&[-1000])); // negative
        assert!(!is_increasing(&[1, 2, 1])); // not increaseing
        assert!(!is_increasing(&[-1, -2])); // negative
    }
}
