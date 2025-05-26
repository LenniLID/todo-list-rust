use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};
use std::process::exit;
use serde_json;
use std::io::prelude::*;
use std::io::BufRead;
use std::path::Path;
use crossterm::cursor::MoveTo;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Tasks {
    todo: Vec<String>,
    done: Vec<String>,
}

fn main() {
    let path = "safe-file.json";
    let mut number: usize = 0;
    let mut rm_input = String::new();
    let mut done_input = String::new();

    if !Path::new(path).exists() {
        let empty = Tasks { todo: Vec::new(), done: Vec::new() };
        let ser = serde_json::to_string_pretty(&empty).unwrap();
        std::fs::write(path, ser).expect("Konnte Datei erstellen");
    }

    let data = std::fs::read_to_string(path)
        .expect("Konnte Datei nicht lesen");

    let mut tasks: Tasks = match serde_json::from_str(&data) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("⚠️  Warnung: JSON corrupted or invalid: {}", e);
            Tasks { todo: Vec::new(), done: Vec::new() }
        }
    };

    let ser = serde_json::to_string_pretty(&tasks).unwrap();
    std::fs::write(path, ser).expect("Konnte Datei nicht schreiben");


    let mut todo = tasks.todo;
    let mut done = tasks.done;

    println!("todo: \n{}", todo.join(""));
    println!("done: \n{}", done.join(""));

    loop {
        println!("add a task:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            exit(0);
        }
        else if input.trim() == "pop" {
            todo.pop();

            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();
        }
        else if input.trim() == "pop done" {
            done.pop();

            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();
        }
        else if input.trim() == "pop todo" {
            todo.pop();

            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();
        }    
        else if input.trim() == "rm all" {
            todo.clear();
            done.clear();


            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();
        }
        else if input.trim() == "rm todo" {
            todo.clear();

            let ser = serde_json::to_string_pretty(&todo).unwrap();
            std::fs::write(path, ser).expect("Konnte Datei nicht schreiben");
        }
        else if input.trim() == "rm done" {
            done.clear();

            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();

        }
        else if input.trim() == "done" {
            println!("we got this far 1");


            number = loop {
                println!("enter the number of the task you want to move to done:");

                done_input.clear();
                io::stdin().read_line(&mut done_input).unwrap();



                match done_input.trim().parse::<usize>() {
                    Ok(num) => break num,
                    Err(_) => println!("bist du behindert du sollst eine Zahl eingeben"),
                }

                done_input.clear();
            };
            number -= 1;

            let removed_from_todo = todo.remove(number);
            done.push(removed_from_todo);

            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();

            number = 0;
        }
        else if input.trim() == "rm" {

            number = loop {
                println!("enter the number of the task you want to remove:");

                rm_input.clear();
                io::stdin().read_line(&mut rm_input).unwrap();

                match rm_input.trim().parse::<usize>() {
                    Ok(num) => break num,
                    Err(_) => println!("bist du behindert du sollst eine Zahl eingeben"),
                }
                rm_input.clear();
            };
            number -= 1;
            if number < todo.len() {
                todo.remove(number);
            }
            else { println!("this number is not fucking in the list"); }
            number = 0;
        }
        else if input.trim() == "help" {

            println!("Available commands:");
            println!("  exit       - Exit the application");
            println!("  pop        - Remove the last task from the todo list");
            println!("  pop done   - Remove the last task from the done list");
            println!("  pop todo   - Remove the last task from the todo list");
            println!("  rm all     - Clear both todo and done lists");
            println!("  rm todo    - Clear the todo list");
            println!("  rm done    - Clear the done list");
            println!("  done       - Move a task from todo to done list (you'll be asked for the task number)");
            println!("  rm         - Remove a specific task from the todo list (you'll be asked for the task number)");
            println!("  help       - Display this help message");
            println!("  [any text] - Add a new task to the todo list");

            let mut help_input = String::new();
            io::stdin().read_line(&mut help_input).unwrap();

        }
        else {
            todo.push(input.to_string());
            tasks.todo = todo.clone();
            tasks.done = done.clone();

            let ser = serde_json::to_string_pretty(&tasks).unwrap();

            std::fs::write(path, ser).unwrap();
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        println!("todo: \n{}", todo.join(""));
        println!("done: \n{}", done.join(""));

    }
}
