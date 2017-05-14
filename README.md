# rust-fbapi
Facebook graph API rest implementation in rust.

# Completeness
Not even close! This library is only currently used for personal use, and I'll
add features to it as I need them. 

# Running tests
To run the tests, a valid user access token is needed. This is currently
hardcoded into the tests/mod.rs file, as a static variable called
`ACCESS_TOKEN`.

# Example
This example is taken from `examples/get_user.rs`. Here we read the access
token from stdin, then use the access token to request the name and picture of
the user. We then print the user's name to the console.
```
extern crate fbapi;

fn main() {
  use std::io;

  // Read access token from stdin
  println!("Input access token: ");
  let mut buffer = String::new();
  let stdin = io::stdin();
  stdin.read_line(&mut buffer).unwrap();

  let access_token = &buffer;


  println!("Making request to facebook api...");

  let fb_api = fbapi::FBApi::new((2, 9)); // Initialise api version 2.9

  // Setup edge, request picture, gender, first name and last name
  let mut user_edge = fbapi::graph::Edge::user_me();
  user_edge.add_fields(vec!["picture", "gender", "first_name", "last_name"]);

  // Actually make the request and deserialize into fbapi::model::User type
  let user : fbapi::model::User = fb_api.get(user_edge, access_token).unwrap();

  // Print the result
  println!("User name = {} {}", user.first_name.unwrap(), user.last_name.unwrap());
}
```
