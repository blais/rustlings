fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a: [i32; 200] = [1; 200];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        // println!("{:?}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
