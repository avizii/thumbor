use std::borrow::{Borrow, BorrowMut};

/// https://wiki.jikexueyuan.com/project/rust-primer/intoborrow/borrow.html
/// https://doc.rust-lang.org/std/borrow/trait.Borrow.html
/// https://github.com/rust-lang/rust/issues/24140

// Borrow
// BorrowMut
fn check<T>(s: T)
where
    T: Borrow<str>
{
    assert_eq!("Hello", s.borrow());
}

fn bor_test() {
    let s = "Hello".to_string();
    check(s);

    let s = "Hello";
    check(s);
}

struct Bor(String);

fn bor_test2() {
    let b = Bor("borrow in 02.24".to_string());
    // in this case `&b` is similar to `b.borrow()`
    // because the default implement in std:  `impl<T: ?Sized> Borrow<T> for T {}`
    print_bor(&b);
    print_bor(b.borrow());

    // in this case `&mut b` is similar to `b.borrow_mut()`
    // because the default implement in std:  `impl<T: ?Sized> BorrowMut<T> for T {}`
    let mut b = Bor("mut borrow in 02.24 evening".to_string());
    print_mut_bor(&mut b);
    print_mut_bor(b.borrow_mut());
}

fn print_bor(b: &Bor) {
    println!("bor.name = {}", b.0);
}

fn print_mut_bor(b: &mut Bor) {
    println!("mut bor.name = {}", b.0);
}




// ToOwned


// AsRef


// AsMut


fn main() {
    bor_test();

    bor_test2();
}