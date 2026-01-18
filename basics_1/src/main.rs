fn main() {
    let mut v = vec![17, 3, 0, 1, 8, 6, 54, 89, 46, 24, 22];
    let target = 24;

    println!("length of vector: {}", v.len());

    //print even elements
    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            println!("{}", v[i]);
        }
    }

    //since the unsorted vector will be sorted so the vector will get changed and hence we get a different result
    //binary search
    println!(
        "Binary search output is: {}",
        binary_search(&mut v, &target)
    );

    //linear search
    let output = linear_search(&v, &target);
    println!("Linear search output is: {}", output);
}

//linear search
fn linear_search(v: &Vec<i32>, target: &i32) -> i32 {
    let mut output: i32 = -1;
    for i in 0..v.len() {
        if v[i] == *target {
            output = i as i32;
        }
    }
    output
}

fn binary_search(v: &mut Vec<i32>, target: &i32) -> i32 {
    v.sort();

    let mut low = 0;
    let mut high = v.len() - 1;

    // while low <= high {
    //     let mid = low + (high - low) / 2;
    //     if v[mid] == *target {
    //         return mid as i32;
    //     } else if v[mid] > *target {
    //         high = mid - 1;
    //     } else {
    //         low = mid + 1;
    //     }
    // }

    for mut i in 0..v.len() / 2 {
        if low <= high {
            let mid = low + (high - low) / 2;
            if v[mid] == *target {
                return mid as i32;
            } else if v[mid] > *target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        i += 1;
    }
    return -1;
}
