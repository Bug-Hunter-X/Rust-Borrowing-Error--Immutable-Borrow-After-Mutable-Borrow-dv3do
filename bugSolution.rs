fn main() {
    let mut x = 5;
    // Option 1: Modify the mutable reference first.
    { 
        let y = &mut x;
        *y += 1; // Modify x through the mutable reference
    } 
    // Now it's safe to create an immutable reference 
    let z = &x;
    println!("x = {}", *z); // Output: x = 6

    // Option 2: Create immutable references before mutable borrows
    // Not always possible depending on your code
    // let z = &x; // This would cause a compile-time error
    // *y += 1; 
    // println!("x = {}", *z);
} 