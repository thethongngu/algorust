use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
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

    fn name(&self) -> &'static str {
        "Selection Sort"
    }
}
