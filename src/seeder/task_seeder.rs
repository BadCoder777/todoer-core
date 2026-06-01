use crate::task::task_schema::Task;
use crate::task::add_task::add_task;

pub fn task_seeder(state: &mut Vec<Task>) {
  add_task(
    Task {
      title: "Task 1".to_string(),
      description: "description 1".to_string(),
      priority: 3,
      id: "gjgjjj".to_string(),
    },
    state
  );
  add_task(
    Task {
      title: "Task 2".to_string(),
      description: "description 2".to_string(),
      priority: 1,
      id: "gjggbjhfv".to_string(),
    },
    state
  );
}
