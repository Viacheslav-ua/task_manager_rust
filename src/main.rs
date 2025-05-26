use core::task;
use std::{result, vec};

use chrono::{DateTime, Local};


enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned(),
        }
    }
}
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time:  DateTime<Local>
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self {
            name,
            description,
            priority,
            add_time: Local::now(),
        }
    }

    fn print_task(&self) {
        println!("Task Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Priority: {}", self.priority.to_string());
        println!("Added on: {}", self.add_time.format("%d-%m-%Y %H:%M:%S"));
        println!("-------------------------");}
}

struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed successfully.", name))
            
        } else {
            Err(format!("Task \"{}\" not found.", name))
        }
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;
                    Ok(format!("Task \"{}\" updated successfully.", task.name))
                }
                None => Err("Error borrowing task.".to_owned()),
                
            }
        } else {
            Err(format!("Task \"{}\" not found.", name))
        }
    }
}

fn main() {
    let task = Task::new(
        "Complete Rust project".to_owned(),
        "Finish the Rust project by the end of the week.".to_owned(),
        Priority::High,
    );

    task.print_task();
}
