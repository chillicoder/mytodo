use std::env;
use mytodo::db::{mark_task, create_task, query_task, establish_connection};

fn help() {
    println!("subcommands:");
    println!("\tnew <title>: create a new task");
    println!("\tshow: show list of tasks");
    println!("\tdone <task id>: mark task as done");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "done" => done_task(&args[2..]),
        _ => help(),
    }
}

fn done_task(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing <task id>");
        help();
        return;
    }

    let conn = establish_connection();
    let task_id = args[0].parse::<i32>().unwrap();
    let task = mark_task(&conn, task_id, false);

    if task.done {
        println!("Task #{} is done! Yay!", task.id);
    } else {
        println!("Task #{} is not done", task.id);
    }
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n--------");
    for tasks in query_task(&conn) {
        if tasks.done {
            println!("{}) [X]\t {}", tasks.id, tasks.title)
        } else {
            println!("{}) [ ]\t {}", tasks.id, tasks.title)
        }
    }
}

