extern crate alloc;
use std::{fs::File, io::{BufReader, Read, Write}, path::PathBuf};

use crate::conway::Grid;
use postcard::{Error, from_bytes, to_allocvec};
use serde::{Serialize, de::DeserializeOwned};
use alloc::vec::Vec;


impl<T: Serialize + DeserializeOwned> Grid<T>{
    pub fn save(&self, path: PathBuf){
        let buf: Vec<u8> = to_allocvec(self).expect("Ser err");
        let mut file = File::create(path).expect("Can't open file");
        file.write_all(&buf).expect("Can't write file")
        // self.get_grid().get(row, col)
    }
    pub fn load(path: PathBuf) -> Result<Self, Error>{
        let buf = std::fs::read(path).unwrap();
        from_bytes(&buf)
    }
}