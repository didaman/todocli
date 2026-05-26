mod repository;
mod task;

use crate::repository::{JsonTaskRepository, TaskRepository};
use std::error::Error;

pub enum Command {
    Add(String),
    List,
    Done(u32),
    Remove(u32),
}

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("用法: todo <list|add|done|remove> ...");
        }

        let sub = args[1].as_str();
        match sub {
            "list" => {
                if args.len() == 2 {
                    Ok(Config {
                        command: Command::List,
                    })
                } else {
                    Err("用法: todo list")
                }
            }
            "add" => {
                if args.len() == 3 {
                    let task_name = args[2].clone();
                    Ok(Config {
                        command: Command::Add(task_name),
                    })
                } else {
                    Err("用法: todo add 'buy milk' ")
                }
            }
            "done" => {
                if args.len() == 3 {
                    let id = args[2].parse::<u32>().map_err(|_| "id 必须是正整数")?;
                    Ok(Config {
                        command: Command::Done(id),
                    })
                } else {
                    Err("用法: todo done 2")
                }
            }
            "remove" => {
                if args.len() == 3 {
                    let id = args[2].parse::<u32>().map_err(|_| "id 必须是正整数")?;
                    Ok(Config {
                        command: Command::Remove(id),
                    })
                } else {
                    Err("用法: todo remove 2")
                }
            }
            _ => Err("用法: todo <list|add|done|remove> ..."),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let repository = JsonTaskRepository::new();

    match config.command {
        Command::List => {
            let tasks = repository.list()?;
            for task in tasks {
                println!("{task}");
            }
            Ok(())
        }
        Command::Add(name) => {
            repository.add(&name)?;
            Ok(())
        }
        Command::Done(id) => repository.done(id),
        Command::Remove(id) => repository.remove(id),
    }
}
