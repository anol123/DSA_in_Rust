use std::mem::swap;

mod array_1;
mod search;
mod vec_1;

fn main() {
    println!("DSA learning in progress");

    let mut v = vec![2, 5, 3, 9, 5, 6, 1];

    bubble_sort(&mut v);
    println!("{:?}", v);
    // selection_sort(&mut v);
    // insertion_sort(&mut v);
    // merge_sort(&mut v);
}

pub fn bubble_sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        let mut swapped = false;
        for j in 0..v.len() - i - 1 {
            if (v[i] > v[j + 1]) {
                v.swap(i, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
pub fn selection_sort(v: &mut Vec<i32>) {
    todo!()
}
pub fn insertion_sort(v: &mut Vec<i32>) {
    todo!()
}
pub fn merge_sort(v: &mut Vec<i32>) {
    todo!()
}
