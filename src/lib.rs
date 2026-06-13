//! This is the library portion of your project. Text here will appear in the docs
//! when you run `cargo doc`.

#![deny(clippy::all)]
#![forbid(unsafe_code)]

/// Documentation comment for [`hello_world`].
pub fn hello_world() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::hello_world();
    }
}
