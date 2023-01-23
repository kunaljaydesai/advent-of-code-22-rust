use std::collections::BinaryHeap;
use std::fs::File;
use std::path::Path;

/*
    BufRead isn't explicitly mentioned anywhere in the file, why do we need it imported?
    Additionally, if it's not included, then `lines` method throws an error
    I see that this has something to do with traits...
    Reading: https://doc.rust-lang.org/book/ch10-00-generics.html
    - Traits are similar to interfaces in other languages, but there are some differences - according to Rust Book
    - They can have default implementations
    - Traits can be parameters (any object that implement's that trait can be passed as a paramter)

    BufReader implements the BufRead trait:
    - https://doc.rust-lang.org/std/io/struct.BufReader.html#impl-BufReader%3CR%3E
    - https://doc.rust-lang.org/src/std/io/buffered/bufreader.rs.html#55-96

    Why do traits need to be imported in Rust?
    - https://stackoverflow.com/questions/25273816/why-do-i-need-to-import-a-trait-to-use-the-methods-it-defines-for-a-type
*/
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    /*
    What is the '?' operator in Rust?
    - https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about
    - https://www.becomebetterprogrammer.com/rust-question-mark-operator/#:~:text=operator%20in%20Rust%20is%20used,or%20Option%20in%20a%20function.
    */

    /*
    Related: Exceptions in Rust
    https://doc.rust-lang.org/book/ch09-00-error-handling.html

    Two types of errors: recoverable and unrecoverable.
    - Recoverable errors are where you just want to report the error to the user and retry the operation
    - Unrecoverable errors are things like accessing beyond the end of an arraya and you immediately want to stop the program

    For recoverable errors, there is type Result<T, E> and panic! macro for non-recoverable errors
    Result is a type that represents either success or failure: https://doc.rust-lang.org/std/result/enum.Result.html#:~:text=Result%20is%20a%20type%20that,the%20module%20documentation%20for%20details.
    T contains the success type and E contains the Error type.
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    When handling the Result<T, E> return type, it's common to use the match keyword like so:

    match result {
        Ok(success) => success,
        Err(error) => panic!(error),
    }

    For unrecoverable errors, there is type panic!. You can either call this explicitly in the code, or by doing something bad like accessing the end of an array.
    You can have Rust show a stack trace of the panic by setting an environment variable RUST_BACKTRACE = 1
    */

    let path = Path::new("day1.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut curr_sum = 0;
    let mut heap = BinaryHeap::<i32>::new();
    /*
    lines() is a function implemented by the BufRead trait. BufReader implements the BufRead trait.
    */
    for line_result in reader.lines() {
        /*
        Calling a function on an object changes the object's ownership
        Assignment leads to ownership and re-assignment also moves ownership, read here to understand why: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move
        However, if a value implements the Copy trait, then the value is copied

        https://depth-first.com/articles/2020/01/27/rust-ownership-by-example/
        String ownership examples: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type
        */

        /*
        References:
        If you don't want to copy, but you also don't want to change ownership, you can borrow with the & character
        If a function takes in a reference, the function doesn't own the value so it won't drop it upon completion of the function's execution
        References are immutable by default, but adding the `mut` keyword makes it so that the reference is modifiable.
        There is one big caveat which is that if you have a mutable reference to a value, you can't have any other references to that value.


        https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
        */

        let line = line_result?;

        /*
        Why can we call this function but still access `line` in the else statement?
        Because .len takes in a string reference, so line is borrowed, not moved.
        Ownership does not change as a result
        */
        let is_new_line = line.len() == 0;
        if is_new_line {
            heap.push(curr_sum);
            curr_sum = 0;
        } else {
            /*
            What is unwrap? Give me the result and if there's an error, then panic. It's somewhat equivalent to:
            match line {
                Ok(line) => line,
                Err(e) => panic!("Error"),
            }
            */
            curr_sum += line.parse::<i32>().unwrap();
        }
    }
    const TOP_K_ELVES: i32 = 3;
    let mut total_calories = 0;
    for _ in 0..TOP_K_ELVES {
        total_calories += heap.pop().unwrap();
    }

    /*
        What is the ! operator on println and why don't I need to import println via use?
    */
    println!(
        "{:?} total calories were retrieved by the top {:?} elves",
        total_calories, TOP_K_ELVES,
    );
    Ok(())
}
