pub fn vec_1() {
    let mut v = vec![1, 2, 3, 4, 5];
    // v.push(2);
    // println!("{:?}",v);

    // for i in 0..v.len(){
    //      println!("{}",v[i]);
    //      //v.push(2*i); //since v.len() value is already decided at the start of loop, so the loop will run 6 times and print the first 5 elements only

    // }

    let mut i = 0;
    let mut j = v.len() - 1;

    //using while loop

    //  while i < j {
    //      swap(&mut v, i, j);

    //      i += 1;
    //      j -= 1;
    //  }

    //using for loop
    for i in 0..v.len() {
        if i < j {
            swap(&mut v, i, j);
            j -= 1;
        } else {
            break;
        }
    }

    println!("{:?}", v);
}

fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
    v[i] = v[i] + v[j];
    v[j] = v[i] - v[j];
    v[i] = v[i] - v[j];
}
