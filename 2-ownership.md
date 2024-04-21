# Variables and Ownership

## Variables
- [default] immutable
- re-assign?
    - make mutable
        ```rust
        let mut x = 10;
        x = 20;
        ```
    - shadowing
        ```rust
        let x = 10;
        let x = 20;
        ```
        - purpose: convert type without renaming
- data type
    - scalar
        - integers - signed (+-) & unsigned (+)
            - normal - [default] u32
            - decimal - [default] f64
            - hex
            - octal
            - binary
            - byte - u8
            - integer overflow (modulo)
        - floating point number
        - bool
        - char 'A'
        - &str
            - string literal 
            - @ binary (stack)
        - String
            - @ heap
    - compound - group of values
        - tuple
            - fixed size arr
            - different type OK
                ```rust
                let tup = (val1, val2, val3)
                let (val1, val2, val3) = tup
                ```
        - array
        
- snake_case
#### Control
1. If else if else
2. Ternary if else
#### Loop
1. Loop {}
    - return value
    - label
    ```rust
        let value = loop {}

        'outer_loop': loop {
            'inner_loop': loop {
                if condition1 {
                    break;
                }

                if condition2 {
                    break 'outer_loop';
                }
            }
        }
    ```
2. While ... {}
3. Inside collection
    - For ... in [] {}
    - For ... in 0..4 {}

## Ownership Origin
- set of rules on `how Rust manages memory`
- fear not, it `wont slow down` ur app
### Stack and heap memory
- stack
    - LIFO
    - faster, no pointer pointer club
    - stack only memory, copied, reason:
        - stored entirely on the stack
        - quick to copy
        - deep & shallow copy no speed diff
        - supported types:
            - integer, float, bool, char
            - tuples, with above type
                - (i32, i32) v
                - (i32, String) x
- heap
    - less organized
    - steps:
        - [ur code] request certain amount of space
        - [malloc] search for empty spot @heat
            - marks it as in use
            - returns a pointer (address of that loc)
            - book keeping to prepare for next alloc
        - [rust] out of scope? drop allocation

#### Sample
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 inallocate
```
[image] move
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```
[image] clone

#### 1. Ownership rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

No shallow copy (pointing to same content on heap) to prevent race condition - memory safety

#### 2. [ALT][Preferable] Reference rules
- Moving ownership is tedious
- reference `&var`
- pointing to the pointer to ... original pointer
- <u>type</u>:
    - [default] immutable ref
        ```rust
        fn main() {
            let s1 = String::from("hello");

            let len = calculate_length(&s1);

            println!("The length of '{}' is {}.", s1, len);
        }

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
        ```
    - mutable ref
        ```rust
        fn main() {
            let mut s = String::from("hello");
            change(&mut s);
        }

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
        ```
        - restrictions:
            - if you have a mutable reference to a value
                - NO other references to that value
                - avoid data race (race condition when these beha occurs):
                    - min 2 pointers access the same data ATM
                    - 1 of the pointers write data ATM
                    - No mechanism to synch access to the data
                - Rust dun wan compile
- <u>Rules</u>:
    1. At any given time, u can have either __ or __
        - one mutable ref
        - any number of immutable refs
    2. References must be static
- Dangling pointer
    - a pointer that references a location in memory that may have been given to someone else
    - X happened on rust - error
        ```rust
        fn main() {
            let reference_to_nothing = dangle();
        }

        fn dangle() -> &String {
            let s = String::from("hello");

            &s
        }
        ```
- Slice
    - Cont. seq of els in a collection rather than the whole collection
    - Reference, X ownership
