// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.

    //SAFETY: The caller must ensure that the provided address is valid, properly aligned,
    // and points to a `u32` that can be safely modified. The caller must also ensure
    // that there are no other aliasing mutable references to the same memory location
    // during this function call. This function does not perform any of these checks itself.
    //unsafe { todo!("Your code goes here")
    // Cast the address back to a mutable pointer to a u32.
    unsafe {
        // Cast the address back to a mutable pointer to a u32.
        let ptr: *mut u32 = address as *mut u32;

        // Dereference the pointer to get a mutable reference to the value.
        if !ptr.is_null() {
            *ptr = 0xAABBCCDD;
        }
        // We are using `std::ptr::write` to avoid any potential aliasing issues.
        //std::ptr::write(mut_ptr, 0xAABBCCDD);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
