use serde::Deserialize;

#[derive(Deserialize)]
pub struct Recipe {
  pub id: i32,
  pub name: String,
  pub price: i32,
  pub description: String
}
