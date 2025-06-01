fn main() {
    println!("Chapter 4");

    // The generic idea about ownership is that in Rust, there are two types of variables: one
    // that lives on the stack and one that lives on the heap. The idea is that the variable
    // that lives on the stack implements the Copy trait, which (i think) is a function that
    // indicates to Rust that (and how) a variable is copied.
    //
    // All primitive data types: the number types (i32, i64, isize, usize, i128, i8, i16,
    // usize, f32, f43), booLean (true, false), raw string, arrays, and tuples, to mention the least (or maybe the
    // vast majority) all implement the Copy trait and the implication of the variables all
    // live on the stack.
    //
    //
    // Before we proceed, it's important to note that the value a variable holds is tied to
    // that variable, the variable goes out of scope and the value the variable hold get
    // immediately cleaned up. I think all languages has this. So variables that are on the
    // stack are immediately cleaned up when the variables goes out of scope (as it is in
    // almost every other programming language)
    //
    // The caveat of this is when the variable needs to be store on the heap. Programming
    // languages like C and C++, provides functions to manually allocate and deallocate memory
    // on the heap. In Rust, you don't have to manually allocate and deallocate memory, Rust
    // uses the concept of ownership, which has one simple rule, only one variable can have
    // access to the data stored on a specific part of memory. the aim of this is to prevent a
    // situation whereby multiple variables point to one part of memory and you attempt to
    // clear a part of memory that's unallocated leading to undefined behaviour.
    //
    //
    // I think it's important at this point to mention the Drop function. Since Rust doesn't
    // provide a mechanism for developers to manually allocate and deallocate/free memory, Rust
    // has this function implemented by every type of variable that's stored on the heap. The
    // Box::new() class is a container that helps store stuff on the heap.
    // When a function goes out of scope, Rust calls the drop() function on every heap variable (just as it tries to clear out the stack variables of that function). The problem with this is that if multiple variables point to the same part of the heap, Rust would first call drop() on the first variable, successfully deallocating that point in memory, and then call drop() again on subsequent variables, attempting to deallocate a point in memory that's already been deallocated (leading to undefined behaviours).
    //
    //
    // Hence the rule that only one variable can be allowed to have/access(own) the corresponding
    // value in that portion of memory on the heap.
    //
    // Ownership can be passed from one variable to another in two ways:
    // - Assignment to another variable
    // - passing it as an argument to another function
    //
    // DEMONSTRATION 1 (ASSIGNMENT)
    //let x = String::new(); // this is stored on the heap;
    //let y = x; // here's we're assigning the value of x to y. and passing ownership of the value in
    // x to y. Since x and y can't point to the same part of the heap in the same
    // function.
    //println!("{x}") // trying to access x here give the  error: borrow of moved value: x; x no
    // longer has ownership of the value and should not be used
    //
    //
    // DEMONSTRATION 2 (PASSING AS FUNCTION PARAMETERS)
    //fn count_entities(list: &mut Vec<i32>) -> usize {
    //    list.push(6);
    //    list.len()
    //}
    //let mut v = vec![1, 2, 3, 4, 5, 6];
    //
    //count_entities(&mut v);
    //for value in v {
    //    println!("{value}")
    //}

    // That brings us to borrowing
    //
    slices();
}

fn _ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn slices() {
    let mut arr = vec![1, 2, 3, 4, 5];
    let mut arr_slice = &arr[0..=2];

    println!("{:?}", arr);
    println!("{:?}", arr);
    println!("{:?}", arr);
    println!("{:?}", arr_slice);

    arr.pop();

    let new_vec = vec![6, 7, 8, 9, 10];
    arr_slice = &new_vec[..=2];
    for num in arr_slice {
        println!("{num}");
    }
}
