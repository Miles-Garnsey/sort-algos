use super::Sorter;

// InsertionSort also sucks. Less efficient on large lists. Divides slice into sorted/not sorted.
// Sorted initially empty, not sorted is initially the original list.
// We're doing this by index here.
// Differs from bubblesort because bubble goes from left to right, doing one swap for each step, but then iterating again if any swaps were performed.
// Conversely, InsertionSort finds the right place in one hit (by moving an entry all the way back to the very start of the slice if required) and therefore avoids the outer loop.
// They're the same in terms of complexity, just depends on whether going left or right is better.

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where 
        T: Ord 
    {
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location in slice[..=unsorted]
            // Just keep moving the element you've just taken left until the element to the right is smaller than it.
            let mut i = unsorted;
            if self.smart {
                while i > 0 && slice[i-1] > slice[i] {
                    slice.swap(i-1, i);
                    i -= 1;
                }
            } else { 
                // We could try the below as an alternative.
                // This looks smarter, but a naive insert step risks pushing out the bounds of the vector, so 
                // wouldn't actually help avoid overhead from the swaps, which make sure we always have bounded memory.
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) { // Binary search may or may not find the required element. 
                    Ok(i) => i, // Found an element with same value, so can place it before or after.
                    Err(i) => i, // Did not find an element with the requested value, but returns the index for where the unsorted element WOULD have been if it had been found (so place unsorted there).
                };
                // But if we don't 'insert' and instead rotateright, this shifts all elements in the slice over by 1 with wraparound,
                // Avoids re-allocating due to vec busting out of its allocation.
                slice[i..=unsorted].rotate_right(1); //rotateright starting at the target location and ending at our existing location. 
                // So the current element (which is unsorted) then goes to the target location due to wraparound, while the left hand side of the array (which is sorted) just gets an insert of the unsorted element.
            }
        }


    }
}

#[test]
fn insertion_works() {
    let mut things = vec![4,2,3,1];
    InsertionSort{ smart: true }.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
    InsertionSort{ smart: false }.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
}