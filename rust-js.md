## Rust for JS/TS dev

### What is Rust?
Rust is a systems programming language known for its focus on safety and performance.
It's designed to provide memory safety without using garbage collection, making it an excellent choice for high-performance applications.
Rust achieves this through its unique ownership model, which manages memory through a set of rules that the compiler checks at compile time.

### Why Should JavaScript Developers Learn Rust?

1. **Performance:** Rust offers control over low-level system details, enabling highly performant and efficient software development. Significantly faster, as it compiles directly to machine code.
2. **Memory Safety:** 
  - Rust’s ownership model ensures memory safety without the overhead of a garbage collector.
  - Performance & resource critical
3. **Concurrency:**
  - Rust provides powerful yet safe abstractions for dealing with concurrent programming.
  - Helps you write code that is free from data races and other common concurrency problems, which can be challenging in JS

4. **Tooling:** Rust comes with Cargo, its package manager and build system -> similar w/ yarn/npm + webpack babel

5. **Growing Ecosystem:** While Rust is often thought of as a language for systems programming, its ecosystem for web development and other areas is rapidly growing, with frameworks like Actix and Rocket for web services, and WASM support for running Rust on the web.

6. **Interoperability:** For TypeScript developers, learning Rust can be beneficial for creating high-performance, reusable libraries that can be integrated into Node.js applications through native modules or WebAssembly.

### Transitioning from JavaScript to Rust
- **Syntax and Semantics:** Similar + 
new concepts like match expressions, enums, and traits that don’t have direct analogs in JavaScript.
- **Type System:** 
- **Error Handling:** Unlike JavaScript's try-catch error handling, Rust uses a combination of the `Result` and `Option` types for error handling, which can lead to safer, more predictable code.


<details>
  <summary>When to choose rust?  </summary>

  1. **High Performance Needs:**
      - requires high performance and efficiency.
      - perfect for system-level programming, high-load servers, & performance-critical applications
        - due to its ability to run very close to the hardware.
  2. **System Programming:** 
    - dev: operating systems, file systems, game engines, and other systems
      - direct control over hardware resources is necessary.
  3. **Concurrency:** 
    - more robust and safer tools for concurrent programming
      - ownership and borrowing concepts
        - prevent race conditions (data race) without needing a garbage collector
        - suitable for multi-threaded applications.
  4. **Memory Safety:**
    - Guaranteed memory safety (without a garbage collector) benefit from Rust
    - Includes embedded systems, network servers, and other systems where you want to avoid the overhead of runtime checks.
      - case: discord notification spikes on go -> migrate to rust
  5. **Reliability and Stability:**
    - Rust’s strong type system and compile-time checks ensure that many common bugs are caught before runtime, leading to more reliable and stable applications
      - e.g: null pointer dereferencing and buffer overflows 
  6. **Interacting with Low-Level System Components:**
    - For applications that need to interact with hardware or need precise control over system operations, Rust's low-level capabilities make it a strong candidate.
</details>

<details>
  <summary>When to choose JS over Rust?</summary>
  
  1. Web dev
      - native support
      - vast ecosystem
  2. Quick prototyping, Fullstack app, Cross platform (desktop mobile web)
  3. Even-driven application
      - JS ED model suitable for apps that has lots of async opr (SPA & real-time)
      - with minimum computational
      - more mature lib support
      - IF IO w/ safe concurrent operation use Rust
          - handle concurrency wo data race 
      - IF IO w/ heavy computation use Rust
        - Rust near C-level perf __ ideal for both IO & CPU intensive
          - mng memory & system resources effectively

</details>


### Concept
<details>
  <summary>Similar concept</summary>
  
  1. Function & Closure
    - Rust's closure & fn pointer 
    - JS's fn & arrow function
  2. Module & imports (reusability)
    - Rust: mod & use
    - JS: import & export
  3. Control structure
    - Both: if else, loops (for,while)
      - Rust: match
      - JS: switch
      - Rust: if let
      - JS: ternary assignment
  4. Error handling
    - Rust: Result & Options (for Error & Nullable types)
  5. Aysnc programming
    - Rust: async await with futures
    - JS: async/await for promises
</details>

<details>
  <summary>Rust only</summary>
  
  1. Ownership & Borrowing & Lifetime
     Ensure compole-time memory management wo gc
  2. Memory safety wo GC
  3. Pattern matching
     Can be used on:
     - Variable binding
     - Control structure
     - Crazily cool destructuring
  5. Traits & associated types
     - Traits:
       - JS: interface
       - Rust: + associated types (MOAR powerful)   
  6. ADTs (Algebraic Data Types)
    Rust uses enums (and match statements) extensively to create types that can hold different kinds of data, known as sum types, which is more integrated than TypeScript’s union types.
</details>

<details>
  <summary>JS only</Summary>
  
  1. Prototypal Inheritance
  2. Event Loop and Non-blocking I/O: 
       - Designed for event-driven architecture
           - utilizes a single-threaded event loop
           - making it ideal for I/O-heavy operation
  3. Dynamic Typing
  4. First-Class and Higher-Order Functions
      - Function as first-class citizens:
        - can be stored in variables
        - passed as arguments
        - used as return values from other functions
        - Very flexible approach compared to Rust
  5. Global Object
      - Global object accessible in all scopes
        - Core part of its design
</details>

<details>
  <summary>Are Rust programmer missing out?</summary>

  1. Prototyping inheritance:
      - Rust: composition > inheritance. similar behaviour = traits & trait objects
      - Usefulness in Rust: UH UH
        - clash with strong type & memory safety guarantee
        - compile-time checks wont benefit from prototypal model
  2. Event loop & non-blocking I/O:
  3. Global object
       - Rust: not aligned w/ safety guarantees w/ concurrent programming
       - Rust prefer: passing params explicitly to func & mng state in controlled manner
          - Alternative (not recommended):
              -  `static` var
              -  `lazy_static` crate
              -  NOT RECOMMENDED - potential issue w/ mutability & thread safety
  4. First class & Higher-order function
      - quite similar
  TL'DR:
  ```
    Rust’s design choices make it ideal for scenarios where control and reliability are paramount, even if that means forgoing some of the flexibility found in JavaScript/TypeScript.
  ```
</details>

