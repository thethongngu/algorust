use super::Sorter;

pub struct QuickSort;

impl QuickSort {
    fn partition<T: Sized + Ord>(slice: &mut [T], left: usize, right: usize) -> usize {
        let pivot_index = rand::random::<usize>() % (right - left + 1) + left;
        slice.swap(pivot_index, left);

        let mut start_half = left + 1;
        for i in (left + 1)..=right {
            if slice[left] > slice[i] {
                slice.swap(start_half, i);
                start_half += 1;
            }
        }

        slice.swap(left, start_half - 1);
        start_half - 1
    }

    fn qsort<T: Sized + Ord>(slice: &mut [T], left: usize, right: usize) {
        if left >= right {
            return;
        }
        let pivot_index = QuickSort::partition(slice, left, right);
        if pivot_index > 0 {
            QuickSort::qsort(slice, left, pivot_index - 1)
        };
        QuickSort::qsort(slice, pivot_index + 1, right);
    }
}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        if slice.len() <= 1 {
            return;
        }
        QuickSort::qsort(slice, 0, slice.len() - 1);
    }

    fn name(&self) -> &'static str {
        "Quick Sort"
    }
}
