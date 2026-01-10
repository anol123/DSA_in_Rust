fn main() {
    let arr = [1,2,3,4,5];
    let mut sum =0;

    for i in 0..arr.len(){
        sum+=arr[i];
    }
    println!("Sum of all elements in array is: {}", sum);
    
}
