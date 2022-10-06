pub fn bubble_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    if slice.len() <= 1 {
        return;
    }

    for i in 0..slice.len() {
        let mut sorted = true;
        for j in 0..(slice.len() - i - 1) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

pub fn insertion_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j - 1] > slice[i] {
            j -= 1;
        }

        for k in j..i {
            slice.swap(k, i);
        }
    }
}

pub fn selection_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    for i in 0..slice.len() {
        let mut max_pos = i;
        for j in (i + 1)..slice.len() {
            if slice[j] < slice[max_pos] {
                max_pos = j;
            }
        }
        slice.swap(i, max_pos);
    }
}

fn partition<T>(slice: &mut [T], left: usize, right: usize) -> usize
where
    T: Sized + Ord,
{
    // Choose random index and swap it with left-most item
    let pivot_index = rand::random::<usize>() % (right - left + 1) + left;
    slice.swap(pivot_index, left);

    // partition range from [left + 1..right] into 2 partitions (`start_half` indicates start of second half)
    let mut start_half = left + 1;
    for i in (left + 1)..=right {
        if slice[left] > slice[i] {
            slice.swap(start_half, i);
            start_half += 1;
        }
    }

    // swap pivot back to separation index of 2 halves, return pivot index
    slice.swap(left, start_half - 1);
    start_half - 1
}

fn qsort<T>(slice: &mut [T], left: usize, right: usize)
where
    T: Sized + Ord,
{
    if left >= right {
        return;
    }
    let pivot_index = partition(slice, left, right);

    // usize not allow negative value when compute `pivot_index - 1`
    if pivot_index > 0 {
        qsort(slice, left, pivot_index - 1)
    };
    qsort(slice, pivot_index + 1, right);
}

pub fn quick_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    // usize not allow negative calculation (panic if remove this)
    if slice.len() <= 1 {
        return;
    }
    qsort(slice, 0, slice.len() - 1);
}

fn merge_sort_helper<T>(slice: &mut [T], left: usize, right: usize)
where
    T: Ord + Clone,
{
    if left >= right {
        return;
    }

    // sort 2 smaller partitions
    let mid = left + (right - left) / 2;
    merge_sort_helper(slice, left, mid);
    merge_sort_helper(slice, mid + 1, right);

    // combine 2 sorted partitions to temporarilt vector
    let mut tmp: Vec<T> = Vec::new();
    tmp.reserve(right - left + 1);
    let mut left_id = left;
    let mut right_id = mid + 1;

    while left_id <= mid && right_id <= right {
        if slice[left_id] < slice[right_id] {
            tmp.push(slice[left_id].clone());
            left_id += 1;
        } else {
            tmp.push(slice[right_id].clone());
            right_id += 1;
        }
    }
    while left_id <= mid {
        tmp.push(slice[left_id].clone());
        left_id += 1;
    }
    while right_id <= right {
        tmp.push(slice[right_id].clone());
        right_id += 1;
    }

    // move to original slice
    for index in (left..=right).rev() {
        slice[index] = tmp.pop().unwrap();
    }
}

pub fn merge_sort<T>(slice: &mut [T])
where
    T: Ord + Clone,
{
    // usize not allow negative calculation (panic if remove this)
    if slice.len() <= 1 {
        return;
    }
    merge_sort_helper(slice, 0, slice.len() - 1);
}

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
        bubble_sort, insertion_sort, is_increasing, merge_sort, quick_sort, selection_sort,
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

    #[test]
    fn test_normal() {
        normal_case(bubble_sort);
        normal_case(insertion_sort);
        normal_case(selection_sort);
        normal_case(insertion_sort);
        normal_case(quick_sort);
        normal_case(merge_sort);
    }

    #[test]
    fn test_increasing() {
        increasing_case(bubble_sort);
        increasing_case(insertion_sort);
        increasing_case(selection_sort);
        increasing_case(insertion_sort);
        increasing_case(quick_sort);
        increasing_case(merge_sort);
    }

    #[test]
    fn test_decreasing() {
        decreasing_case(bubble_sort);
        decreasing_case(insertion_sort);
        decreasing_case(selection_sort);
        decreasing_case(insertion_sort);
        decreasing_case(quick_sort);
        decreasing_case(merge_sort);
    }

    #[test]
    fn test_empty() {
        empty_case(bubble_sort);
        empty_case(insertion_sort);
        empty_case(selection_sort);
        empty_case(insertion_sort);
        empty_case(quick_sort);
        empty_case(merge_sort);
    }

    #[test]
    fn test_one_element_only() {
        one_element_only_case(bubble_sort);
        one_element_only_case(insertion_sort);
        one_element_only_case(selection_sort);
        one_element_only_case(insertion_sort);
        one_element_only_case(quick_sort);
        one_element_only_case(merge_sort);
    }

    #[test]
    fn test_two_element_only() {
        two_element_only_case(bubble_sort);
        two_element_only_case(insertion_sort);
        two_element_only_case(selection_sort);
        two_element_only_case(insertion_sort);
        two_element_only_case(quick_sort);
        two_element_only_case(merge_sort);
    }

    #[test]
    fn test_slice() {
        slice_case(bubble_sort);
        slice_case(insertion_sort);
        slice_case(selection_sort);
        slice_case(insertion_sort);
        slice_case(quick_sort);
        slice_case(merge_sort);
    }
}
