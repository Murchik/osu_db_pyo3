// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use pyo3::prelude::*;

mod osu_types;
mod osudb;
mod osudb_beatmap;
mod osudb_header;
mod parser;

#[pyfunction]
fn read_header(path: String) -> PyResult<String> {
    let db = osudb::read(path);
    let h = db.header;

    let bm = db.beatmaps.get(db.beatmaps.len() - 1).unwrap();
    let s = format!(
        "version: {}\nfolder_count: {}\naccount_unlocked: {}\nunlocked_date: {}\nplayername: {}\nbeatmaps_number: {}\npermissons_level: {}",
         h.version,
         h.folder_count,
         h.account_unlocked,
         h.unlocked_date,
         h.playername,
         h.beatmaps_number,
         h.permissons_level
        );
    let s2 = format!("{} {} - {}", bm.id_beatmap, bm.artist, bm.title);
    let r = format!("{}\n\n{}", s, s2);
    Ok(r)
}

/// A Python module implemented in Rust.
#[pymodule]
fn osu_db_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_header, m)?)?;
    Ok(())
}
