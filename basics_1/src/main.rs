fn main() {
    let arr = [1,2,3,4,5];
    let mut sum =0;

    for i in 0..arr.len(){
        sum+=arr[i];
    }
    println!("Sum of all elements in array is: {}", sum);

    let arr2 = [3,7,2,9,4,4,9];
    let mut max = arr2[0];

    for i in 0..arr2.len(){
        if arr2[i]>max{
            max = arr2[i];
        }
    }

    println!("Max element in array2 is : {} ", max);

    let mut min = arr2[0];
    for i in 0..arr2.len(){
        if arr2[i]<min{
            min = arr2[i];
        }
    }
    println!("Min element in array2 is : {} ", min);

    
}
