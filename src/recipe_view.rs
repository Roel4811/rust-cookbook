use ::std::*;
use super::recipe::Recipe;

pub struct RecipeView {}

impl RecipeView {
  pub fn ask_user_for(&self, stuff: &str) -> String {
    println!("Please give me a {}", stuff);
    let mut x = String::with_capacity(10);
    io::stdin().read_line(&mut x).expect("Error reading input");
    let input = x.trim();
    String::from(input)
  }

  pub fn show_recipes(&self, recipes: &Vec<Recipe>) {
    for recipe in recipes {
      println!(
        "name: {}, price: {}, description: {}",
        recipe.name, recipe.price, recipe.description
      )
    }
  }
}