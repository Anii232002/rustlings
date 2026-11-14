// This exercise is an altered version of the `errors4` exercise. It uses some
// concepts that we won't get to until later in the course, like `Box` and the
// `From` trait. It's not important to understand them in detail right now, but
// you can read ahead if you like. For now, think of the `Box<dyn ???>` type as
// an "I want anything that does ???" type.
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, the `Box` is declared as of type `Box<dyn Trait>` where
// `Trait` is the trait the compiler looks for on any value used in that
// context. For this exercise, that context is the potential errors which
// can be returned in a `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Add the correct return type `Result<(), Box<dyn ???>>`. What can we
// use to describe both errors? Is there a trait which both errors implement?
fn main()-> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
/*
In main(), two different lines use the ? operator, and each line can fail with a completely different error type:
• Line 1 (pretend_user_input.parse()?): Fails with a ParseIntError if the string is not a valid number.
• Line 2 (PositiveNonzeroInteger::new(x)?): Fails with your custom CreationError if the number is zero or negative.
Because a normal Result<T, E> only accepts one concrete error type for E, Rust cannot return both ParseIntError and CreationError directly. Wrapping the error in Box<dyn Error> solves this in three steps:
• Shared Trait (dyn Error): Both ParseIntError (from the standard library) and CreationError (via impl Error for CreationError {} on line 34) implement the std::error::Error trait. Writing dyn Error tells Rust to accept any type that implements that trait.
• Heap Allocation (Box<...>): Because ParseIntError and CreationError take up different amounts of memory, Rust does not know their exact size at compile time. Putting them inside a Box stores the actual error on the heap and gives main a fixed-size pointer to return.
• Automatic Conversion (? + From): When either line fails, the ? operator automatically uses Rust's built-in From trait to package the specific error into a Box<dyn Error> and exit main early.

 */