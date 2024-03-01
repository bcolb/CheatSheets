# Rust

Some brief notes on getting started with Rust inspired by the [Rust in Motion](https://www.oreilly.com/library/view/rust-in-motion/10000MNLV201742/) video series and the [rust-lang book](https://doc.rust-lang.org/book/)

Current Rust version when I started this was rustc 1.76.0 (07dca489a 2024-02-04).

## Install Rust with Rustup

Go to [rustup.rs](https://rustup.rs)

Check version

```rustc --version```

Check the version of cargo (the rust package manager)

```cargo --version```

[View the docs](https://doc.rust-lang.org/)

## Hello World

Create an executable Rust program

```cargo new --bin helloworld```

Then navigate to the new subdirectory

```cd helloworld```

The main code will live in src/main.rs

```
fn main() {
    println!("Hello, world!");
} 
```
Two ways to build:

```cargo build```

Run with

```./target/debug/helloworld```

Or build with

```cargo build --release```

And run with 

```./target/release/helloworld```

Or do it all in one command (build and run)

```cargo run```

## Simple Variables

## Data Types

### Boolean

Typical boolean, true or false. Used in control flow.

```
let x = true;
let y = false;
```

### Integer

Numbers, no decimal. Signed and unsigned.

Signed: i8, i16, i32, i64
Unsigned: u8, u16, u32, u64

i8: -128-127
u8: 0-255

isize and usize are infrastructure dependent 

```
let x = 16;
```

### Floating Point

Numbers with decimal points

f32 and f64

```
let x = 3.14;
```

### Character

char Data type, a unicode scalar value (holds more than just ASCII)

Use single quotes (strings use double quotes)
```
let c = 'a';
let d = 'v';
```

## Compound Variables

### Tuples

Group multiple elements together. They do not need to be the same type.

```let tup = (7, 'a', false);```

Reference an item in a tuple as such

```
let first = tup.0;
let second = tup.1;
```

Destructuring is done as such

```
let (x, y, z) = tup;
```

### Array

Arrays have a fixed length set when initialized, cannot append. All values must have the same type.

```let a = [2, 4, 1, 6, 7, 10];```

Access an element of an array

```a[0]```

Modify an element

```let third = a[2]```

Use a vec instead of an array if the size of the number of elements needs to change, as arrays are fixed size immutable and vectors are dynamic mutable.

### Slice

Reference to a continguous subset of data.

```
let a = [10, 20, 30, 40, 50]
let b = &a[0..1]
```

String slice (&str)
```
let s = "hi";
```

## Functions

Here is the form of a function in pseudocode

```
fn functionname(param1: type1, param2: type2) -> return_type {
    // body of the function
    data
}
```

An example function

```
fn hello_someone(name: &str) {
    println!("Hello {}!", name);
}
```

Calling a function

```
 fn main() {
    hello_someone("Brice");
 }
```

Returning a value

```
fn cube(num:i32) -> i32 {
    num * num * num
}
```

The idiomatic way to return is just leave the semicolon off the last line and that data is returned.

There is a return keyword that can be used to return out of a function early as well.

## Control Flow

### if / else if / else

Expression needs to evaluate to a boolean
```
if expression {
    // code if true
} else if expression2 {
    // code to run if expression2 is t rue
} else {
    // other code to run if both expressions are false
}
```

Simple control flow example
```
fn tentotwenty(num:i32) {
    // Prints different statements to demonstrate control flow
    if num < 10 {
        println!("Number {} is less than 10.", num);
    } else if num >= 10 && num <= 20 {
        println!("Number {} is between 10 and 20 inclusive.", num);
    } else {
        println!("Number {} is greater than 20.", num);
    }
}
```

### loop, while, for

Using the loop keyword loops forever until you break out of it with break.

```
fn testloop(){
    loop {
        println!("Type 'break' to break out of the loop.");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");

        if word.trim() == "break" {
            break;
        }
    }
    println!("Congrats, you typed and entered 'break'!");
}
```

While loops break on a condition

```
fn testwhileloop() {
    let mut word = String::new();
    while word.trim() != "break" {
        println!("Type 'break' to break out of the while loop.");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }
    println!("Congrats, you typed and entered 'break' to exit the while loop!");
}
```

For loops are the most common

```
fn testforloop() {
    // 1..11 is create a range between 1 (inclusive) and 7 (exclusive), so 1,2,3,4,5,6
    for i in 1..7 {
        println!("Now on number {}", i);
    }    
}

```
### match

Pattern matching. Matches against a number of expressions and then runs code that codes with that pattern (like switch/case)

```
let x = 5;
match num {
    1 => println!("The number is one"),
    2 => println!("The number is two"),
    3 => println!("The number is three"),
    4 => println!("The number is four"),
    _ => println!("The number is not 1-4 but something else"),
}
```

## Enums

Enums (enumerations) define custom types, works well with pattern matching.

Properties:
- Can only be one value at a time
- Can list (enumerate) all possible values

```
// for use in enum example
enum PrimaryColor {
    red,
    yellow,
    blue,
}

fn use_enum() {
    // code
    let color = PrimaryColor::red;
    color_something(color);
}

fn color_something(color: PrimaryColor) {
    println!("Do something with the color");
    match color {
        PrimaryColor::red => println!("Hex code for red is #FF0000"),
        PrimaryColor::yellow => println!("Hex code for yellow is #FFFF00"),
        PrimaryColor::blue => println!("Hex code for blue is #0000FF"),
    }
}
```

## Structs

Short for structures, custom types that group attribtues together (primitive data types).

Structs are for when you want to have the same attributes for each value of a type.

Enums by comparison are for describing a choice between a set of values.

```
enum WorkerType {
    cpu,
    gpu,
}

struct Worker {
    name: String,
    id_number: i8,
    type: WorkerType,
    cores: u8,
    cores_in_use: u8,
    current_jobs: u8,
    jobs_computed: u8,
}

let compute_instance = Worker {
    name: String::from("Worker 1"),
    id_number: 1,
    type: WorkerType::cpu,
    cores: 2,
    cores_in_use: 0,
    current_jobs: 0,
    jobs_computed: 0,
};

println!(
    "{} has {} cores",
    compute_instance.name,
    compute_instance.cores,
);
```

Tuple structs - have a name for a struct but not for fields

```
struct Triangle(u32, u32, u32);

let triangle1 = Triangle(3,4,5);
```

Unit structs - can define methods on them

```
struct ExampleStruct;
```

## Methods

Structs define data on custom types where methods define behavior on custom types.

Can be defined in the context of an enum or a struct.

```
impl Worker {
    fn run_job(&mut self, job_cores: u8) {
        if (job_cores + self.cores_in_use) <= self.cores {
            println!("Starting a new job using {} cores", job_cores);
            self.cores_in_use = self.cores_in_use + job_cores;
            self.current_jobs = self.current_jobs + 1;
        } else {
             println!("Not enough cores available, currently using {} of {} availale cores and {} were requested.", self.cores_in_use, self.cores, job_cores);
        }
    }
}


compute_instance.run_job(1);
compute_instance.run_job(2);
```

Can define a constructor method with the 'new' keyword.

## Ownership

Ownership is how Rust manages memory allocation and deallocation.

Ownership rules in Rust:
- Each value in rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Borrowing

Borrwing doesn't require cloning, can pass a reference to a function (comparable to a pointer but checked by compiler).

Rust ensures you can't have a reference to an invalid space in memory.

```
fn name() -> & String {
    let n = String::from("bcolb");
    &n
}

fn main() {
    let the_name = name();
}
```

Rust makes sure references are always valid. In general, having functions borrow values rather than take ownership is more idiomatic.

## Slices

A slice is a data type that always borrows data owned by some other structure; consists of a pointer and a link.

```
let s_slice = &s[2..4]
```

Can create from strings, arrays, and vectors.

```
let var = [0.00, 1.23, 4.56, 5.71];
let var_slice = &var[..1];
```

Can make it mutable too.

```
let mut v = vec![10, 20, 30, 40];
v.push(50);
let v_slice = &v[1..2];
```

The borrow checker makes sure that slices are always valid.

Slice indices are checked at runtime if they can't be checked at compile time.

Arrays, vectors, slices.

Rust feature 'deref' converts a reference to a string to a string slice.

## Mutable Borrowing


```
let mut x = 5;

fn increment(val: &mut u32){
    val += 1;
}
```

## Borrowing Code Patterns

  - New Scopes
  - Temporary variables
  - Entry API
  - Splitting up structs

## Borrowing beyond memory

### Sockets

Sockets are system resource that are a connection to a network endpoint for sending and receiving data.

TCP - Transmission Control Protocol (underlies much of the internets traficc)
- Bind to a port
- Close when done

Memory vs Sockets: Similarities

Memory
- use after free
- Double Free
- Memory leaks
- Mitigated with garbage colelction
- Mitigated with ownership

Sockets
- user after close
- Closing twice
- Socket Leaks
- Not mitigated with garbage collection
- Mitigated with ownership

### Other Borrowing

- Mutex<T> (Mutual Exclusion): only let one thread at a time change the inner value
  - Lock is automatically released once owner goes out of scope
- Rc<T> (Reference Counted) Allows for multiple owners to a reference
  - Keeps track of owner count and cleans up once the last owner goes out of scope
- Files - close when done using when owner goes out of scope
- Custom Types - use the Drop trait
  - One method: drop
  - drop takes &mut self


