#![allow(unused)]
use std::io;

struct TodoItem {
    id: u32, // stores the id of the task
    task: String, // store the task which is intended to complete
    complated: bool // give status weather the task is completed or not
}
fn main() {
    let mut todos: Vec<TodoItem> = Vec::new(); // creates a new vector

    loop {
        println!("\n--- To-Do List ---");
        println!("\n--- To-Do List ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Exit");
        println!("Enter your choice: ");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

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
            4 => delete_task(&mut todos),
            5 => edit_task(&mut todos),
            6 => {
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
    io::stdin().read_line(&mut task).expect("Failed to read input");

    let id: u32 = todos.len() as u32 + 1;

    let todo: TodoItem = TodoItem {
        id,
        task: task.trim().to_string(),
        complated: false
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
        let status = if todo.complated {"[✓]"} else {"[X]"};
        println!("{} {} - {}", todo.id, status, todo.task);
    }
}
fn mark_complet(todos: &mut Vec<TodoItem>) {
    println!("Enter task ID to mark as complete:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        todo.complated = true;
        println!("Task marked as complete!");
    } else {
        println!("Task not found");
    }
}

fn delete_task(todos: &mut Vec<TodoItem>) {
    println!("Enter task ID to delete todo:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

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
    io::stdin().read_line(&mut input).expect("Failed to read input");

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
        io::stdin().read_line(&mut task).expect("Failed to read task");
        todo.task = task.trim().to_string();
    } else {
        println!("Task with ID {} not found.", id);
    }

    println!("Task edited");
}