use crate::task::task_schema::Task;

pub fn print_all_tasks(state: &[Task]) {
  for task in state {
    println!("{:?}", task);
  }
}
