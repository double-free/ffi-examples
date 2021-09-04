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
}
