use crate::repository::TaskRepository;
use crate::task::Task;
use directories::ProjectDirs;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

const APP_NAME: &str = "todocli";

pub(crate) struct JsonTaskRepository;

impl JsonTaskRepository {
    pub(crate) fn new() -> Self {
        Self
    }
}

fn task_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let proj_dirs = ProjectDirs::from("com", "didaman", APP_NAME).ok_or("无法获取项目目录")?;
    fs::create_dir_all(proj_dirs.data_dir())?;
    Ok(proj_dirs.data_dir().join("task.json"))
}

fn load_or_init_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let task_file_path = task_file_path()?;

    let contents = match fs::read_to_string(&task_file_path) {
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

impl TaskRepository for JsonTaskRepository {
    fn list(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let all_tasks = load_or_init_tasks()?;
        Ok(all_tasks)
    }

    fn add(&self, task_name: &str) -> Result<Task, Box<dyn Error>> {
        let mut all_tasks = load_or_init_tasks()?;
        let new_task = Task {
            id: all_tasks.last().map(|t| t.id + 1).unwrap_or(1),
            task_name: task_name.to_string(),
            is_done: false,
        };
        all_tasks.push(new_task.clone());
        save_task(&all_tasks)?;
        Ok(new_task)
    }

    fn remove(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let mut all_tasks = load_or_init_tasks()?;
        if let Some(index) = all_tasks.iter().position(|t| t.id == id) {
            all_tasks.remove(index);
            save_task(&all_tasks)?;
            Ok(())
        } else {
            Err("没找到 id".into())
        }
    }

    fn done(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let mut all_tasks = load_or_init_tasks()?;
        if let Some(task) = all_tasks.iter_mut().find(|t| t.id == id) {
            task.is_done = true;
            save_task(&all_tasks)?;
            Ok(())
        } else {
            Err("没找到 id".into())
        }
    }
}

fn save_task(all_tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let contents = serde_json::to_string(all_tasks)?;
    let task_file_path = task_file_path()?;
    fs::write(task_file_path, contents)?;
    Ok(())
}
