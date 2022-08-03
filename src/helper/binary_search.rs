//! A binary search implementation.
//! returns the last index of the element which meet the condition.
pub fn binary_search(vec: Vec<i32>, condition: &(dyn Fn(i32) -> bool + 'static)) -> Option<usize> {
    let mut left = 0;
    let mut right = vec.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if condition(vec[mid]) {
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
    let condition = |x: i32| x > 5;
    assert_eq!(binary_search(vec, &condition), Some(5));
}
// fn binary_search(vec: Vec<dyn Sized>, condition: dyn Fn(dyn Sized) -> (bool)) {}
