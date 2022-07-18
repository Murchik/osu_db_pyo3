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
        "version: {}\nfolder_count: {}\naccount_unlocked: {}\nunlocked_date: {}\nplayername: {}\nbeatmaps_number: {}",
         h.version,
         h.folder_count,
         h.account_unlocked,
         h.unlocked_date,
         h.playername,
         h.beatmaps_number,
        );
    let s2 = format!(
        "Last read beatmap: '{} {} - {}'",
        bm.id_beatmap, bm.artist, bm.title
    );

    let r = format!("{}\n\n{}", s, s2);

    Ok(r)
}

#[pymodule]
fn osu_db_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_header, m)?)?;
    Ok(())
}
