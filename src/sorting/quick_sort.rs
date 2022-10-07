fn partition<T>(slice: &mut [T], left: usize, right: usize) -> usize
where
    T: Ord,
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
    T: Ord,
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
