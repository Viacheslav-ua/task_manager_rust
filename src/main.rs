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

fn main() {
    let task = Task::new(
        "Complete Rust project".to_owned(),
        "Finish the Rust project by the end of the week.".to_owned(),
        Priority::High,
    );

    task.print_task();
}
