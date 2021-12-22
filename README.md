# ffi-examples

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


## String

There are two important differences between Rust string and C string, which makes the conversion difficult:

- Rust `String` is an array of `u8`, but c string is an array of `i8`.

- Rust `String` (and `str`) does not guarantee it ends with `\0`.

The 2nd difference makes it **unsafe** to directly covert a Rust string into a `const char*` (though there is no any warning that prevent you do so).  See [Rust FFI: Sending strings to the outside world | Huy's Blog (thefullsnack.com)](https://thefullsnack.com/en/string-ffi-rust.html).



To bridge Rust string with C string, Rust provides a set of C-compatible string types: `CString` and `CStr`. They are compatible to C strings, i.e., **nul-terminated string with no nul bytes in the middle**.


### Convert string

- When convert rust `String` to C `const char*`, we must use `CString` as intermediate.

| Rust | C | to C | from C (unsafe) |
| :-: | :-: | :-: | :-: |
| `String` | `const char*` | `CString::new()` + `as_ptr()` | `std::ffi::CStr::from_ptr()` |

### Convert byte array with copy

| Rust | C | to C | from C (unsafe) |
| :-: | :-: | :-: | :-: |
| array `[i8; N]` | `char[N]` | loop and assign | loop and assign |

### Convert byte array without copy

| Rust | C | to C | from C (unsafe) |
| :-: | :-: | :-: | :-: |
| array `[i8; N]` | `const char*` | `as_ptr()` | `std::ffi::CStr::from_ptr()` |

## Polymorphism and `dyn` trait

Some C library accepts a callback and trigger it when certain event happens, the Rust side needs to:

- register a `Box<dyn Trait>` raw pointer and callback function to the C lib
- Implement the trait in custom ways

This requires the conversion from Rust `Box<dyn Trait>` to C `void*`, which is non-trivial.

### Define callback

```rust
unsafe extern "C" fn change_data(data: *mut i32, param: *const ::std::os::raw::c_void) {
    let processor = (param as *const Box<dyn Processor>).as_ref().unwrap();
    processor.process(data.as_mut().unwrap());
}
```

### Register callback

```rust
let p = MyProcessor {};
// NOTE: we must explicitly pass it as Box<dyn Trait>
let raw_p = Box::into_raw(Box::new(Box::new(p) as Box<dyn Processor>));
mylib::register_callback(&mut t as _, Some(change_data), raw_p as _);
```
Please note that the nested `Box::new`, it is necessary because the `Box<dyn Trait>` is what we want to pass to C, and itself has two fields, one to the object and one to the vtable, we can't directly use `Box::into_raw` on a `Box<dyn Trait>` because it discard information of vtable.

### Takeaways

- You must pass a pointer to `Box<dyn Trait>` (instead of `<dyn Trait>`) to C.
- You must explicitly let Rust know it's `Box<dyn Trait>`, not a normal `Box<T>`
- If the callback will be called multiple times, do not use `Box::from_raw` to convert C pointer to Rust pointer, instead, use `as_ref()` or `as_mut()`.

## Unmovable self reference type
Self reference type is unmovable and need to be carefully handled.

### Undefined behavior if a self reference type is moved

```rust
println!("channel count = {}", OesAsyncApi_GetChannelCount(async_context));
self.async_context_ = Some(*async_context);
let async_context = self.async_context_.as_mut().unwrap() as *mut OesAsyncApiContextT;
println!("channel count = {}", OesAsyncApi_GetChannelCount(async_context));
```

The code above is a roundtrip from `*mut T` to `Option<T>` and then to `*mut T`. The result is:
```
channel count = 1
channel count = -22
```
Interesting! We did not change the object, but the result changes.

This is because the object is moved when we wrap it into `Option<T>`, but the object is unmovable because it refers to itself.

```rust
pub type OesAsyncApiContextT = SEndpointContextT;
pub type SEndpointContextT = _SEndpointContext;
pub struct _SEndpointContext {
    #[doc = " 内部参考数据指针"]
    pub pInternalRefs: *mut ::std::os::raw::c_void,
    #[doc = " 按64位对齐的填充域"]
    pub __filler: *mut ::std::os::raw::c_void,
    #[doc = " 通知线程终止运行的标志变量"]
    pub terminateFlag: uint8,
    #[doc = " 按64位对齐的填充域"]
    pub __filler2: [uint8; 7usize],
}
```
The `pInternalRefs` points to itself.

```text
channel count = 1 for context at 0x7f9564000000
async context: Some(_SEndpointContext { pInternalRefs: 0x7f9564000000, __filler: 0x0, terminateFlag: 0, __filler2: [0, 0, 0, 0, 0, 0, 0] })
```

So, if we move this object, it moves to new memory address, but the `pInternalRefs` is still pointing to old address.

### Deal with unmovable types

`Box::from_raw` and `Box::into_raw` will not move this object. Do not try to deference the pointer and assign, it usually moves the object around.
