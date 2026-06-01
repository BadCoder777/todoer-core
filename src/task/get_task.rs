use crate::task::task_schema::Task;

pub fn get_task_by_id<'a>(id: &str, state: &'a Vec<Task>) -> Option<&'a Task> {
  state.iter().find(|item| item.id == id)
}
