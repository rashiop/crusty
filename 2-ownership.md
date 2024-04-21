# Variables
- default immutable
- re-assign?
    - make mutable
        ```
            let mut x = 10;
            x = 20;
        ```
    - shadowing
        ```
            let x = 10;
            let x = 20;
        ```
        - usage:
            - convert type without renaming
- data type
    - scalar
        - integers - signed (+-) & unsigned (+)
            - normal - default u32
            - decimal - default f64
            - hex
            - octal
            - binary
            - byte (u8)
            - integer overflow (modulo)
        - floating point number
        - booleans
        - characters
    - compound - group of value
        - tuple
            - fixed size arr, different type OK
            - let tup = (val1, val2, val3)
            - let (val1, val2, val3) = tup
- snake_case

## Control
1. If else if else
2. Ternary if else

## Loop
1. Loop {}
    - return value
    - label
    ```
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

## 1. Ownership rules
1. Each value in Rust has a value that's called its owner
2. There can only be 1 owner at a time
3. Value will be dropped once owner out of scope

## 2. Reference rules
Moving ownership is tedious
Alternatif: reference

Rules:
1. At any given time, u can have either __ or __
    - one mutable ref
    - any number of immutable refs
2. References must be static