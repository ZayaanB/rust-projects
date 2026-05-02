pub mod storage;

use std::error::Error;
use std::path::{Path, PathBuf};

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::storage::{load_tasks, save_tasks};

pub const DEFAULT_TASKS_FILE: &str = "tasks.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add { text: String },
    List,
    Done { id: u64 },
}

pub struct Config {
    pub tasks_path: PathBuf,
    pub command: Command,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let subcommand = match args.next() {
            Some(arg) => arg,
            None => return Err("missing command"),
        };

        match subcommand.as_str() {
            "add" => {
                let rest: Vec<String> = args.collect();
                if rest.is_empty() {
                    return Err("the `add` command needs text after it");
                }

                Ok(Config {
                    tasks_path: PathBuf::from(DEFAULT_TASKS_FILE),
                    command: Command::Add {
                        text: rest.join(" "),
                    },
                })
            }
            "list" => match args.next() {
                None => Ok(Config {
                    tasks_path: PathBuf::from(DEFAULT_TASKS_FILE),
                    command: Command::List,
                }),
                Some(_) => Err("the `list` command takes no arguments"),
            },
            "done" => {
                let id_string = match args.next() {
                    Some(arg) => arg,
                    None => return Err("the `done` command needs exactly one task id"),
                };

                if args.next().is_some() {
                    return Err("the `done` command needs exactly one task id");
                }

                let id = match id_string.parse() {
                    Ok(value) => value,
                    Err(_) => return Err("invalid task id"),
                };

                Ok(Config {
                    tasks_path: PathBuf::from(DEFAULT_TASKS_FILE),
                    command: Command::Done { id },
                })
            }
            _ => Err("unknown command"),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = config.tasks_path.as_path();

    match config.command {
        Command::Add { text } => add_task(path, text),
        Command::List => list_tasks(path),
        Command::Done { id } => complete_task(path, id),
    }
}

fn add_task(path: &Path, text: String) -> Result<(), Box<dyn Error>> {
    let mut tasks = load_tasks(path)?;

    let next_id = match tasks.iter().map(|task| task.id).max() {
        Some(max_id) => max_id.saturating_add(1),
        None => 1,
    };

    tasks.push(Task {
        id: next_id,
        text,
        completed: false,
    });

    save_tasks(path, &tasks)?;
    Ok(())
}

fn list_tasks(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut tasks = load_tasks(path)?;
    tasks.sort_by_key(|task| task.id);

    for task in &tasks {
        let line = format_line(task);
        if task.completed {
            println!("{}", line.green());
        } else {
            println!("{}", line.cyan());
        }
    }

    Ok(())
}

fn complete_task(path: &Path, id: u64) -> Result<(), Box<dyn Error>> {
    let mut tasks = load_tasks(path)?;

    let mut found = false;
    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = true;
            found = true;
            break;
        }
    }

    if found {
        save_tasks(path, &tasks)?;
        Ok(())
    } else {
        Err(format!("No task with id {id} found in {}", path.display()).into())
    }
}

fn format_line(task: &Task) -> String {
    let marker = if task.completed { "[x]" } else { "[ ]" };
    format!("{marker} {} {}", task.id, task.text)
}
