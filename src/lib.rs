
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod admin;
pub mod ds;
pub mod hosted;
pub mod nvm;
pub mod queue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
