
pub mod coinmarket;
pub use coinmarket::rest_api as api;


extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

#[macro_use]
extern crate serde_derive;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        // TODO real tests!

        assert_eq!(2 + 2, 4);
    }
}
