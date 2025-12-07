//todo_app

use rocket::*;
mod task;
use task::*;
use rocket::http::Status;
use rocket::serde::json::Json;


#[get("/tasks")]
fn fetch_tasks()->Json<Vec<Task>> {
    let tasks=load_tasks();
    Json(tasks)
}

#[post("/create-tasks",format="json", data="<task>")]
fn create_tasks(task:Json<Task>)->Status {
    let mut tasks=load_tasks();
    if let Some(_index)=tasks
        .iter()
        .position(|item| item.task_name==task.0.task_name)
    {
        return Status::Conflict;
    }
    tasks.push(task.0);

    save_tasks(&tasks);
    Status::Created
}

#[put("/update-tasks",format="json", data="<task>")]
fn update_tasks(task:Json<Task>)->Status {
    let mut tasks=load_tasks();
    if let Some(index)=tasks
        .iter()
        .position(|item| item.task_name==task.0.task_name)
    {
        tasks.remove(index);
        tasks.insert(index,task.0);
        save_tasks(&tasks);
        return Status::NoContent;

    }
    else{
        return Status::NotFound;
    }
    tasks.push(task.0);

    save_tasks(&tasks);
    Status::Created
}
#[delete("/delete-tasks",format="json", data="<task>")]
fn delete_tasks(task:Json<Task>)->Status {
    let mut tasks=load_tasks();
    if let Some(index)=tasks
        .iter()
        .position(|item| item.task_name==task.0.task_name)
    {
        tasks.remove(index);
        save_tasks(&tasks);
        return Status::NoContent;

    }
    else{
        return Status::NotFound;
    }
    tasks.push(task.0);

    save_tasks(&tasks);
    Status::Created
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_tasks,create_tasks,update_tasks,delete_tasks])
}