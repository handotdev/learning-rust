// Reference pointers - point to a resource in memory

pub fn run() {
  let arr1 = [1,2,3];
  let arr2 = arr1;

  // With non-primitive types, if you assign one to another,
  // the first will be a copy of the value. You'll need to use a reference (&)
  // to point to the resource

  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, vec2));
}