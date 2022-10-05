use super::Sorter;

pub struct InsertionSort;

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
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

    fn name(&self) -> &'static str {
        "Insertion Sort"
    }
}
