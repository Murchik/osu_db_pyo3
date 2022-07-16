#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use pyo3::ffi::vectorcallfunc;
use pyo3::prelude::*;

use std::fs;
use std::io::{BufRead, BufReader, Read};

type Byte = i8;
type Short = i16;
type Int = i32;
type Long = i64;

type Single = f32;
type Double = f64;

type Bool = u8;

type DateTime = i64;

fn read_osu_string_from_buff(buff: &Vec<u8>, mut pos: usize) -> String {
    let is_present: Bool = Bool::from_le_bytes(buff[pos..pos + 1].try_into().unwrap());
    pos += 1;
    if is_present == 0 {
        return "".to_string();
    }

    let str_len: Byte = Byte::from_le_bytes(buff[pos..pos + 1].try_into().unwrap());
    pos += 1;

    let str: String = String::from_utf8(buff[pos..pos + str_len as usize].to_vec())
        .expect("Error reading osu_string");

    return str;
}

#[pyfunction]
fn read_header(path: String) -> PyResult<String> {
    let f = fs::read(path).expect("Error reading a file");

    let version: Int = Int::from_le_bytes(f[0..4].try_into().unwrap());
    let folder_count: Int = Int::from_le_bytes(f[4..8].try_into().unwrap());
    let acc_unlocked: Bool = Bool::from_le_bytes(f[8..9].try_into().unwrap());
    let time: DateTime = DateTime::from_le_bytes(f[9..17].try_into().unwrap());
    let username = read_osu_string_from_buff(&f, 17);
    let num_of_beatmaps: Int = Int::from_le_bytes(f[32..36].try_into().unwrap());

    let result = format!(
        "osu!.db version: {}\nfolder_count: {}\naccount_unlock: {}\nunlock_date: {}\nusername: {}\nnum_of_beatmaps: {}",
        version, folder_count, acc_unlocked, time, username, num_of_beatmaps
    );
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn osu_db_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_header, m)?)?;
    Ok(())
}
