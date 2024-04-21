# Rust
- Why
- Pros
- Cons
- Usage
- Limitation
- Company adoption / real world usage
- Deployment
- Package manager / build tools
- FAQ
    - Comparision with another language
        - vs js
        - vs python
        - vs go
        - when to choose rust
    - How it works?
    - async or sync
    - single or multithreaded


## Start rust project
#### Simple project
```
  # 1. compile code
  rustc main.rs
  # 2. run executable
  ./main
```

#### Complex project
Use package manager - cargo

Cargo:
- IF outside git repo, auto init git repo + gitignore
  - override this behavior by using cargo new --vcs=git.
- create a project using `cargo new`.
- build a project using `cargo build`.
- build for release using `cargo build --release`
- build and run a project in one step using `cargo run`.
- build a project without producing a binary to check for errors using `cargo check`.
- Build result in the:
  - default `target/debug`
  - --release `target/release`
```
$ cargo new <project_name>
$ cd <project_name>
$ cargo run (shorthand for build and run)
    cargo build # create executable
    ./target/debug/hello_cargo # run default build
$ cargo check # test error
```

Structure
```
|- <project_name>
|  |- target
|  |  |- debug
|  |     |_ <<project_name>> #executable
|  |_ src
|     |_ main.rs
|-- Cargo.toml # eq package.json
```
