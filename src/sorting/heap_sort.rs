fn move_down<T>(slice: &mut [T], mut pos: usize)
where
    T: Ord,
{
    loop {
        let left_child = pos * 2 + 1;
        let right_child = left_child + 1;

        if left_child >= slice.len() {
            return;
        }

        if right_child >= slice.len() {
            if slice[pos] < slice[left_child] {
                slice.swap(pos, left_child);
            }
            pos = left_child;
        } else {
            let max = if slice[left_child] > slice[right_child] {
                left_child
            } else {
                right_child
            };
            if slice[pos] < slice[max] {
                slice.swap(pos, max);
            }
            pos = max;
        }
    }
}

fn heapify<T>(slice: &mut [T])
where
    T: Ord,
{
    for node in (0..slice.len() / 2).rev() {
        move_down(slice, node);
    }
}

pub fn heap_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    heapify(slice);

    for last in (0..slice.len()).rev() {
        slice.swap(0, last);
        move_down(&mut slice[0..last], 0);
    }
}
