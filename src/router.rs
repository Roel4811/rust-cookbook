use colored::*;
use ::std::*;
use super::recipe_controller;

pub struct Router {
  pub recipe_controller: recipe_controller::RecipeController,
}

impl Router {
  pub fn new(recipe_controller: recipe_controller::RecipeController) -> Router {
    Router { recipe_controller }
  }

  pub fn run(&mut self) {
    self.clear_screen();
    println!("{}", "Welcome to Cookbook Rust!".blue().bold());

    let mut running = true;
    while running {
      self.show_actions();
      let action = self.get_user_input();
      self.clear_screen();
      running = self.route_action(action);
    }
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
  
  fn route_action(&mut self, action: i32) -> bool {
    match action {
      1 => self.recipe_controller.show_recipes(),
      2 => {
        self.recipe_controller.add_recipe();
        self.clear_screen();
        println!("{}", "Recipe added!".green())
      },
      3 => {
        self.recipe_controller.delete_recipe();
        self.clear_screen();
        println!("{}", "Recipe deleted!".green())
      },
      4 => {
        self.quit();
        return false
      },
      _ => {
        println!("{}", "Wrong input!".red());
      },
    };
    true
  }
  
  fn show_actions(&self) {
    println!("\n{}\n", "================".blue().bold());
    println!("What do you want to do? \n");
    println!("{}", "1 - Show recipes".italic().yellow());
    println!("{}", "2 - Add a recipes".italic().yellow());
    println!("{}", "3 - Delete a recipes".italic().yellow());
    println!("{}", "4 - Quit".italic().yellow());
  }
  
  fn quit(&self) {
    println!("Thank you for using the cookbook!");
  }
}


