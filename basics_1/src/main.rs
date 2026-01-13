mod array_1;
fn main() {
   let mut v = vec![1,2,3,4,5];
   v.push(2);
   println!("{:?}",v);  

   for i in 0..v.len(){
        println!("{}",v[i]);
        //v.push(2*i); //since v.len() value is already decided at the start of loop, so the loop will run 6 times and print the first 5 elements only

   }
}
