use super::recipe_repository;
use super::recipe_view;
use super::recipe::Recipe;

pub struct RecipeController {
  pub recipe_repository: recipe_repository::RecipeRepository,
  pub recipe_view: recipe_view::RecipeView,
}

impl RecipeController {
  pub fn new(recipe_repository: recipe_repository::RecipeRepository) -> RecipeController {
    RecipeController {
      recipe_repository,
      recipe_view: recipe_view::RecipeView {},
    }
  }

  pub fn add_recipe(&mut self) {
    let name = self.recipe_view.ask_user_for("Name");
    let price = self.get_price();
    let description = self.recipe_view.ask_user_for("Description");
    let recipe = Recipe { name, price, description};
    self.recipe_repository.add(recipe);
  }

  pub fn get_price(&self) -> i32 {
    let price_string = self.recipe_view.ask_user_for("Price");
    match price_string.trim().parse::<i32>() {
      Ok(price) => price,
      Err(_) => self.get_price(),
    }
  }

  pub fn show_recipes(&self) {
    let recipes = self.recipe_repository.all();
    self.recipe_view.show_recipes(&recipes);
  }
}
