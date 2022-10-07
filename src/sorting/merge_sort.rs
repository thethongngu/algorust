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
