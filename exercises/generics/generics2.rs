// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl Wrapper<u32> {
    pub fn add_42(self ) ->u32 {
        self.value + 42 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_add_42() {
        let wrapper = Wrapper::new(42);
        assert_eq!(wrapper.add_42(),84);
    }
    
    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
