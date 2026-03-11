use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ── 1. Structs, Enums & Derive macros ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// ── 2. Traits & Display impl ───────────────────────────────────────────────

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "🟢 Low"),
            Priority::Medium => write!(f, "🟡 Medium"),
            Priority::High => write!(f, "🟠 High"),
            Priority::Critical => write!(f, "🔴 Critical"),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] #{}: {} (tags: {})",
            self.priority, self.id, self.title, self.tags.join(", "))
    }
}

// ── 3. Custom trait with default implementation ────────────────────────────

trait Summarize {
    fn summary(&self) -> String;
    fn one_liner(&self) -> String {
        format!("Summary: {}", self.summary())
    }
}

impl Summarize for Task {
    fn summary(&self) -> String {
        format!("{} [{}]", self.title, self.priority)
    }
}

// ── 4. Generics & trait bounds ─────────────────────────────────────────────

fn print_all<T: fmt::Display>(items: &[T]) {
    for item in items {
        println!("  {item}");
    }
}

// ── 5. Error handling with Result & custom errors ──────────────────────────

#[derive(Debug)]
enum TaskError {
    NotFound(u32),
    DuplicateId(u32),
    EmptyTitle,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task #{id} not found"),
            TaskError::DuplicateId(id) => write!(f, "Task #{id} already exists"),
            TaskError::EmptyTitle => write!(f, "Task title cannot be empty"),
        }
    }
}

// ── 6. Struct with methods (impl block) ────────────────────────────────────

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: &str, priority: Priority, tags: Vec<&str>) -> Result<&Task, TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        if self.tasks.iter().any(|t| t.id == self.next_id) {
            return Err(TaskError::DuplicateId(self.next_id));
        }
        let task = Task {
            id: self.next_id,
            title: title.to_string(),
            priority,
            tags: tags.into_iter().map(String::from).collect(),
        };
        self.next_id += 1;
        self.tasks.push(task);
        Ok(self.tasks.last().unwrap())
    }

    // ── 7. Option type — safe null handling ────────────────────────────────
    fn find_by_id(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    fn get_by_id(&self, id: u32) -> Result<&Task, TaskError> {
        self.tasks.iter().find(|t| t.id == id).ok_or(TaskError::NotFound(id))
    }

    // ── 8. Closures & iterators ────────────────────────────────────────────
    fn filter_by_priority(&self, priority: &Priority) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| &t.priority == priority)
            .collect()
    }

    fn count_by_priority(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for task in &self.tasks {
            *counts.entry(format!("{:?}", task.priority)).or_insert(0) += 1;
        }
        counts
    }

    // ── 9. Pattern matching with if let / match ────────────────────────────
    fn describe_task(&self, id: u32) -> String {
        match self.find_by_id(id) {
            Some(task) => {
                let urgency = match task.priority {
                    Priority::Critical => "🚨 DROP EVERYTHING",
                    Priority::High => "⚡ Handle soon",
                    Priority::Medium => "📋 When you get a chance",
                    Priority::Low => "🧊 Ice box",
                };
                format!("{task}\n    → {urgency}")
            }
            None => format!("Task #{id} does not exist"),
        }
    }
}

// ── 10. Ownership & borrowing demo ─────────────────────────────────────────

fn demonstrate_ownership() {
    println!("\n═══ Ownership & Borrowing ═══");

    let original = String::from("Hello, Rust!");

    // Borrowing (immutable reference) — original is NOT moved
    let len = calculate_length(&original);
    println!("  '{original}' has {len} characters (borrowed, original still valid)");

    // Clone — explicit deep copy
    let cloned = original.clone();
    println!("  Cloned: '{cloned}'");

    // Move — ownership transfers
    let moved = original; // original is now invalid
    println!("  Moved: '{moved}'");
    // println!("{original}"); // ← This would NOT compile! Ownership moved.
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

// ── 11. Lifetimes (explicit) ───────────────────────────────────────────────

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// ── Main ───────────────────────────────────────────────────────────────────

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║   Rust Showcase — Key Features       ║");
    println!("╚══════════════════════════════════════╝\n");

    // Create task manager and add tasks
    let mut mgr = TaskManager::new();

    let tasks_to_add = vec![
        ("Set up CI/CD pipeline",    Priority::High,     vec!["devops", "infra"]),
        ("Write unit tests",         Priority::Medium,   vec!["testing", "quality"]),
        ("Fix auth bypass",          Priority::Critical, vec!["security", "urgent"]),
        ("Update README",            Priority::Low,      vec!["docs"]),
        ("Refactor database layer",  Priority::High,     vec!["backend", "tech-debt"]),
    ];

    println!("═══ Adding Tasks ═══");
    for (title, priority, tags) in tasks_to_add {
        match mgr.add(title, priority, tags) {
            Ok(task) => println!("  ✅ Added: {task}"),
            Err(e) => println!("  ❌ Error: {e}"),
        }
    }

    // Error handling demo
    println!("\n═══ Error Handling ═══");
    match mgr.add("", Priority::Low, vec![]) {
        Ok(_) => println!("  This shouldn't happen"),
        Err(e) => println!("  ❌ Expected error: {e}"),
    }
    match mgr.get_by_id(99) {
        Ok(task) => println!("  Found: {task}"),
        Err(e) => println!("  ❌ Expected error: {e}"),
    }

    // Pattern matching
    println!("\n═══ Pattern Matching ═══");
    for id in [3, 1, 99] {
        println!("  {}", mgr.describe_task(id));
    }

    // Iterators & closures
    println!("\n═══ Filtering (Closures + Iterators) ═══");
    let high_priority = mgr.filter_by_priority(&Priority::High);
    println!("  High priority tasks:");
    print_all(&high_priority.iter().map(|t| t.to_string()).collect::<Vec<_>>());

    // HashMap stats
    println!("\n═══ Priority Breakdown (HashMap) ═══");
    for (priority, count) in mgr.count_by_priority() {
        println!("  {priority}: {count} task(s)");
    }

    // Option type
    println!("\n═══ Option<T> — Safe Null Handling ═══");
    if let Some(task) = mgr.find_by_id(2) {
        println!("  Found: {}", task.one_liner());
    }
    if mgr.find_by_id(42).is_none() {
        println!("  Task #42: None (no crash, no null pointer!)");
    }

    // Ownership
    demonstrate_ownership();

    // Lifetimes
    println!("\n═══ Lifetimes ═══");
    let s1 = String::from("Rust");
    let result;
    {
        let s2 = String::from("Go");
        result = longest(&s1, &s2);
        println!("  Longest of '{s1}' and '{s2}': '{result}'");
    }

    // Serialization (serde)
    println!("\n═══ Serialization (serde) ═══");
    if let Some(task) = mgr.find_by_id(3) {
        let json = serde_json::to_string_pretty(task).unwrap();
        println!("  Task #3 as JSON:\n{json}");

        let deserialized: Task = serde_json::from_str(&json).unwrap();
        println!("  Round-tripped: {deserialized}");
    }

    println!("\n✅ All demos completed — no runtime errors, guaranteed by the compiler!");
}
