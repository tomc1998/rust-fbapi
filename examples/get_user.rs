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
  let mut user_edge = fbapi::graph::Edge::user_me();
  user_edge.add_fields(vec!["picture", "gender", "first_name", "last_name"]);
  let user : fbapi::model::User = fb_api.get(user_edge, access_token).unwrap();
  println!("User name = {} {}", user.first_name.unwrap(), user.last_name.unwrap());
}
