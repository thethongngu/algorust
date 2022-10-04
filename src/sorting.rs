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
