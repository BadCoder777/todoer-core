use crate::{console::print_all_tasks::print_all_tasks, seeder::task_seeder::task_seeder};

mod task;
mod state;
mod console;
mod seeder;

fn main() {
  let mut state = state::init_state::init_state().state;
  task_seeder(&mut state);
  println!("Welcome to todoer!");
  print_all_tasks(&state);
}
