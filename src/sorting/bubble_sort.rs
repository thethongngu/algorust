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
