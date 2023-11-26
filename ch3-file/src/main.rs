use std::{
    fmt::{write, Display},
    vec,
};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN!!!!"),
            FileState::Closed => write!(f, "CLOSED!!!!!@@@"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }
    }
    pub fn open(&mut self) -> Result<&mut File, bool> {
        self.state = FileState::Open;
        Ok(self)
    }
    pub fn close(&mut self) -> Result<&mut File, String> {
        self.state = FileState::Closed;
        Ok(self)
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}
fn main() {
    let mut f2 = File::new("f2.txt");

    let f3 = f2.open().unwrap();
    let mut buffer: Vec<u8> = vec![];
    let f3_length = f3.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    f3.close();

    println!("{:?}", f3.state);
    println!("f3: name: {}, data: {}", f3.name, f3_length);
    println!("{}", text)
}
