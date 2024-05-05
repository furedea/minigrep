# minigrep

minigrep is a command-line tool introduced in Chapter 12 of The Rust Programming Language (commonly known as TRPL).
It can search a plain text file for lines containing a specified query string.
It is used as an example to learn basic Rust concepts and file I/O operations.

## Features

- Search for lines containing a specified query string in a given file
- Case-sensitive and case-insensitive search modes
- Configurable via command-line arguments and environment variables

## Installation

To install minigrep, you need to have Rust and Cargo installed on your system. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust and Cargo set up, you can clone this repository and build the project using the following commands:

```bash
git clone https://github.com/furedea/minigrep.git
cd minigrep
cargo build --release
```

The compiled binary will be located in the target/release directory.

## Usage
To use minigrep, run the compiled binary followed by the query string and the path to the file you want to search:

```bash
./target/release/minigrep <query> <filename>
```

For example, to search for the word "hello" in a file named "example.txt", you would run:

```bash
./minigrep hello example.txt
```

By default, the search is case-sensitive. To perform a case-insensitive search, set the CASE_INSENSITIVE environment variable to any value before running the command:

```bash
CASE_INSENSITIVE=1 ./minigrep hello example.txt
```

## Project Structure

The project is organized into the following files and directories:

- src/main.rs: The entry point for the binary crate, which calls the minigrep function from the library crate.
- src/lib.rs: The main library file, which contains the minigrep function and the run function that performs the file I/O and search operations.
- src/config.rs: Defines the CommandLineConfig struct and its associated methods for parsing command-line arguments and environment variables.
- src/search.rs: Contains the search and search_case_insensitive functions for performing the actual search operations on the file contents.
- src/tests.rs: Includes unit tests for the various functions and modules in the project.

## Contributing

Contributions to minigrep are welcome! If you find a bug or have a suggestion for improvement, please open an issue on the GitHub repository. If you'd like to contribute code, please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Relation to TRPL

minigrep is a project introduced in Chapter 12, "Creating command-line programs" of The Rust Programming Language.
This chapter describes the basic concepts of developing command-line tools in Rust, such as parsing command-line arguments, file I/O, error handling and writing tests.

minigrep is used as an example to learn these concepts in a practical way, making it a very useful project for those new to Rust.
Through this project, you will also learn about important language features such as Rust's module system, ownership, lifetime and traits.

By implementing minigrep while reading through Chapter 12 of TRPL, you will effectively learn the basics of Rust programming.
