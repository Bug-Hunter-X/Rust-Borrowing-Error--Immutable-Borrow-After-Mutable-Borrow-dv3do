fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference
    let z = &x;    // Immutable reference

    *y += 1; // Modifying x through y
    println!("x = {}", x); // Output: x = 6

    // This line will cause a compile-time error because we can't borrow
    // x immutably in z while it is already mutably borrowed in y
    println!("x = {}", *z);
}