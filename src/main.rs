#![allow(unused)]
use std::io;

struct TodoItem {
    id: u32,         // stores the id of the task
    task: String,    // store the task which is intended to complete
    completed: bool, // give status weather the task is completed or not
    priority: u8,    // priority (0 = low, 1 = medium, 2 = high)
}
fn main() {
    let mut todos: Vec<TodoItem> = Vec::new(); // creates a new vector

    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Mark Task as Incomplete");
        println!("5. Delete Task");
        println!("6. Edit Task");
        println!("7. Exit");
        println!("Enter your choice: ");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let choise: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        match choise {
            1 => add_task(&mut todos),
            2 => view_task(&todos),
            3 => mark_complet(&mut todos),
            4 => mark_incomplete(&mut todos),
            5 => delete_task(&mut todos),
            6 => edit_task(&mut todos),
            7 => {
                println!("Exiting");
                break;
            }
            _ => println!("Invalid choise"),
        }
    }
}

fn add_task(todos: &mut Vec<TodoItem>) {
    println!("Enter the task!");

    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read input");

    println!("Enter task priority (0 = Low, 1 = Medium, 2 = High):");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read priority");
    let priority: u8 = match input.trim().parse() {
        Ok(num) if num <= 2 => num,
        _ => {
            println!("Invalid priority. Setting to Low (0).");
            0
        }
    };

    let id: u32 = todos.len() as u32 + 1;

    let todo: TodoItem = TodoItem {
        id,
        task: task.trim().to_string(),
        completed: false,
        priority,
    };

    todos.push(todo);
    println!("Task added successfully!!");
}

fn view_task(todos: &Vec<TodoItem>) {
    if todos.is_empty() {
        println!("No tasks available!");
        return;
    }

    for todo in todos {
        let priority_label = match todo.priority {
            0 => "Low",
            1 => "Medium",
            2 => "High",
            _ => "Invalid",
        };

        let status = if todo.completed { "[✔️ ]" } else { "[❌]" };
        println!(
            "{} {} - {} (Priority: {})",
            todo.id, status, todo.task, priority_label
        );
    }
}

fn mark_complet(todos: &mut Vec<TodoItem>) {
    println!("Enter task ID to mark as complete:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        if todo.completed == true {
            println!("Task is already completed!!");
            return;
        }
        todo.completed = true;
        println!("Task marked as complete!");
    } else {
        println!("Task not found");
    }
}

fn mark_incomplete(todos: &mut Vec<TodoItem>) {
    println!("Enter task ID to mark as incomplete:");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let id: u32 = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        if todo.completed == false {
            println!("Task is already Incomplete!!");
            return;
        }
        todo.completed = false;
        println!("Task marked as Incomplete");
    } else {
        println!("Task not found");
    }
}

fn delete_task(todos: &mut Vec<TodoItem>) {
    println!("Enter task ID to delete todo:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    if let Some(pos) = todos.iter_mut().position(|todo| todo.id == id) {
        todos.remove(pos);
        println!("Task deleted successfully!");
    } else {
        println!("Task not found!");
    }
}

fn edit_task(todos: &mut Vec<TodoItem>) {
    println!("Enter the id of task which you want to edit!!");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        println!("Enter the new task:");
        let mut task: String = String::new();
        io::stdin()
            .read_line(&mut task)
            .expect("Failed to read task");
        todo.task = task.trim().to_string();

        println!("Enter the new priority (0 = Low, 1 = Medium, 2 = High):");
        let mut priority_input: String = String::new();
        io::stdin()
            .read_line(&mut priority_input)
            .expect("Failed to read input");

        let priority: u8 = match priority_input.trim().parse::<u8>() {
            Ok(num) if num <= 2 => num,
            _ => {
                println!("Invalid priority");
                todo.priority
            }
        };
        todo.priority = priority;

        println!(
            "Task edited successfully! New task: '{}', Priority: {}",
            todo.task,
            match todo.priority {
                0 => "Low",
                1 => "Medium",
                2 => "High",
                _ => "Invalid",
            }
        );
    } else {
        println!("Task with ID {} not found.", id);
    }
}
