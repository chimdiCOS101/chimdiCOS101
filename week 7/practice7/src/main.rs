fn main() {
   //array with data type(explicit integer datatype)
   let arr1:[i32;4]=[10,20,30,40];
   println!("\n Array with data type");
   println!("array is {:?}", arr1);
   println!("array size is :{}",arr1.len());

   //array without data type (implicit float data type)
   let arr2 = [10.2,20.7,30.4,40.9,51.2,72.2];
   println!("\n array without data type ");
   println!("array is {:?}",arr2);
   println!("array size is {}",arr2.len());

   //array with default values that creates and
   //Initialize all it elements with a default value of -1
   let arr3:[i32;8] = [-1;8];
   println!("/n array with default values ");
   println!("array is {:?}",arr3);
   println!("array size is {}",arr3.len());
}
