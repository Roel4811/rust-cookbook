use serde::Deserialize;

#[derive(Deserialize)]
pub struct Recipe {
  pub name: String,
  pub price: i32,
  pub description: String
}

// impl Recipe {
//   pub fn new(name: &str, price: i32, location: &str) -> Recipe {
//       Recipe{
//           name: name.to_string(),
//           price: price,
//           description: location.to_string()
//       }
//   }
// }
