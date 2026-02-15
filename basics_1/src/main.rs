use std::mem::swap;

mod array_1;
mod search;
mod vec_1;

fn main() {
    println!("DSA learning in progress");

    let mut v = vec![2, 5, 3, 9, 5, 6, 1];

    bubble_sort(&mut v);
    //selection_sort(&mut v);
    insertion_sort(&mut v);
    // merge_sort(&mut v);
    println!("{:?}", v);
}

pub fn bubble_sort(v: &mut Vec<i32>) {
    println!("!------------------Sorting through selection sort------------------!");
    for i in 0..v.len() {
        let mut swapped = false;
        for j in 0..v.len() - i - 1 {
            if (v[j] > v[j + 1]) {
                v.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
// pub fn selection_sort(v: &mut Vec<i32>) {
//     println!("!------------------Sorting through selection sort------------------!");
//     let n = v.len();

//     for i in 0..n {
//         let mut min_idx = i;

//         for j in i + 1..n {
//             if v[j] < v[min_idx] {
//                 min_idx = j;
//             }
//         }

//         v.swap(i, min_idx);
//     }
// }

pub fn insertion_sort(v: &mut Vec<i32>) {
    println!("!------------------Sorting through insertion sort------------------!");

    let n = v.len();

    for i in 1..n {
        let key = v[i];
        let mut j = i;

        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1];
            j -= 1;
        }

        v[j] = key;
    }
}
pub fn merge_sort(v: &mut Vec<i32>) {
    todo!()
}
