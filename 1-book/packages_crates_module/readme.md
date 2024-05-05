### Managing Growing Projects with Packages, Crates, and Modules

Several constructs to help structure code:

- **Packages:** A Cargo feature that lets you build, test, and share crates.
    - cargo run: create a package
- **Crates:** A tree of modules that produces a library or executable.
- **Modules and `use`:** Helps to control the organization, scope, and privacy of paths.
- **Paths:** A way of naming an item, such as a struct, function, or module.

Key Points:
- Packages can contain multiple binary crates and optionally one library crate.
- `src/main.rs` is the crate root of a binary crate with the same name as the package.
- `src/lib.rs` is the crate root of a library crate that doesn’t have the same name as the package.
- Use `mod` to declare modules and `use` to bring paths into scope.
- Privacy is crucial; by default, everything in Rust is private and can be exposed with `pub`.

### Notes:

- **Packages and Crates:**
  - A package has a `Cargo.toml` that describes how to build one or more crates.
  - A crate can be a binary or a library (or both).
- **Create a new lib**
    ```
      // on cli
      cargo new --lib restaurant
    ```
- **Defining Modules:**
  - Use `mod` keyword to define a module.
  - Example: `mod front_of_house { ... }`
- **Using Modules:**
  - Bring a module into scope with `use`.
  - Example: `use crate::front_of_house::hosting;`
- **Making Items Public:**
  - Use `pub` to make any item (function, method, module, struct, enum, etc.) public.
  - Example: `pub mod front_of_house;`
- **Structuring Access:**
  - `pub(crate)` makes an item public within the crate.
  - Access parent or sibling modules using `super` or by using their name directly.
- **Paths for Referring to an Item:**
  - Absolute path starts from a crate root by using a crate name or a literal `crate`.
  - Relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.
- **Re-exporting Items:**
  - Use `pub use` to re-export items, allowing external code to use the item as if it's defined in the outer scope.
  - Example: `pub use crate::front_of_house::hosting;`

### Comparison Code with JavaScript

**Rust Code:**
```rust
// lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// main.rs
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

**JavaScript Equivalent:**

```javascript
// frontOfHouse.js
export const hosting = {
    addToWaitlist: function() {}
};

// main.js
import { hosting } from './frontOfHouse.js';

function main() {
    hosting.addToWaitlist();
}
```

**Key Differences:**
- Rust enforces privacy strictly; everything is private by default.
- JavaScript’s modules do not have privacy in the same sense; all exports are public.
- Rust's modules must be explicitly made public and their use declared; JavaScript imports and exports are more straightforward.

This comparison highlights Rust’s focus on safety and encapsulation, contrasting with JavaScript’s more open and flexible module system. These features reflect the different priorities and typical use cases of each language.