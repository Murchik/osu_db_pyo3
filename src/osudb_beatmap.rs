use crate::osu_types::*;

pub struct BeatmapInfo {
    artist: String,         // Artist name
    atrist_unicode: String, // Artist name, in Unicode
    title: String,          // Song title
    title_unicode: String,  // Song title, in Unicode
    author: String,         // Creator name
    difficulty: String,     // Difficulty (e.g. Hard, Insane, etc.)

    audio_file: String, // Audio file name
    hash: String,       // MD5 hash of the beatmap

    osu_file: String, // Name of the .osu file corresponding to this beatmap
    status: Byte, // Ranked status (0 = unknown, 1 = unsubmitted, 2 = pending/wip/graveyard, 3 = unused, 4 = ranked, 5 = approved, 6 = qualified, 7 = loved)

    hitcicrle_num: Short,
    sliders_num: Short,
    spinners_num: Short,
    modification_time: Long, // Last modification time, Windows ticks

    ar: Single,
    cs: Single,
    hp: Single,
    od: Single,
    slider_velocity: Double,
    // sr_std: Int-Double pair*
    // sr_taiko: Int-Double pair*
    // sr_ctb: Int-Double pair*
    // sr_maina: Int-Double pair*
    drain_time: Int,   // s
    total_time: Int,   // ms
    preview_time: Int, // ms

    // timing_points_num: Timing point+
    id_difficulty: Int,
    id_beatmap: Int,
    id_thread: Int,

    grade_std: Byte,
    grade_taiko: Byte,
    grade_ctb: Byte,
    grade_mania: Byte,

    offset_local: Short,
    stack_leniency: Single, // wtf is this

    mode: Byte,
    source: String, // Song source
    tags: String,   // Song tags

    offset_online: Short,
    font: String,      // Font used for the title of the song
    is_unplayed: Bool, // Is beatmap unplayed
    played_time: Long,
    is_osz2: Bool,
    folder_name: String,

    checked_time: Long, // Last time when beatmap was checked against osu! repository

    is_ignore_sound: Bool,
    is_ignore_skin: Bool,
    is_ignore_storyboard: Bool,
    is_ignore_video: Bool,
    is_ignore_override: Bool,

    // modification_time: Int, // ???
    mania_scroll_speed: Byte,
}

impl Default for BeatmapInfo {
    fn default() -> Self {
        BeatmapInfo {
            artist: Default::default(),         // Artist name
            atrist_unicode: Default::default(), // Artist name, in Unicode
            title: Default::default(),          // Song title
            title_unicode: Default::default(),  // Song title, in Unicode
            author: Default::default(),         // Creator name
            difficulty: Default::default(),     // Difficulty (e.g. Hard, Insane, etc.)

            audio_file: Default::default(), // Audio file name
            hash: Default::default(),       // MD5 hash of the beatmap

            osu_file: Default::default(), // Name of the .osu file corresponding to this beatmap
            status: Default::default(), // Ranked status (0 = unknown, 1 = unsubmitted, 2 = pending/wip/graveyard, 3 = unused, 4 = ranked, 5 = approved, 6 = qualified, 7 = loved)

            hitcicrle_num: Default::default(),
            sliders_num: Default::default(),
            spinners_num: Default::default(),
            modification_time: Default::default(), // Last modification time, Windows ticks

            ar: Default::default(),
            cs: Default::default(),
            hp: Default::default(),
            od: Default::default(),
            slider_velocity: Default::default(),
            // sr_std: Int-Double pair*
            // sr_taiko: Int-Double pair*
            // sr_ctb: Int-Double pair*
            // sr_maina: Int-Double pair*
            drain_time: Default::default(),   // s
            total_time: Default::default(),   // ms
            preview_time: Default::default(), // ms

            // timing_points_num: Timing point+
            id_difficulty: Default::default(),
            id_beatmap: Default::default(),
            id_thread: Default::default(),

            grade_std: Default::default(),
            grade_taiko: Default::default(),
            grade_ctb: Default::default(),
            grade_mania: Default::default(),

            offset_local: Default::default(),
            stack_leniency: Default::default(), // wtf is this

            mode: Default::default(),
            source: Default::default(), // Song source
            tags: Default::default(),   // Song tags

            offset_online: Default::default(),
            font: Default::default(), // Font used for the title of the song
            is_unplayed: Default::default(), // Is beatmap unplayed
            played_time: Default::default(),
            is_osz2: Default::default(),
            folder_name: Default::default(),

            checked_time: Default::default(), // Last time when beatmap was checked against osu! repository

            is_ignore_sound: Default::default(),
            is_ignore_skin: Default::default(),
            is_ignore_storyboard: Default::default(),
            is_ignore_video: Default::default(),
            is_ignore_override: Default::default(),

            // modification_time: Int, // ???
            mania_scroll_speed: Default::default(),
        }
    }
}
