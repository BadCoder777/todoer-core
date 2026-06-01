use crate::task::task_schema::Task;

pub fn add_task(task: Task, state: &mut Vec<Task>) {
  state.push(task);
}
