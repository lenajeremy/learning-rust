fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
    println!("{x}");

    // shadowing
    //
    let j = 5;
    let j = j + 1;

    {
        let j = 50;
        println!("{j}")
    }
    println!("{j}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
}
