// impl std::iter::FromIterator<&str> for std::vec::Vec<&str> {}
#![allow(non_snake_case)]
struct InputOutput<'io_life> {
    // buffer: &'io_life Vec<&'io_life str>,
    buffer: std::str::Split<'io_life, &'io_life [char]>,
    // std::str::Split<'_, &[char]>
}
// macro_rules! parse_item {
//     ($element:expr;$tt:ty) => {
//         ($element).parse::<$tt>().unwrap()
//     };
// }
impl<'io_life> InputOutput<'io_life> {
    fn new(buffer_string: &'io_life mut String) -> Self {
        loop {
            match std::io::stdin().read_line(buffer_string) {
                Ok(io_char_found) => {
                    if io_char_found <= 0 {
                        break;
                    }
                }
                Err(_) => {
                    panic!("Refactor data as fast as possible");
                }
            };
        }
        Self {
            buffer: buffer_string.split(&['\n', ' '][..]),
        }
    }
    // fn _parse_item<T: std::str::FromStr>(&mut self) -> Result<T, <T as std::str::FromStr>::Err> {
    //     parse_item!(self.buffer.next().unwrap(); T)
    //     self.buffer.next().unwrap().parse::<T>()
    // }
    fn getData<T: std::str::FromStr>(&mut self) -> T {
        match self.buffer.next() {
            Some(data) => match data.trim().parse::<T>() {
                Ok(data) => data,
                Err(_) => {
                    println!("Parsing error.");
                    panic!()
                }
            },
            None => {
                println!("No data available");
                panic!()
            }
        }
    }
}

fn main() {
    // input output...........
    let mut buffer_string = String::new();
    let mut rw = InputOutput::new(&mut buffer_string);
    // Original code Starts from here....
    let n: i32 = rw.getData();
    if n % 2 == 0 && n > 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}