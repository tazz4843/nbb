#![feature(once_cell)]

mod cache;
mod cfg;
mod render;

pub use render::render_markdown;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
