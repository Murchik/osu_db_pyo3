use crate::osudb_beatmap::BeatmapInfo;
use crate::osudb_header::OsudbHeader;
use crate::parser::Parser;

use crate::osu_types::*;

pub struct OsudbStructrure {
    pub header: OsudbHeader,
    pub beatmaps: Vec<BeatmapInfo>,
}

pub fn read(path: String) -> OsudbStructrure {
    let mut p = Parser::new();
    p.read(path);

    let mut header = parse_header(&mut p);

    let beatmaps_number = header.beatmaps_number() as usize;
    let beamaps = parse_beatmaps(&mut p, beatmaps_number);

    header.permissons_level = parse_perms_level(&mut p);

    if !p.is_finished() {
        panic!("Parser left some data unread");
    }

    OsudbStructrure {
        header: header,
        beatmaps: beamaps,
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

fn parse_beatmaps(p: &mut Parser, num: usize) -> Vec<BeatmapInfo> {
    let mut v = Vec::default();
    for _ in 0..num {
        v.push(BeatmapInfo {
            artist: p.get_string(),         // Artist name
            atrist_unicode: p.get_string(), // Artist name, in Unicode
            title: p.get_string(),          // Song title
            title_unicode: p.get_string(),  // Song title, in Unicode
            author: p.get_string(),         // Creator name
            difficulty: p.get_string(),     // Difficulty (e.g. Hard, Insane, etc.)

            audio_file: p.get_string(), // Audio file name
            hash: p.get_string(),       // MD5 hash of the beatmap

            osu_file: p.get_string(), // Name of the .osu file corresponding to this beatmap
            status: p.get_byte(), // Ranked status (0 = unknown, 1 = unsubmitted, 2 = pending/wip/graveyard, 3 = unused, 4 = ranked, 5 = approved, 6 = qualified, 7 = loved)

            hitcicrle_num: p.get_short(),
            sliders_num: p.get_short(),
            spinners_num: p.get_short(),
            modification_time: p.get_long(), // Last modification time, Windows ticks

            ar: p.get_single(),
            cs: p.get_single(),
            hp: p.get_single(),
            od: p.get_single(),

            slider_velocity: p.get_double(),

            sr_std: p.get_int_double_pairs(),
            sr_taiko: p.get_int_double_pairs(),
            sr_ctb: p.get_int_double_pairs(),
            sr_maina: p.get_int_double_pairs(),

            drain_time: p.get_int(),   // s
            total_time: p.get_int(),   // ms
            preview_time: p.get_int(), // ms

            timing_points_num: p.get_timing_points(),
            id_difficulty: p.get_int(),
            id_beatmap: p.get_int(),
            id_thread: p.get_int(),

            grade_std: p.get_byte(),
            grade_taiko: p.get_byte(),
            grade_ctb: p.get_byte(),
            grade_mania: p.get_byte(),

            offset_local: p.get_short(),
            stack_leniency: p.get_single(), // wtf is this

            mode: p.get_byte(),
            source: p.get_string(), // Song source
            tags: p.get_string(),   // Song tags

            offset_online: p.get_short(),
            font: p.get_string(),      // Font used for the title of the song
            is_unplayed: p.get_bool(), // Is beatmap unplayed
            played_time: p.get_long(),
            is_osz2: p.get_bool(),
            folder_name: p.get_string(),

            checked_time: p.get_long(), // Last time when beatmap was checked against osu! repository

            is_ignore_sound: p.get_bool(),
            is_ignore_skin: p.get_bool(),
            is_ignore_storyboard: p.get_bool(),
            is_ignore_video: p.get_bool(),
            is_ignore_override: p.get_bool(),

            unknown_int: p.get_int(), // Last modification time (?)
            mania_scroll_speed: p.get_byte(),
        })
    }
    v
}

fn parse_perms_level(p: &mut Parser) -> Int {
    p.get_int()
}
