use super::recipe::Recipe;
use std::error::Error;
use std::fs;

// #[derive(Debug)]
pub struct RecipeRepository {
  pub recipes: Vec<Recipe>,
  next_id: i32,
}

impl RecipeRepository {
  pub fn new() -> Result<RecipeRepository, csv::Error> {
    let recipes = import_csv()?;
    let next_id = (recipes.len() + 1) as i32;
    let recipe_repository = RecipeRepository {
      recipes: recipes,
      next_id,
    };
    Ok(recipe_repository)
  }

  pub fn add(&mut self, name: String, price: i32, description: String) {
    let id = self.next_id;
    self.next_id = self.next_id + 1;

    let recipe = Recipe {
      id,
      name,
      price,
      description,
    };
    self.recipes.push(recipe);
    match write_csv(&self.recipes) {
      Ok(_res) => (),
      Err(_err) => println!("Something went wrong saving your recipe(s)"),
    }
  }

  pub fn all(&self) -> &Vec<Recipe> {
    &self.recipes
  }

  pub fn delete(&mut self, id: i32) -> bool {
    if let Some(index) = self.recipes.iter().position(|x| x.id == id) {
      self.recipes.remove(index);
    } else {
      return false
    }

    match write_csv(&self.recipes) {
      Ok(_res) => true,
      Err(_err) => false,
    }
  }

  pub fn find(&mut self, id: i32) -> Option<&mut Recipe> {
    self.recipes.iter_mut().find(|x| x.id == id)
  }

  pub fn update(&mut self, id: i32, name: String, price: i32, description: String) -> bool {
    if let Some(recipe) = self.find(id) {
      recipe.name = name;
      recipe.price = price;
      recipe.description = description;
      true
    } else {
      false
    }
  }
}

fn write_csv(recipes: &Vec<Recipe>) -> Result<(), Box<dyn Error>> {
  let mut wtr = csv::Writer::from_path("recipes.csv")?;
  wtr.write_record(&["id", "name", "price", "description"])?;
  for recipe in recipes {
    wtr.write_record(&[
      &recipe.id.to_string(),
      &recipe.name,
      &recipe.price.to_string(),
      &recipe.description,
    ])?;
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
