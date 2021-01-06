use super::recipe::Recipe;
use ::std::*;
use colored::*;

pub struct RecipeView {}

impl RecipeView {
  pub fn ask_user_for(&self, stuff: &str) -> String {
    println!("\nPlease give me a {}", stuff);
    let mut x = String::with_capacity(10);
    io::stdin().read_line(&mut x).expect("Error reading input");
    let input = x.trim();
    String::from(input)
  }

  pub fn show_recipes(&self, recipes: &Vec<Recipe>) {
    println!(
      "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
      "ID", "Name", "Price", "Description"
    );

    for recipe in recipes {
      println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        recipe.id, recipe.name, recipe.price, recipe.description
      );
    }
  }
}
