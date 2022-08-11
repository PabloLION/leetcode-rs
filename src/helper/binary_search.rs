//! A binary search implementation.
//! vec should be sorted.
//! returns the last index of the element which meet the condition on index.
//! Should type better like this
//! fn binary_search(vec: Vec<dyn Sized>, condition: dyn Fn(dyn Sized) -> (bool)) {}

pub fn binary_search(vec: &Vec<i32>, condition: &(dyn Fn(usize) -> bool)) -> Option<usize> {
    let mut left = 0;
    let mut right = vec.len() - 1;
    while left <= right {
        let mid: usize = ((left + right) / 2).try_into().unwrap();
        if condition(mid) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    if left == vec.len() {
        None
    } else {
        Some(left)
    }
}

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let condition = |x: usize| vec[x] > 5;
    assert_eq!(binary_search(&vec, &condition), Some(5));
}
