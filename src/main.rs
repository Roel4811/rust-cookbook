mod recipe;
mod recipe_controller;
mod recipe_repository;
mod recipe_view;
mod router;

fn main() {
    let recipe_repository = recipe_repository::RecipeRepository::new().unwrap();
    let recipe_controller = recipe_controller::RecipeController::new(recipe_repository);
    let mut router = router::Router::new(recipe_controller);
    let result = router.run().unwrap();
    println!("result: {}", result)
}
