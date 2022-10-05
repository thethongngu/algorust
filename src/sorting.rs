mod bubble_sort;
mod insertion_sort;
mod quick_sort;
mod selection_sort;

pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;

    fn name(&self) -> &'static str;
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
        bubble_sort::BubbleSort, insertion_sort::InsertionSort, is_increasing,
        quick_sort::QuickSort, selection_sort::SelectionSort, Sorter,
    };

    struct SortAlgo<T> {
        algos: Vec<Box<dyn Sorter<T>>>,
    }

    impl<T> SortAlgo<T> {
        fn new() -> Self {
            SortAlgo {
                algos: vec![
                    Box::new(BubbleSort),
                    Box::new(InsertionSort),
                    Box::new(SelectionSort),
                    Box::new(QuickSort),
                ],
            }
        }
    }

    #[test]
    fn utility_function() {
        assert!(is_increasing(&[1, 2, 3])); // normal
        assert!(is_increasing(&[1, 2, 2])); // equal
        assert!(is_increasing::<i32>(&[])); // empty
        assert!(is_increasing(&[-1000])); // negative
        assert!(!is_increasing(&[1, 2, 1])); // not increaseing
        assert!(!is_increasing(&[-1, -2])); // negative
    }

    #[test]
    fn normal() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = vec![3, 7, 11, -4, 6, 1, 1];
            algo.sort(&mut arr);
            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr), "Incorrect: {}", algo.name());
        }
    }

    #[test]
    fn increasing() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = vec![-3, 7, 9, 10, 20, 88];
            algo.sort(&mut arr);
            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr), "Incorrect: {}", algo.name());
        }
    }

    #[test]
    fn decreasing() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = vec![88, 20, 10, 9, 7, -3];
            algo.sort(&mut arr);
            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr), "Incorrect: {}", algo.name());
        }
    }

    #[test]
    fn empty() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = Vec::<i32>::new();
            algo.sort(&mut arr);
            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr), "Incorrect: {}", algo.name());
        }
    }

    #[test]
    fn one_element_only() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = vec![4];
            algo.sort(&mut arr);
            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr), "Incorrect: {}", algo.name());
        }
    }

    #[test]
    fn two_element_only() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = vec![4, -10];
            algo.sort(&mut arr);
            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr), "Incorrect: {}", algo.name());
        }
    }

    #[test]
    fn slice_of_vec() {
        let sorts = SortAlgo::new();
        for algo in sorts.algos {
            let mut arr = vec![4, 9, 0, 0, -2, -4, 5, 7, 9, 5, 2];
            algo.sort(&mut arr[2..10]);

            println!("{}: {:?}", algo.name(), arr);
            assert!(is_increasing(&arr[2..10]), "Incorrect: {}", algo.name());
            assert_eq!(
                arr[2..10],
                vec![-4, -2, 0, 0, 5, 5, 7, 9],
                "Incorrect: {}",
                algo.name()
            );
        }
    }
}
