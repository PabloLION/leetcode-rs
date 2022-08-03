//! A binary search implementation.
//! returns the last index of the element which meet the condition.
pub fn binary_search(vec: Vec<i32>, condition: &(dyn Fn(i32) -> bool + 'static)) -> Option<usize> {
    // this is a fake implementation
    for i in 0..vec.len() {
        if condition(vec[i]) {
            return Some(i);
        }
    }
    return None;
}

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let condition = |x: i32| x > 5;
    assert_eq!(binary_search(vec, &condition), Some(5));
}
// fn binary_search(vec: Vec<dyn Sized>, condition: dyn Fn(dyn Sized) -> (bool)) {}
