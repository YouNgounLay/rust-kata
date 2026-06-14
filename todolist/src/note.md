# 1) Vec<T> Owned Vector

```rust
fn print_list(tasks: Vec<Task>) {
    // Do something
}
```
 
**What you can do**: 
- Full range of control: Read, Write or Add, Remove, Modify Elements
- Change the collection's length (push, pop, insert, remove)
- The vector lives until it goes out of scope or is moved

**When to use** - to transfer ownership completely. i.e. returning a new vector from a function

# 2) &Vec<T> - Immutable reference

```rust
fn print_list(tasks: &Vec<Task>) {
    // Do something
}
```

**What you can do**: 
- Read elements 
- Call methods that only read (len, is_empty, indexing)
- Cannot add, remove, or modify elements
- Cannot change the vector's length

**When to use** - You just need to look at the contents without changing anything.


# 3) &mut Vec<T> - Mutable reference
```rust
fn print_list(tasks: &mut Vec<Task>) {
    // Do something
    
}
```

**What you can do**: 
- Everything that &Vec<Task> can do (read-only), plus:
- Add, remove and modify elements
- Change the vector's length
- Mutate elements inside the vector

**When to use** -  The function needs to modify the vector (push, pop, or change existing elements). 


# 4) &mut [Task] - Mutable slice

```rust
fn print_list(tasks: &mut [Task]) {
    // Do something
}
```

**What you can do**:
- Read and modify existing elements
- Iterate over elements
- Cannot change the length (no push/pop/insert/remove)
- Cannot add or remove elements - the slice size is fixed 

**When to use** - You only need to modify the elements, not the vector's length. Also, slices can refer to parts of a vector (&mut tasks[1..3]).

# Quick Rule of Thumb

- Read only -> `&[T]` (preferred) or `&Vec<T>`
- Modify elements only -> `&mut [T]`
- Add/remove elements -> `&mut Vec<T>`
- Need to return a new vector -> `Vec<T>`




