# Notes

**Notes from "The Rust Programming Language* by Zayaan Bhanwadia (the notes not the book)**
*chapter 4*
---
## Variables

**Declaration:**
```
let x = 5;
```

*Note: Variables by nature are no mutable in rust*

**Declaration (mutable):**
```
let mut x = 5;
x = 6;
```
*mut keyword allows variables to be mutable*

**Constants:**
- Variables that are  never mutable
- Must be type annotated

```
const x: i32 = 100_000;
```
 
## Shadowing

Shadowing allows a variable with the same name be allowed to be declared twice

Shadowing may allow variables to vary in:
- Types
- Mutability

---

## Arithmetic Operators
| Operator | Function |
|----------|---------|
| + | Add |
| - | Subtract |
| * | Multiply |
| / | Divide |
| % | Modulo |

---

## Data Types

### Scalar Data Types:

**Integers (signed and unsigned)**
- Ex. i32 (signed 32 bit int)
- Ex. u64 (unsigned 64 bit int)
- Default is 32 bit
- Overflow: Values will wrap around

**Floating Point**
- Ex. f32 (32 bit float)

**Boolean**
- Can only take values 'true' and 'false'

**Character**
- Denoted with single quote
- Unicode

### Compound Data Types:

**Tuples:**
- Can hold multiple data types
- Comma separated list in ()
- Extracting:
    - Destructuring
    - Dot notation
- Index starts at 0

Destructuring:
```
let my_tup = ("Hello", 100);
let (word, num) = my_tup;
```

Dot Notation:
```
let num = my_tup.1;
```

**Arrays:**
- Fixed length
- Comma separated list in []
- Indexes start at 0
- MUST access within indexes

Single Initialization Value:
```
let my_list = [0; 8];
// create a list with 8 entries all set to 0
```

---

## Functions
Functions:
- Declared using the *fn* keyword
- Naming uses the snake_case naming convention
    - fn my_function(parameters) {...}
- Parameter types must be specified and separated with commas
    - parameter1: type, parameter2: type
- Returning
    - Use return keyword
    - Declare using *'->'* after parameter bracket
    - Last line is implicitly returned and doesn't need a return keyword or semicolon

```
fn main()
{
    let sum = another_function(3, 2); // calling functions
}

fn another_function(s: i32, p:i32) -> i32 // declaring functions
{
    println!("Another function: {}", s + p);
    return s + p;
}
```


**Statement vs Expression:**
- Expression: Have a return value
- Statement: Do something

---

## Conditionals

Syntax:
- if condition {...}
- else if condition {...}
- else {...}

**condition MUST be a boolean**

Example Conditional Block:
```
fn example(num: i32) -> char
{
    if num < 0 {
        return 'N';
    }
    else if num > 0 {
        return 'P';
    }
    else {
        return 'Z';
    }
}
```

---

## Loops

**Loop Keyword**
- Excecute until a break statement is added
- Can return values

```
let mut count = 0;
let result = loop 
    {
        count += 1;

        if count == 10 {
            break count;
        }
    };
```

**While Loops**
- Exceute while a condition is true
  
```
let mut number = 3;
while number != 0
{
    println!("{}", number);
    number -= 1;
}
```

**For Loop**
- Standard foor loops in other languages
- Also have a for each loop for collections
  
```
let l = [1, 2, 3, 4];
for element in l.iter() 
{
    println!("{}", element);
}

for number in 1..4
{
    println!("{}", number);
}
```

---

## Comments

**Inline Comments**
- Denoted using //

**Multi Line Comments**
- Denoted using /* ... */

---

## Ownership

Allows rust to make memory safe garantees without a garbage collector

**Garbage Collector:**
- Error free (may have bugs)
- Faster to write
- Lack of control
- Slow and unpredictable runtime
- Larger program size

**Manual Management:**
- Full control over memory
- Fast runtime and smaller program size
- Error prone
- Slower write time

**Ownership Model:**
- Control over memory
- Error free (compiler)
- Fast runtime and smaller program time
- Slow writing time and learning curve

---

### Stack vs Heap

Stack:
- Fixed size and cannot grow in size during runtime
- Stores stack frames
  - Made for functions
  - Stores local variables
  - Size determined at compiled time
  - Variable lives as long as stack lives

Heap:
- Less organized and dynamic
- Controlled lifetime by user
- Allocates memory for variables on stack 
  - Returns a pointer to memory location in heap

*Pushing to stack is faster than allocating on heap (needs to look for space)*
*Accessing on heap is faster than accessing on stack (must follow pointer)*

### Ownership Rules

1. Each value in Rust has a variable called owner
2. there can only be one owner at a time
3. When the owner goesd out of scope the value will be dropped

```
// s is not valid here since its not declared
fn main() {
     let s = String::from("hello");
     // s is valid from this point
}
// s is not valid here since owner is out of scope
```

### Interactions

Rust defaults to the move operation when setting 1 variable equal to another. This results in the first owner being dropped and no longer valid.

This is how interactions happen for datatypes that are **NOT** int, float, char

```
let x = String::from("Hello");
let y = x.clone();

println!("{}", x); // x is still valid after clone
```

To make a clone we use the **.clone()** method on variables to perform a shallow copy (allocate memory for a new object on the heap).

```
let x = String::from("Hello");
let y = x;

println!("{}", x); // this will result in an error
```

**Copy trait:** trait allowing ints, floats, and chars to be copied instead of moved.

### Ownership Functions

1. Passing parameters into a function transfers ownership.
2. Returning variables passes ownership of the variable.

*The above statements hold for datatypes without the copy trait*

```
fn main()
{
    let s = String::from("Hello");
    new_function(s);
    println!("{}", s); // this will cause an error
}

fn new_function(some: String) {}
```

### References

**References:**
- References dont take ownership of the value
- Represent a pointer to a variable and multiple can exist
- Denoted with a &
- Immutable by default

**Borrowing:** Passing references as function parameters

```
fn main() {
    let s1 = String::from("Hi");
    let num = len(&s1);
    println!("{}, {}", num, s1);
}

fn len(s: &String) -> usize {
    s.len()
}
```

**Mutable References:**
- Allow the borrower to modify data without taking ownership
  - The variable itself must be mutable
- Only can borrow 1 mutable reference at a time
- Prevent datarace (read and write same time)
- Denoted &mut varname

**Dangling Pointers:**
- Reference pointing to invalid memory
- Borrowed value cannot be returned
- Compile time error

**String Slicing:**
- Act as pointers to parts of start of the string
- Point to the string object itself
- Denoted &str

```
let mut s = String::from("Hello World");
let hello = &s[..5]; // 0 - 5
let world = &s[..]; // 6 - 11
```

## Functions

| Function Name | Purpose | Example |
| ------------- | ------- | ------- |
| `len` | Returns the length of a string, slice, or collection | `word.len()` -> `5` |
| `iter` | Returns an iterator over a collection | `nums.iter()` |
| `clone` | Creates a copy of a value | `word.clone()` |
| `push` | Adds an item to the end of a vector or string | `vec.push(4)` |
| `pop` | Removes and returns the last item | `vec.pop()` |
| `trim` | Removes whitespace from the start and end of a string | `text.trim()` |
| `contains` | Checks whether a value or substring exists | `text.contains("hi")` |
| `split_whitespace` | Splits a string into words by whitespace | `text.split_whitespace()` |


## Structs
...