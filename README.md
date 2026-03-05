# 🦀 Rust Showcase — Key Features Walkthrough

A task manager that demonstrates 11 core Rust features in ~170 lines.

## What the Program Does

Creates a simple task manager that adds tasks with priorities, filters them, serializes to JSON, and demonstrates Rust's memory safety guarantees — all without a single runtime error possible.

## Key Rust Features Demonstrated

### 1. Structs & Enums
Rust has no classes. Instead you use **structs** (data) and **enums** (variants). Our `Task` struct holds an id, title, priority, and tags. The `Priority` enum has four variants: Low, Medium, High, Critical. Enums in Rust are far more powerful than C/Java enums — each variant can carry data.

### 2. Derive Macros
`#[derive(Debug, Clone, Serialize, Deserialize)]` auto-generates trait implementations at compile time. One line gives you debug printing, cloning, and JSON serialization. Zero boilerplate.

### 3. Traits (like Interfaces, but better)
We implement `Display` for pretty-printing and define a custom `Summarize` trait with a default method. Traits can have default implementations that types can override — more flexible than traditional interfaces.

### 4. Generics & Trait Bounds
`fn print_all<T: fmt::Display>(items: &[T])` — works on any slice of displayable items. The compiler generates specialized code for each concrete type (zero-cost abstraction, no vtable overhead).

### 5. Error Handling (Result\<T, E\>)
No exceptions in Rust. Functions that can fail return `Result<Ok, Err>`. The compiler **forces** you to handle errors — you literally cannot ignore them. Our `TaskError` enum has specific variants for each failure mode.

### 6. Option\<T\> — No Null Pointers
Rust has no null. Instead, `Option<T>` is either `Some(value)` or `None`. The compiler forces you to check before accessing. This eliminates null pointer exceptions at compile time.

### 7. Pattern Matching
`match` is like switch-case on steroids. The compiler ensures you handle every possible case (exhaustive matching). Combined with enums, it makes invalid states unrepresentable.

### 8. Closures & Iterators
`.iter().filter(|t| &t.priority == priority).collect()` — functional-style chains with zero-cost abstractions. The compiler optimizes these into the same machine code as hand-written loops.

### 9. Ownership & Borrowing (The Big One)
Rust's killer feature. Every value has exactly one owner. You can **borrow** (`&reference`) without taking ownership, or **move** ownership. The compiler enforces this at compile time — no garbage collector needed, no use-after-free, no data races. The demo shows all three: borrowing, cloning, and moving.

### 10. Lifetimes
`fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` — lifetime annotations tell the compiler how long references are valid. This prevents dangling references at compile time. Most of the time Rust infers lifetimes automatically; explicit annotations are only needed in ambiguous cases.

### 11. Serde (Serialization)
The `serde` crate is Rust's serialization framework. Add `#[derive(Serialize, Deserialize)]` and your struct can convert to/from JSON, YAML, TOML, MessagePack, etc. The demo round-trips a task through JSON.

## Why Rust Matters (for someone coming from C/C++)

- **Memory safety without GC** — same performance as C, but the compiler catches use-after-free, buffer overflows, and data races
- **Zero-cost abstractions** — iterators, generics, and traits compile to the same code you'd write by hand
- **Cargo** — built-in package manager and build system (no CMake/Make headaches)
- **Great for embedded & systems** — `no_std` mode works on bare metal, compiles to ARM/RISC-V
- **AI-friendly** — the strict compiler gives AI coding agents precise error messages to self-correct

## Running It

```bash
cargo run
```

## Sample Output

```
╔══════════════════════════════════════╗
║   Rust Showcase — Key Features       ║
╚══════════════════════════════════════╝

═══ Adding Tasks ═══
  ✅ Added: [🟠 High] #1: Set up CI/CD pipeline (tags: devops, infra)
  ✅ Added: [🟡 Medium] #2: Write unit tests (tags: testing, quality)
  ✅ Added: [🔴 Critical] #3: Fix auth bypass (tags: security, urgent)
  ✅ Added: [🟢 Low] #4: Update README (tags: docs)
  ✅ Added: [🟠 High] #5: Refactor database layer (tags: backend, tech-debt)

═══ Serialization (serde) ═══
  Task #3 as JSON:
  {
    "id": 3,
    "title": "Fix auth bypass",
    "priority": "Critical",
    "tags": ["security", "urgent"]
  }

✅ All demos completed — no runtime errors, guaranteed by the compiler!
```

## License

MIT
