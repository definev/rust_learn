fn main() {
  // -----OWNERSHIP RULES-----
  // 1. A value of variable is owned the variable.
  // 2. Only one owner at a time.
  // 3. When the owner goes out of scope, the value is dropped.

  {
    // s is not available in here
    let s: String = String::from("Imutable string"); // s is owned by this scope
                                                     // s is available in here
  } // Out of scope, s is dropped

  println!("{}", s); // s is not available in here

  let s1 = gives_ownership(); // gives_ownership moves its return
                              // value into s1

  let s2 = String::from(s1); // s2 comes into scope

  let s3 = takes_and_gives_back(s2); // s2 is moved into
                                     // takes_and_gives_back, which also
                                     // moves its return value into s3

  println!("s3: {}", s3);

  takes_ownership(s3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
  // gives_ownership will move its
  // return value into the function
  // that calls it

  let some_string = String::from("yours"); // some_string comes into scope

  some_string // some_string is returned and
              // moves out to the calling
              // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
  // a_string comes into
  // scope

  a_string.to_owned() // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
  // Here, some_string goes out of scope and `drop` is called. The backing
}
