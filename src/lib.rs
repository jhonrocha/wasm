use std::format as f;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> String {
    String::from("Hello World!")
}

#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    f!("Hello, {name}!")
}

#[wasm_bindgen]
pub fn fibo(num: u64) -> u64 {
    if num <= 1 {
        return num;
    }

    let mut resp = [0, 1];
    for _ in 1..num {
        resp = [resp[1], resp[0] + resp[1]]
    }
    resp[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello World!");
    }

    #[test]
    fn test_hello() {
        assert_eq!(hello("Jhon"), "Hello, Jhon!");
    }

    #[test]
    fn test_fibo() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(10), 55);
    }
}
