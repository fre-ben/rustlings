// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    println!("{}", x);
    let y = &mut x;
    *y += 100;
    println!("{}", x);
    let z = &mut x;
    *z = *z + 1000;
    println!("{}", x);
    assert_eq!(x, 1200);
}
