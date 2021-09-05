mod mylib;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn pointers_test() {
        unsafe {
            let mut t: mylib::T = mem::zeroed();

            // const
            assert_eq!(0, mylib::read_int(&t as _));

            // mutable
            mylib::plus_one(&mut t as _);
            assert_eq!(1, t.x);

            // do not crash when pass nullptr
            let nullptr = std::ptr::null_mut::<mylib::T>();
            mylib::plus_one(nullptr);
        }
    }

    #[test]
    fn string_to_c_char_test() {
        unsafe {
            // to c_char array
            let s = String::from("abc");
            let c_string = std::ffi::CString::new(s).unwrap();
            let mut t: mylib::T = mem::zeroed();
            // assign one by one
            for (i, c) in c_string.as_bytes_with_nul().iter().enumerate() {
                t.name[i] = *c as i8;
            }
            // TODO: any approach that does not require a copy?

            // from c_char array (take ownership)
            let name = std::ffi::CStr::from_ptr(t.name.as_ptr()).to_str().unwrap();
            assert_eq!(name, "abc");
        }
    }
}
