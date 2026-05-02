use std::error::Error;
use std::fs;
use std::path::Path;

use crate::Task;

pub fn load_tasks(path: &Path) -> Result<Vec<Task>, Box<dyn Error>> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Vec::new());
        }
        Err(error) => {
            return Err(
                format!("Failed to read {}: {}", path.display(), error).into(),
            );
        }
    };

    let tasks = serde_json::from_str(&contents).map_err(|error| {
        format!(
            "Failed to parse tasks from {} (invalid JSON): {}",
            path.display(),
            error
        )
    })?;

    Ok(tasks)
}

pub fn save_tasks(path: &Path, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(tasks).map_err(|error| {
        format!(
            "Failed to serialize tasks for {}: {}",
            path.display(),
            error
        )
    })?;

    fs::write(path, json).map_err(|error| {
        format!("Failed to write tasks to {}: {}", path.display(), error)
    })?;

    Ok(())
}
