use super::recipe_repository;

pub struct RecipeController {
  pub recipe_repository: recipe_repository::RecipeRepository,
}

impl RecipeController {
  pub fn new(recipe_repository: recipe_repository::RecipeRepository) -> RecipeController {
    RecipeController { recipe_repository }
  }

  pub fn add_recipe(&self) -> bool {
    println!("here you can add a recipe");
    true
  }
  pub fn show_recipes(&self) -> bool {
    println!("the recipes will be printed here");
    for recipe in &self.recipe_repository.recipes {
      println!(
        "name: {}, price: {}, description: {}",
        recipe.name, recipe.price, recipe.description
      )
    }
    true
  }
}
