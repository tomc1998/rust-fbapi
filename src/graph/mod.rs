//! A module for representing the graph api. Contains definition for edges and
//! fields, for example.

pub enum Method {
  Get, Post, Update, Delete
}
impl Method {
  /// Returns "GET", "POST", "UPDATE", or "DELETE" depending on the value of
  /// this method.
  pub fn to_str(&self) -> &'static str {
    match *self {
      Method::Get => "GET",
      Method::Post => "POST",
      Method::Update => "UPDATE",
      Method::Delete => "DELETE",
    }
  }
}

/// An edge on the graph API. 
/// # Example
/// // Request a user with birthday
/// let mut me_req = Edge::user_me(); // Create a /me edge
/// me_req.add_fields(vec!["id", "birthday"]);
/// let user : User = fb_api.get(me_req, ACCESS_TOKEN);
///
/// // Request a picture
/// let mut picture_req = Edge::picture();
/// picture_req.add_parameters(vec![("height", "200")]);
/// let picture : UserPicture = fb_api.get(picture_req, ACCESS_TOKEN);
pub struct Edge {
  /// The path of the edge , for example `/me/picture`.
  path: String,

  /// Fields in an edge, for example `/me?fields=picture` where 'picture'
  /// is a field.
  fields: Vec<String>,

  method: Method,

  parameters: Vec<(String, String)>,
}

impl Edge {
  pub fn add_fields(&mut self, fields: Vec<&str>) {
    for f in fields {
      self.fields.push(f.to_owned());
    }
  }

  pub fn add_parameters(&mut self, parameters: Vec<(&str, &str)>) {
    for (name, val) in parameters {
      self.parameters.push((name.to_owned(), val.to_owned()));
    }
  }

  pub fn set_method(&mut self, m: Method) {
    self.method = m;
  }

  fn new() -> Edge {
    Edge {
      path: "".to_owned(),
      fields: Vec::new(),
      parameters: Vec::new(),
      method: Method::Get,
    }
  }

  /// Create an edge for requesting a user.
  pub fn user(id: &str) -> Edge {
    Edge {
      path: "/".to_owned() + id,
      ..  Edge::new()
    }
  }

  pub fn user_me() -> Edge {
    Edge::user("me")
  }

  pub fn get_path(&self) -> &str {
    return &self.path;
  }

  /// Build a HTTP request from the data in this edge.
  pub fn build_body(&self) -> String {
    let mut body = "".to_owned();
    // Add fields
    if self.fields.len() > 0 {
      body += "fields=";
      for ii in 0..self.fields.len() {
        let f = &self.fields[ii];

        // Check if last field to add
        if ii == self.fields.len()-1 {
          body += f;
        }
        else {
          body = body + f + ",";
        }
      }
    }

    // Add parameters
    for p in &self.parameters {
      body = body + "&" + &p.0 + "=" + &p.1;
    }

    // Add method parameter
    body = body + "&method=" + self.method.to_str();

    return body;
  }
}

