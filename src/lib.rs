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

    #[test]
    fn polymorphism_test() {
        trait Processor {
            fn process(&self, data: &mut i32);
        }
        struct MyProcessor {}

        impl Processor for MyProcessor {
            fn process(&self, data: &mut i32) {
                *data *= 2;
            }
        }

        // callback function for C
        unsafe extern "C" fn change_data(data: *mut i32, param: *const ::std::os::raw::c_void) {
            let processor = (param as *const Box<dyn Processor>).as_ref().unwrap();
            processor.process(data.as_mut().unwrap());
        }
        unsafe {
            let mut t: mylib::T = mem::zeroed();
            t.x = 3;

            let p = MyProcessor {};
            // NOTE: we must explicitly pass it as Box<dyn Trait>
            let raw_p = Box::into_raw(Box::new(Box::new(p) as Box<dyn Processor>));
            mylib::register_callback(&mut t as _, Some(change_data), raw_p as _);
            mylib::trigger_callback(&mut t as _);

            // collect memory
            Box::from_raw(raw_p);
            assert_eq!(6, t.x);
        }
    }
}
