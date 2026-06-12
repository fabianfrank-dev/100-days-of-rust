use std::fmt;

#[derive(Debug, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Status::Todo => "[ ]",
            Status::InProgress => "[~]",
            Status::Done => "[x]",
        };
        write!(f, "{label}")
    }
}

#[derive(Debug)]
struct Task {
    id: usize,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: usize, title: &str) -> Self {
        Task {
            id,
            title: title.to_string(),
            status: Status::Todo,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} {} {}", self.id, self.status, self.title)
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { tasks: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, title: &str) -> usize {
        let id = self.next_id;
        self.tasks.push(Task::new(id, title));
        self.next_id += 1;
        id
    }

    fn set_status(&mut self, id: usize, status: Status) -> bool {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => { task.status = status; true }
            None => false,
        }
    }

    fn pending(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.status != Status::Done).collect()
    }

    fn print_all(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet.");
            return;
        }
        for task in &self.tasks {
            println!("  {task}");
        }
    }
}

fn main() {
    let mut mgr = TaskManager::new();

    let id1 = mgr.add("Write README");
    let id2 = mgr.add("Add unit tests");
    let id3 = mgr.add("Publish crate");

    mgr.set_status(id1, Status::Done);
    mgr.set_status(id2, Status::InProgress);

    println!("=== All tasks ===");
    mgr.print_all();

    println!("\n=== Still pending ({}) ===", mgr.pending().len());
    for task in mgr.pending() {
        println!("  {task}");
    }

    // Try updating a non-existent task
    let found = mgr.set_status(99, Status::Done);
    println!("\nUpdate task #99: {}", if found { "ok" } else { "not found" });

    // Collect just the titles of done tasks using iterators
    let done_titles: Vec<&str> = mgr.tasks.iter()
        .filter(|t| t.status == Status::Done)
        .map(|t| t.title.as_str())
        .collect();
    println!("\nCompleted: {:?}", done_titles);
}
