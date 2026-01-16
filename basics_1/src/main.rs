fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let target = 4;

    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            println!("{}", v[i]);
        }
    }
    //linear search
    let output = linear_search(&v, target);
    println!("Linear search output is: {}", output);
}

//linear search
fn linear_search(v: &Vec<i32>, target: i32) -> i32 {
    let mut output: i32 = -1;
    for i in 0..v.len() {
        if v[i] == target {
            output = i as i32;
        }
    }
    output
}
