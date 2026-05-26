use core::fmt;
use std::fs;
use std::error::Error;
use std::io::ErrorKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task{
    id: u32,
    task_name: String,
    is_done: bool,
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}",self.id, self.task_name)
    }
}


fn load_or_init_tasks() -> Result<Vec<Task>, Box<dyn Error>>{
    let task_file_path = "./task.json";
    let contents = match fs::read_to_string(task_file_path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            // 文件不存在时先创建空文件，并把内容视为 ""
            fs::File::create(task_file_path)?;
            String::new()
        }
        Err(err) => return Err(err.into()),
    };
    
    let all_tasks: Vec<Task> = if contents.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents)?
    };
    Ok(all_tasks)
}

fn save_task(all_tasks: &Vec<Task>) -> Result<(), Box<dyn Error>>{
    let contents = serde_json::to_string(all_tasks)?;
    fs::write( "./task.json", contents)?;
    Ok(())
}

pub fn add_task(task_name: &str) -> Result<(), Box<dyn Error>> {
    let mut all_tasks = load_or_init_tasks()?;
    let new_task = Task{
        id: all_tasks.last().map(|t| t.id +1).unwrap_or(1),
        task_name: task_name.to_string(),
        is_done: false,
    };
    all_tasks.push(new_task);
    save_task(&all_tasks)?;
    Ok(())
}

pub fn show_task() -> Result<(), Box<dyn Error>>{
    let all_tasks = load_or_init_tasks()?;
    for task in all_tasks.iter(){
        println!("{}",task);
    }
    Ok(())
}

pub fn done_task(id: u32) -> Result<(), Box<dyn Error>>{
    let mut all_tasks = load_or_init_tasks()?;
    let mut find_id = false;
    for task in all_tasks.iter_mut(){
        if task.id == id{
            task.is_done = true;
            find_id = true;
        }
    }
    if !find_id{
        return Err("没找到 id".into());
    }
    save_task(&all_tasks)?;
    Ok(())
}

pub fn remove_task(id: u32) -> Result<(), Box<dyn Error>>{
    let mut all_tasks = load_or_init_tasks()?;
    if let Some(index) = all_tasks.iter().position(|t| t.id == id){
        all_tasks.remove(index);
        save_task(&all_tasks)?;
        Ok(())
    }else{
        Err("没找到 id".into())
    }
}