#![allow(dead_code)]
#![feature(conservative_impl_trait)]

extern crate zmq;
extern crate url;

mod frame;
mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
