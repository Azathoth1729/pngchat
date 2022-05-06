fn main() {}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {
    #[test]
    fn str_test1() {
        let s = vec![2, 2];
        let s = std::str::from_utf8(&s).unwrap();
        println!("{}", s);
    }

    #[test]
    fn str_test2() {
        for b in "RuSt".bytes() {
            print!("{} ", b);
        }
        println!();
    }

    #[test]
    fn str_as_bytes_test() {
        let s = "bor";
        let bytes = s.as_bytes();
        println!("{:?}", bytes);
        println!("{:?}", s);
    }

    #[test]
    fn str_as_bytes_mut_test() {
        let mut s = String::from("Hello");
        let bytes = unsafe { s.as_bytes_mut() };

        assert_eq!(b"Hello", bytes);

        println!("{:?}", bytes);
        bytes[0] = b"a"[0];
        println!("{:?}", bytes);
        println!("{:?}", s);
    }
}
