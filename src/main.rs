use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use prettytable::{Table, Row, Cell};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

// This struct remains the same
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    due_date: String,
    priority: String,
    completed: bool,
    id: u32,
}

impl Task {
    fn new(description: String, due_date: String, priority: String, completed: bool, id: u32) -> Self {
        Task {
            description,
            due_date,
            priority,
            completed,
            id,
        }
    }
}

fn sort_tasks_by_due_date(tasks: &HashMap<String, Task>) -> HashMap<String, Task> {
    let mut task_vec: Vec<(&String, &Task)> = tasks.iter().collect();
    task_vec.sort_by(|(_, a), (_, b)| a.due_date.cmp(&b.due_date));

    task_vec
        .into_iter()
        .map(|(id, task)| (id.clone(), task.clone()))
        .collect()
}

fn filter_tasks_by_status(tasks: &HashMap<String, Task>, completed: bool) -> HashMap<String, Task> {
    tasks
        .iter()
        .filter(|&(_, task)| task.completed == completed)
        .map(|(id, task)| (id.clone(), task.clone()))
        .collect()
}


fn main() {
    let mut tasks: HashMap<String, Task> = load_tasks();
    let mut next_id: u32 = tasks.keys().map(|k| k.parse::<u32>().unwrap_or(0)).max().unwrap_or(0) + 1;

    loop {
        println!("{}", "Task List CLI".blue().bold());

        let options = vec![
            "Add a task ➕".green().bold(),
            "List tasks 📋".cyan().bold(),
            "Mark a task as done ✅".magenta().bold(),
            "Sort tasks by due date 🔄".cyan().bold(),
            "Filter tasks by completion 🔍".magenta().bold(),
            "Delete a task 🗑️".red().bold(),
            "Exit 🚪".yellow().bold(),
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                add_task(&mut tasks, &mut next_id);
            }
            1 => {
                list_tasks(&tasks);
            }
            2 => {
                mark_task_as_done(&mut tasks);
            }
            3 => {
                delete_task(&mut tasks);
            }
            4 => {
                tasks = sort_tasks_by_due_date(&tasks);
                list_tasks(&tasks);
            }
            5 => {
                let completed = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a status to filter by")
                    .default(0)
                    .items(&["Completed", "Not Completed"])
                    .interact()
                    .unwrap();

                tasks = filter_tasks_by_status(&tasks, completed == 0);
                list_tasks(&tasks);
            }
            6 => {
                save_tasks(&tasks);
                break; // Exit the loop when "Exit" is selected
            }
            _ => {}
        }

        // Pause to let the user see the menu before proceeding
        println!("Press Enter to continue...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn add_task(tasks: &mut HashMap<String, Task>, next_id: &mut u32) {
    println!("{}", "Add a Task".green().bold());
    let description = Input::<String>::new()
        .with_prompt("Enter task description")
        .interact()
        .unwrap();

    let due_date = Input::<String>::new()
        .with_prompt("Enter due date")
        .interact()
        .unwrap();

    let priority = Input::<String>::new()
        .with_prompt("Enter priority of the task (e.g., high or low)")
        .interact()
        .unwrap();

    let task = Task::new(description, due_date, priority, false, *next_id);
    tasks.insert((*next_id).to_string(), task);
    *next_id += 1; // Increment the task ID counter

    println!("Task added!");
}

fn list_tasks(tasks: &HashMap<String, Task>) {
    if tasks.is_empty() {
        println!("No tasks to display.");
    } else {
        println!("{}", "Task List".cyan().bold());
        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("Task ID").style_spec("bF"),
            Cell::new("Description").style_spec("bF"),
            Cell::new("Due Date").style_spec("bF"),
            Cell::new("Priority").style_spec("bF"),
            Cell::new("Status").style_spec("bF"),
        ]));

        for (task_id, task) in tasks.iter() {
            let status = if task.completed { "Completed".green() } else { "Not Completed".red() };

            table.add_row(Row::new(vec![
                Cell::new(task_id),
                Cell::new(&task.description),
                Cell::new(&task.due_date),
                Cell::new(&task.priority),
                Cell::new(&*status),
            ]));
        }

        table.printstd();
    }
}

fn mark_task_as_done(tasks: &mut HashMap<String, Task>) {
    if tasks.is_empty() {
        println!("No tasks to mark as done.");
    } else {
        println!("{}", "Mark a Task as Done".magenta().bold());
        list_tasks(&tasks);

        let task_id = Input::<String>::new()
            .with_prompt("Enter the task ID to mark as done")
            .interact()
            .unwrap();

        if let Some(task) = tasks.get_mut(&task_id) {
            task.completed = true;
            println!("Task marked as done.");
        } else {
            println!("Task not found with the provided ID.");
        }
    }
}

fn delete_task(tasks: &mut HashMap<String, Task>) {
    if tasks.is_empty() {
        println!("No tasks to delete.");
    } else {
        println!("{}", "Delete a Task".red().bold());
        list_tasks(&tasks);

        let task_id = Input::<String>::new()
            .with_prompt("Enter the task ID to delete")
            .interact()
            .unwrap();

        if tasks.remove(&task_id).is_some() {
            println!("Task deleted.");
        } else {
            println!("Task not found with the provided ID.");
        }
    }
}

fn load_tasks() -> HashMap<String, Task> {
    let file = File::open("tasks.json");
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            if f.read_to_string(&mut contents).is_ok() {
                match serde_json::from_str(&contents) {
                    Ok(tasks) => return tasks,
                    Err(_) => {}
                }
            }
        }
        _ => {}
    }
    HashMap::new()
}

fn save_tasks(tasks: &HashMap<String, Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .expect("Error opening file");

    let serialized = serde_json::to_string_pretty(&tasks).expect("Serialization failed");

    if file.write_all(serialized.as_bytes()).is_err() {
        println!("Write failed.");
    }
}