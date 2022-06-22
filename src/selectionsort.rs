use super::Sorter;

// SelectionSort is an in-place algo. It is even worse in complexity than InsertionSort.
// We just find smallest element of the list, then move to front, then find smallest of remainder of the list, then move to front.
// Very slow but uses no extra memory because nothing is stored at all.

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where 
        T: Ord 
    {
        for unsorted in 0..slice.len() { // note the 0 here, differs from previous examples.
            let (smallest_in_rest, _) = slice[unsorted..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v) //minbykey gives a reference to each el, instead of giving ownership. But the reference to tuple only lives for the duration of the iteration. So we can't use the tuple. As a result, we need to tell the closure to deref the tuple.
            .expect("slice is non-empty");// returns an option in case the iterator is empty. Expect unwraps option.
            //or...
            // let mut smallest_in_rest = unsorted;
            // for i in (unsorted + 1)..slice.len() { //Equivalent of just finding min().
            //     smallest_in_rest = i
            // }
            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest)
            }
        }
    }
}

#[test]
fn Selection_works() {
    let mut things = vec![4,2,3,1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
}