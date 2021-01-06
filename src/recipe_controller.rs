use super::recipe_repository;
use super::recipe_view;

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
    let price = self.ask_user_and_convert_to_i32("Price");
    let description = self.recipe_view.ask_user_for("Description");
    self.recipe_repository.add(name, price, description);
  }

  pub fn ask_user_and_convert_to_i32(&self, name: &str) -> i32 {
    let price_string = self.recipe_view.ask_user_for(name);
    match convert_to_i32(price_string) {
      Ok(price) => price,
      Err(_) => self.ask_user_and_convert_to_i32(name),
    }
  }

  pub fn delete_recipe(&mut self) -> bool {
    self.show_recipes();
    let id = self.ask_user_and_convert_to_i32("ID");
    self.recipe_repository.delete(id)
  }

  pub fn show_recipes(&self) {
    let recipes = self.recipe_repository.all();
    self.recipe_view.show_recipes(&recipes);
  }

  pub fn update_recipe(&mut self) -> bool {
    self.show_recipes();
    let id = self.ask_user_and_convert_to_i32("ID");
    match self.recipe_repository.find(id) {
      Some(_) => {
        let name = self.recipe_view.ask_user_for("Name");
        let price = self.ask_user_and_convert_to_i32("Price");
        let description = self.recipe_view.ask_user_for("Description");
        self.recipe_repository.update(id, name, price, description)
      },
      None => false,
    }
  }
}

fn convert_to_i32(s: String) -> Result<i32, std::num::ParseIntError> {
  s.trim().parse::<i32>()
}