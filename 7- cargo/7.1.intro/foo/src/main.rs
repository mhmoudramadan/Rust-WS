// Each file in tests is a separate integration test, i.e. a test that is meant to test your library
//  as if it were being called from a dependent crate.

// The Testing chapter elaborates on the three different testing styles: Unit, Doc, and Integration.



/// `#[cfg(test)]` is an attribute in Rust that indicates that the following module or block of code
/// should only be compiled and included in the build when running tests. This is commonly used to
/// include test-specific code that should not be part of the final production build of the application.
#[cfg(test)]
mod tests {
    // Import the necessary modules
    use std::fs::OpenOptions;
    use std::io::Write;

    // This test writes to a file
    /// `#[test]` is an attribute in Rust that is used to mark a function as a test function. When you
    /// annotate a function with `#[test]`, it indicates to the Rust test framework that this function
    /// should be executed as part of the test suite. This allows you to write test functions that will
    /// be run when you execute your tests using tools like `cargo test`.
    #[test]
    fn test_file() {
        // Opens the file mahmoud.txt or creates one if it doesn't exist.
    /// This code snippet is opening a file named "mahmoud.txt" or creating one if it doesn't exist.
    /// Here's a breakdown of each method called on `OpenOptions::new()`:
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("mahmoud.txt")
            .expect("Failed to open mahmoud.txt");

        // Print "mahmoud" 5 times.
    /// The code snippet `for _ in 0..5 { file.write_all("mahmoud\n".as_bytes()).expect("Could not write
    /// to mahmoud.txt"); }` is iterating 5 times using a range from 0 to 4 (inclusive) denoted by
    /// `0..5`. During each iteration, it writes the string "mahmoud" followed by a newline character to
    /// the file represented by the `file` variable. The `write_all` method writes the specified byte
    /// slice to the file, and in this case, it is converting the string "mahmoud\n" to a byte slice
    /// using `as_bytes()` before writing it to the file. If there is an error during the writing
    /// process, it will panic with the message "Could not write to mahmoud.txt".
        for _ in 0..5 {
            file.write_all("mahmoud\n".as_bytes())
                .expect("Could not write to mahmoud.txt");
        }
    }

    // This test tries to write to the same file
    #[test]
    fn test_file_also() {
        // Opens the file mahmoud.txt or creates one if it doesn't exist.
      /// This code snippet is opening a file named "mahmoud.txt" or creating one if it doesn't exist.
      /// Here's a breakdown of each method called on `OpenOptions::new()`:
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("mahmoud.txt")
            .expect("Failed to open mahmoud.txt");

        // Print "ramadan" 5 times.
        /// This code snippet is iterating 5 times using a range from 0 to 4 (inclusive) denoted by
        /// `0..5`. During each iteration, it writes the string "ramadan" followed by a newline
        /// character to the file represented by the `file` variable. The `write_all` method writes the
        /// specified byte slice to the file, and in this case, it is converting the string "ramadan\n"
        /// to a byte slice using `as_bytes()` before writing it to the file. If there is an error
        /// during the writing process, it will panic with the message "Could not write to mahmoud.txt".
        for _ in 0..5 {
            file.write_all("ramadan\n".as_bytes())
                .expect("Could not write to mahmoud.txt");
        }
    }
}
