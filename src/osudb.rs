use crate::osudb_beatmap::BeatmapInfo;
use crate::osudb_header::OsudbHeader;
use crate::parser::Parser;

pub struct OsudbStructrure {
    pub header: OsudbHeader,
    beatmaps: Vec<BeatmapInfo>,
}

pub fn read(path: String) -> OsudbStructrure {
    let mut p = Parser::new();
    p.read(path);

    let header = parse_header(&mut p);

    let beatmaps_number = header.beatmaps_number() as usize;
    let beatmaps = parse_beatmaps(&mut p, beatmaps_number);

    OsudbStructrure {
        header: header,
        beatmaps: Default::default(),
    }
}

fn parse_header(p: &mut Parser) -> OsudbHeader {
    OsudbHeader {
        version: p.get_int(),
        folder_count: p.get_int(),
        account_unlocked: p.get_bool(),
        unlocked_date: p.get_datetime(),
        playername: p.get_string(),
        beatmaps_number: p.get_int(),
        permissons_level: Default::default(),
    }
}

pub fn parse_beatmaps(p: &mut Parser, num: usize) -> Vec<BeatmapInfo> {
    Vec::default()
}
