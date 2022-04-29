fn main() {
    println!("Hello, world!");
    let m = find_median(vec![3, 1, 1, 9, 999, 9999]);
    println!("Median was: {:?}", m);
    assert_eq!(m, Some((9 + 3) / 2));
    let m = find_median(vec![]);
    assert_eq!(m, None);
}

fn find_median(mut list: Vec<i32>) -> Option<i32> {
    if list.is_empty() {
        return None;
    }
    let len = list.len();
    println!("list contains {} elements", len);
    let middle = (len - 1) / 2;
    println!("Middle index is {}", middle);
    // unstable sort is faster, not that this matters much in a toy example
    println!("List before sorting: {:?}", list);
    list.sort_unstable();
    println!("List after sorting: {:?}", list);
    Some(match len % 2 {
        // if list is odd, only one middle value.
        1 => list[middle],
        // if list is even, average the two middle values.
        0 => (list[middle] + list[middle + 1]) / 2,
        v => panic!("Modulo 2 gave: {}", v),
    })
}

