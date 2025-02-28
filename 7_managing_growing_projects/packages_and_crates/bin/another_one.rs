//each filein this folder will represent another binary crate

In Rust, **packages, crates, and modules** help organize code efficiently. Here's how they relate to each other:

### 1. **Crate**
A **crate** is the smallest unit of code compilation in Rust. It's equivalent to a **library** or an **executable**.

- **Binary Crate**: Produces an executable (e.g., a program with a `main` function).
- **Library Crate**: Produces a reusable library (e.g., without a `main` function).

A crate consists of Rust source files and dependencies specified in `Cargo.toml`.

---

### 2. **Package**
A **package** is a collection of one or more crates. It is managed by **Cargo**, Rust's package manager.

- A package typically contains:
  - **One library crate (`src/lib.rs`)** (optional)
  - **One or more binary crates (`src/main.rs`, `src/bin/*.rs`)** (optional)
  - A `Cargo.toml` file defining dependencies and metadata.

> 📝 A package **cannot** contain multiple library crates but can have multiple binary crates.

---

### 3. **Module**
A **module** is a way to organize and structure code **within a crate**. It allows breaking a large program into smaller, manageable parts.

- Defined using the `mod` keyword:
  ```rust
  mod math {
      pub fn add(a: i32, b: i32) -> i32 {
          a + b
      }
  }

  fn main() {
      println!("{}", math::add(2, 3)); // Output: 5
  }
  ```
- Modules can be nested, private, or public (`pub` keyword).
- They can be placed in separate files and linked using `mod`:
  ```rust
  mod math;  // Loads `math.rs` or `math/mod.rs`
  ```

---

### **Summary**
| Concept  | Description |
|----------|------------|
| **Crate** | A single compilation unit, either a binary or library. |
| **Package** | A collection of crates managed by Cargo. |
| **Module** | A way to organize code **inside** a crate. |

