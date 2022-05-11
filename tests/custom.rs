#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

use std::fmt::Write;
use std::str;

mod std_test {
    use super::*;
    #[test]
    fn str_test1() {
        let s = vec![65, 67];
        let s = str::from_utf8(&s).unwrap();
        println!("{}", s);
    }

    #[test]
    fn str_test2() {
        println!("{:?}", "RuSt".bytes());
    }

    #[test]
    fn str_as_bytes_test() {
        let s = "This is where your secret message will be!";
        let bytes = s.as_bytes();
        println!("{:?}, length is: {}", bytes, bytes.len());
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

    #[test]
    fn vec_test() {
        let v = vec![1, 2, 3];
        let mut iter = v.into_iter();

        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn vec_test2() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let it = vec.iter();

        let it2 = it.copied();

        println!("it2: {:?}", it2);
    }

    #[test]
    fn vec_test3() {
        let v = [1, 2, 3, 4];
        println!("{:?}", &v[3..3]);
    }

    #[test]
    fn slice_test() {}

    #[test]
    fn fmt_test() {
        let mut output = String::new();
        write!(&mut output, "Hello {}!\n", "world").unwrap();
        write!(&mut output, "Hello {}!", "world").unwrap();
        println!("{}", output);
    }
}

#[test]
fn chunk_test() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let length_bytes = data_length.to_be_bytes();
    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    let crc_bytes = crc.to_be_bytes();
    assert_eq!(length_bytes, &chunk_data[0..4]);
    assert_eq!(chunk_type, &chunk_data[4..8]);
    assert_eq!(data_length, message_bytes.len() as u32);
    println!("length_bytes: {:?}", length_bytes);
    println!("chunk_type: {:?}", chunk_type);
    println!("crc_bytes: {:?}", crc_bytes);
    println!("{:?}", &chunk_data[0..4]);
    println!("{:?}", &chunk_data[chunk_data.len() - 4..chunk_data.len()]);
}
