use super::Sorter;

// Quicksort - pick an element at random, walk the list to put everything smaller than the element to one side, everything larger to the other side.
// Do this recursively until the vec is sorted.
// Imagine two pointers, each one pointing at the ends of the slice. We have a pivot at the first element 0, and the remainder of the slice. 
// The pointers point at the left/right EDGES of the elements, so don't include the elements.
// Look at the element to the right of left, if it is <= pivot then it should stay in left, so we move left 1 to the right.
// Now we have one element in left.
// If the next element to the right of left is not <= to the pivot, then we rotateright the element (so it goes onto the end of the vec)
// then we shift right left by 1 element. Now there is one element in right.

pub struct QuickSort;

fn quicksort<T: Ord> (slice: &mut [T]) {
    let (pivot, slice) = slice.split_first_mut().expect("this slice is always non-empty"); // Get mutable references to two subslices. First one is the pivot (item at location 0), second one is the rest of the slice. We need this because once we have any mutable reference into the slice, we can modify other parts of the slice, and therefore can't get additional mutable refs into the slice.
    let mut left = 0; 
    let mut right = slice.len() - 1;

    for i in 0..slice.len() {
        if slice[i] <= pivot {
            // already on correct side.
            left += 1;
        } else {
            // move element to right side.
            slice.swap(left, right);
            right -= 1;

        }
    }

    // let mut left = vec![];
    // let mut right = vec![];
    // for i in slice {
    //     if slice[i] <= pivot {
    //         left.push(slice[i]);
    //     } else {
    //         right.push(slice[i])
    //     }
    //     // Recurse
    //     quicksort(left);
    //     quicksort(right);
    //     // Merge them together

    // }

}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where 
        T: Ord 
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice);
        
        for unsorted in 1..slice.len() {


        }


    }
}

#[test]
fn it_works() {
    let mut things = vec![4,2,3,1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
}