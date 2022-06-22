use super::Sorter;

// Just walks the array, swapping anything that's out of order. Very simple, very slow. 
// Walks repeatedly until nothing needs to be swapped anymore.
// Watch out for i+1 issues where we can exceed the bounds of the slice. i at end of slice 
// has already been compared with i in the previous step t-1 and therefore doesn't need any further comparison.

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where 
        T: Ord 
    {
            let mut swapped = true;
            while swapped { // Keep looping until we aren't swapping anything anymore. 
                swapped = false; // Assume we don't need to swap anymore.
                for i in 1..(slice.len()) { // Note bounds check in for predicate.
                    if slice[i-1] > slice[i] {
                        slice.swap(i-1, i); // Internally, uses memswap, nice rust feature.
                        swapped = true; // We swapped, we'll iterate again in the outer loop.
                    }
                }
            }
    }
}

#[test]
fn bubble_works() {
    let mut things = vec![4,2,3,1];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1,2,3,4])
}