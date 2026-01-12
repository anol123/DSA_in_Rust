mod array_1;
fn main() {
   let v = vec![1,2,3,4,5];
   println!("{:?}",v);  

   for i in 0..v.len(){
        println!("{}",v[i]);
   }
}
