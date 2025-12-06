use std::fs;

//function to read a file using the library fs
pub fn read_file(path: &str) ->Vec<u8>{
    fs::read(path).expect("Failed to read your file")
}

//function to write a file using the library fs
pub fn write_file(path: &str, data: &[u8]){
    fs::write(path, data).expect("Failed to write your file")
}