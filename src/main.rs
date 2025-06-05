use std::{fs::File, io::{BufReader, Write}, path::Path, vec};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>,
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

    fn new_from_console() -> Self {
        let name = match ConsoleManager::input("Enter task name") {
            Ok(name) => name,
            Err(err) => panic!("Error getting task name: {err}"),
        };

        let description = match ConsoleManager::input("Enter task description") {
            Ok(description) => description,
            Err(err) => panic!("Error getting description: {err}"),
        };

        let priority = match ConsoleManager::input("Enter task priority (Low, Medium, High)") {
            Ok(priority_string) => match priority_string.to_lowercase().as_str() {
                "low" => Priority::Low,
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => {
                    eprintln!("Invalid priority. Defaulting to Low.");
                    Priority::Low
                }
            },
            Err(err) => panic!("Error getting priority: {err}"),
        };
        
        Self::new(name, description, priority)
    }

    fn print_task(&self) {
        println!("Task Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Priority: {}", self.priority.to_string());
        println!("Added on: {}", self.add_time.format("%d-%m-%Y %H:%M:%S"));
        println!("-------------------------");
    }
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

    fn store_to_file(&self, file_path: &str) -> Result<String, String> {
        if !Path::new(file_path).exists() {
            let file = match File::create(file_path) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {err}").to_owned()),
            };

            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("File created successfully.".to_owned()),
                Err(err) => Err(format!("Error writing to file: {err}")),
            }
            
        } else {
            Err(format!("File \"{}\" already exists.", file_path).to_owned())
        }
    }
    fn read_from_file(&mut self, file_path: &str) -> Result<String, String> {
        if Path::new(file_path).exists() {
            let file = match File::open(file_path) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error opening file: {err}")),
            };

            let reader = BufReader::new(file);

            self.tasks = match serde_json::from_reader(reader) {
                Ok(tasks) => tasks,
                Err(err) => return Err(format!("Error reading from file: {err}")),
            };

            Ok(format!("Tasks loaded successfully from \"{}\".", file_path))
        } else {
            return Err(format!("File \"{}\" does not exist.", file_path).to_owned());
        }
    }
}

struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add Task".to_owned(),
                "Find Task".to_owned(),
                "Edit Task".to_owned(),
                "Remove Task".to_owned(),
                "Print Tasks".to_owned(),
                "Srore tasks to file".to_owned(),
                "Read tasks from file".to_owned(),
            ],
        }
    }

    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}: ", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("\nEnter command index") {
            Ok(command) => match command.as_str() {
                "1" => {
                    self.tasks_manager.add_task(Task::new_from_console());
                }

                "2" => {
                    let name = match Self::input("Enter task name to find") {
                        Ok(name) => name,
                        Err(err) => {
                            eprintln!("Error getting task name: {err}");
                            return;
                        }
                    };

                    match self.tasks_manager.find_task(&name) {
                        Some(index) => {
                            println!("Task found:");
                            self.tasks_manager.tasks[index].print_task();
                        },
                        None => println!("Task \"{}\" not found.", name),
                    }
                }
                
                "3" => {
                    let name = match Self::input("Enter task name to edit") {
                        Ok(name) => name,
                        Err(err) => {
                            eprintln!("Error getting task name: {err}");
                            return;
                        }
                    };

                    match self
                        .tasks_manager
                        .edit_task(&name, Task::new_from_console())
                    {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => eprintln!("{}", msg),
                    }
                }

                "4" => {
                    let name = match Self::input("Enter task name to remove") {
                        Ok(name) => name,
                        Err(err) => {
                            eprintln!("Error getting task name: {err}");
                            return;
                        }
                    };

                    match self.tasks_manager.remove_task(&name) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => eprintln!("{}", msg),
                    }
                }

                "5" => self.tasks_manager.print_tasks(),

                "6" => {
                    let file_path = match Self::input("Enter file path to store tasks") {
                        Ok(path) => path,
                        Err(err) => {
                            eprintln!("Error getting file path: {err}");
                            return;
                        }
                    };

                    match self.tasks_manager.store_to_file(&file_path) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => eprintln!("{}", msg),
                    }
                }

                "7" => {
                    let file_path = match Self::input("Enter file path to read tasks") {
                        Ok(path) => path,
                        Err(err) => {
                            eprintln!("Error getting file path: {err}");
                            return;
                        }
                    };

                    match self.tasks_manager.read_from_file(&file_path) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => eprintln!("{}", msg),
                    }
                }

                _ => eprintln!("I don't understand this command. Please try again."),
            },
            Err(err) => eprintln!("Error getting user input: {err}"),
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
        manager.print_menu();
    loop {
        manager.process_command();
        println!()
    }
}
