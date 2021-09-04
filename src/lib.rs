mod mylib;

#[cfg(test)]
mod tests {
    #[test]
    fn plus_one_test() {
        assert_eq!(2, unsafe { crate::mylib::plus_one(1) });
    }
}
