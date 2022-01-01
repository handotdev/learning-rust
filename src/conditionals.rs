// Conditionals - Used to check and act on results

pub fn run() {
  let age: u8 = 18;
  let check_id: bool = true;
  let knows_person_of_age = true;

  // If-else
  if age >= 21 && check_id || knows_person_of_age {
    println!("You can have a beer!");
  } else if age < 21 && check_id {
    println!("You have to leave");
  } else {
    println!("Need ID");
  }

  // Shorthand if
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of age: {}", is_of_age);
}
