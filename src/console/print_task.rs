use crate::task::get_task::get_task_by_id;
use crate::task::task_schema::Task;

pub fn print_task(id: &str, state: &mut Vec<Task>) {
  match get_task_by_id(id, state) {
      Some(task) => println!("{:?}", task),
      None => println!("Task not found"),
    }
}
