fn main() {
    println!("Hello, world!");
}
// Cargo is Rust's build system and package manager (kinda similar to Python's pip)
// It handles tasks like building codes, downloading libraries, etc.
// Commands: --------------------------------------------------------
// 1. cargo new <code-name>   --- creates new project
// 2. cargo build  --- (should run inside a cargo project's directory) builds the code, meaning, creates an executable inside target/debug/hello-cargo.exe
// 3. cargo run   --- runs the project
// 4. cargo check   --- quickly checks your code for bugs, warnings, errors. You may not want to create an executable or build an executable as it takes time. In that case, cargo check comes handy.
// 5. cargo build --release   --- When you are done coding and debugging, you can create a release which creates executable in ./target/release instead of ./target/debug
// ------------------------------------------------------------------
// Using cargo run is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use cargo run.