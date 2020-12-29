use super::recipe::Recipe;
use std::error::Error;
use std::fs;

// #[derive(Debug)]
pub struct RecipeRepository {
  pub recipes: Vec<Recipe>,
}

impl RecipeRepository {
  pub fn new() -> Result<RecipeRepository, csv::Error> {
    let recipes = import_csv()?;
    let recipe_repository = RecipeRepository { recipes: recipes };
    Ok(recipe_repository)
  }

  pub fn add(&mut self, recipe: Recipe) {
    self.recipes.push(recipe);
    match write_csv(&self.recipes) {
      Ok(_res) => (),
      Err(_err) => (),
    }
  }

  pub fn all(&self) -> &Vec<Recipe> {
    &self.recipes
  }
}

fn write_csv(recipes: &Vec<Recipe>) -> Result<(), Box<dyn Error>> {
  let mut wtr = csv::Writer::from_path("recipes.csv")?;
  wtr.write_record(&["name", "price", "description"])?;
  for recipe in recipes {
    wtr.write_record(&[&recipe.name, &recipe.price.to_string(), &recipe.description])?;
  }
  wtr.flush()?;
  Ok(())
}

fn import_csv() -> Result<Vec<Recipe>, csv::Error> {
  let csv = fs::read_to_string("recipes.csv").unwrap();
  let mut reader = csv::Reader::from_reader(csv.as_bytes());
  let mut recipes = vec![];

  for recipe in reader.deserialize() {
    let recipe: Recipe = recipe?;
    recipes.push(recipe);
  }

  Ok(recipes)
}
