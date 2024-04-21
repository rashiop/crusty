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