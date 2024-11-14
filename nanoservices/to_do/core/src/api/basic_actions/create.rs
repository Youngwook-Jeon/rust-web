use dal::json_file::save_one;
use glue::errors::NanoServiceError;

use crate::enums::TaskStatus;
use crate::structs::ToDoItem;

pub fn create(title: &str, status: TaskStatus) -> Result<ToDoItem, NanoServiceError> {
    let item = ToDoItem {
        title: title.to_string(),
        status,
    };

    let _ = save_one(&title.to_string(), &item)?;
    Ok(item)
}