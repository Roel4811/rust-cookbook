use ::std::*;

use super::recipe_controller;
use super::recipe_repository;
use std::error;

pub struct Router {
  pub recipe_controller: recipe_controller::RecipeController,
}

impl Router {
  pub fn new(recipe_controller: recipe_controller::RecipeController) -> Router {
    Router { recipe_controller }
  }

  pub fn run(&self) -> Result<String, Box<dyn error::Error>> {
    println!("Hi there! Welcome to the cookbook! What do you want to do?");
    let mut running = true;
    let recipe_repository = recipe_repository::RecipeRepository::new()?;
    for recipe in recipe_repository.recipes {
      let name = String::from(recipe.name);
      print!("We found a recipe: {}\n", name)
    }
  
    while running {
      self.show_actions();
      let action = self.get_user_input();
      self.clear_screen();
      running = self.route_action(action);
    }
  
    let result = String::from("Done");
    Ok(result)
  }
  
  fn clear_screen(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  }
  
  fn get_user_input(&self) -> i32 {
    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");
    let action = x.trim().parse::<i32>();
    return match action {
      Ok(n) => n,
      _ => 9,
    };
  }
  
  fn route_action(&self, action: i32) -> bool {
    return match action {
      1 => self.recipe_controller.show_recipes(),
      2 => self.recipe_controller.add_recipe(),
      3 => self.quit(),
      _ => {
        println!("Wrong input!");
        true
      }
    };
  }
  
  fn show_actions(&self) {
    println!("\n ================ \n");
    println!("What do you want to do?");
    println!("1 - Show recipes");
    println!("2 - Add a recipes");
    println!("3 - Quit");
  }
  
  pub fn quit(&self) -> bool {
    println!("Thank you for using the cookbook!");
    false
  }
}


