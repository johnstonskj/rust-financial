use crate::request::RequestResult;

pub trait Provider
    where Self: std::marker::Sized {

    fn new() -> RequestResult<Self>;

    fn attribution(&self) -> String;

    fn url(&self) -> String;
}
