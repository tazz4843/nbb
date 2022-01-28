#[macro_use]
extern crate serde;

mod blog_post;
mod index;
mod not_found;
mod post;
mod renderer;

pub use blog_post::*;
pub use index::*;
pub use not_found::*;
pub use post::BlogPost;
pub use tera::Error as TeraError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
