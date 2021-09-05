# ffi-exmaples

Rust FFI examples with C.

## C struct

We can initialize C struct `T` in Rust by:

```rust
let t: T = std::mem::zeroed();
```

## Raw pointers

We can use `as` to convert Rust reference to C pointer, the `mut` and `const` keep unchanged.


| Rust | C | to C | from C (unsafe) |
| :-: | :-: | :-: | :-: |
| `&T`| `const T*` | `&x as *const T` |  `&*p_x`|
| `&mut T`| `T*` | `&x as *mut T` | `&mut *p_x`|
| `*const T` | `NULL` | `::std::ptr::null::<T>()` | /  |


## Strings

There are several scenarios when convert strings:

- Copy
  - Rust `String` to (or from) C `char[]`
- Non-Copy
  - Rust `str` to (or from) C `char[]`

Rust's string is an array of `u8`, but C's string is an array of `i8`, a.k.a, `c_char`. This introduce extra complexity in conversion of Rust and C strings.



### Convert string with copy

| Rust | C | to C | from C (unsafe) |
| :-: | :-: | :-: | :-: |
| array `[i8; N]` | `char[N]` | loop and assign | loop and assign |

### Convert string without copy

| Rust | C | to C | from C (unsafe) |
| :-: | :-: | :-: | :-: |
| array `[i8; N]` | `const char*` | `as_ptr()` | `std::ffi::CStr::from_ptr()` |


## Polymorphism and trait

Some C library accepts a callback and trigger it when certain event happens, the Rust side needs to:

- register a `Box<dyn Trait>` and callback function to the C lib
- Implement the trait in custom ways

This requires the conversion from Rust `Box<dyn Trait>` to C `void*`, which is non-trivial.


## Unmovable self reference type
