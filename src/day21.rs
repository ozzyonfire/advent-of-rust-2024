use std::{
    env,
    fs::{self, remove_file, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(_) = remove_file(&self.file_path) {
            println!("The file was already removed.");
        }
    }
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut temp_dir = env::temp_dir();
        let file_name = format!("{}.log", timestamp);
        fs::create_dir_all("./tmp")?;
        temp_dir.push(file_name);

        let file = fs::File::create(&temp_dir)?;

        Ok(Self {
            file_path: temp_dir,
            file,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write_all(data)?;
        Ok(())
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

fn main() {
    let mut temp = TempFile::new().unwrap();
    temp.write("Hello world!".as_bytes()).unwrap();
    let data = temp.read_to_string();
    match data {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error reading data, {}", e),
    }
}
