#![crate_type = "proc-macro"]
extern crate proc_macro;

use quote::quote;
use size_format::SizeFormatterBinary;
use std::fmt;
use std::mem::size_of;
use syn::{parse_macro_input, DeriveInput};

/// The PrintSize proc macro will print the size of the
/// struct.
#[proc_macro_derive(PrintSize)]
pub fn print_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // The size of
    println!("Derive Input:\n{:?}", input);

    let expanded = quote! {
        const HELLO: &'static str = "HELLO!";
    };

    // Print out the generated code.
    println!("{}", expanded);
    // Hand the generated tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct AllocSize {
    num_bytes: u64,
}

impl From<u64> for AllocSize {
    fn from(val: u64) -> Self {
        Self { num_bytes: val }
    }
}

impl From<usize> for AllocSize {
    fn from(val: usize) -> Self {
        AllocSize::from(val as u64)
    }
}

impl fmt::Debug for AllocSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatter = SizeFormatterBinary::new(self.num_bytes);
        write!(f, "{0}", formatter.to_string())
    }
}

impl fmt::Display for AllocSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// StackSized is implementable for Sized types.
/// It reports the size in bytes of the type when stack allocated.
trait StackSized {
    fn stack_size(&self) -> AllocSize;
}

impl StackSized for () {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for bool {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for u8 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for u16 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for u32 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for u64 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for u128 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for i8 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for i16 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for i32 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for i64 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for i128 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for f32 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for f64 {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for char {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for usize {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl StackSized for isize {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

impl<T: Sized> StackSized for Box<T> {
    fn stack_size(&self) -> AllocSize {
        AllocSize::from(size_of::<Self>())
    }
}

#[cfg(test)]
mod tests {
    use super::AllocSize;
    use super::StackSized;

    #[test]
    fn unit_stack_size() {
        let expected = AllocSize::from(0u64);
        let observed = ().stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn bool_stack_size() {
        let expected = AllocSize::from(1u64);
        let observed = true.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn u8_stack_size() {
        let expected = AllocSize::from(1u64);
        let observed = 0u8.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn u16_stack_size() {
        let expected = AllocSize::from(2u64);
        let observed = 0u16.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn u32_stack_size() {
        let expected = AllocSize::from(4u64);
        let observed = 0u32.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn u64_stack_size() {
        let expected = AllocSize::from(8u64);
        let observed = 0u64.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn u128_stack_size() {
        let expected = AllocSize::from(16u64);
        let observed = 0u128.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn i8_stack_size() {
        let expected = AllocSize::from(1u64);
        let observed = 0i8.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn i16_stack_size() {
        let expected = AllocSize::from(2u64);
        let observed = 0i16.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn i32_stack_size() {
        let expected = AllocSize::from(4u64);
        let observed = 0i32.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn i64_stack_size() {
        let expected = AllocSize::from(8u64);
        let observed = 0i64.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn i128_stack_size() {
        let expected = AllocSize::from(16u64);
        let observed = 0i128.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn f32_stack_size() {
        let expected = AllocSize::from(4u64);
        let observed = 0f32.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn f64_stack_size() {
        let expected = AllocSize::from(8u64);
        let observed = 0f64.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn char_stack_size() {
        let expected = AllocSize::from(4u64);
        let observed = '0'.stack_size();
        assert_eq!(expected, observed);
    }

    #[test]
    fn box_stack_size() {
        let expected = 0usize.stack_size();
        let observed = Box::new(0).stack_size();
        assert_eq!(expected, observed);
    }
}
