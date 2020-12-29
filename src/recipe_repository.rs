use super::recipe::Recipe;

// #[derive(Debug)]
pub struct RecipeRepository {
  pub recipes: Vec<Recipe>,
}

impl RecipeRepository {
  pub fn new() -> Result<RecipeRepository, csv::Error> {
    let recipes = import_csv()?;
    let recipe_repository = RecipeRepository {
      recipes: recipes,
    };
    Ok(recipe_repository)
  }

  pub fn add(&self, recipe: Recipe) -> bool {
    self.recipes.push(recipe);
    true
  }

  pub fn all(&self) -> &Vec<Recipe> {
    &self.recipes
  }
}

fn import_csv() -> Result<Vec<Recipe>, csv::Error> {
  let csv = "name,price,description
  pizza,10,Margarita
  spaghetti,12,Vongole";

  let mut reader = csv::Reader::from_reader(csv.as_bytes());
  let mut recipes = vec![];

  for recipe in reader.deserialize() {
    let recipe: Recipe = recipe?;
    recipes.push(recipe);
  }

  Ok(recipes)
}
