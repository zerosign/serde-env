extern crate serde;

pub mod de;
pub mod error;
pub mod ser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
