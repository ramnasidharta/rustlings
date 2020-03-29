// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = Vec::new();

    //let mut vec1 = fill_vec(vec0);
    // vec0 cannot be used after here, because vec1 is pointing to the same data
    // in the heap, so if both goes out of scope, there would be a double free error.
    // So Rust says: vec0 is not valid anymore, it was moved and now fill_vec has
    // ownership over it.

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let mut vec1 = fill_vec(vec0);  // was in line 9.
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
