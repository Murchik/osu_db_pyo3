use crate::osu_types::*;

pub struct BeatmapInfo {
    pub artist: String,         // Artist name
    pub atrist_unicode: String, // Artist name, in Unicode
    pub title: String,          // Song title
    pub title_unicode: String,  // Song title, in Unicode
    pub author: String,         // Creator name
    pub difficulty: String,     // Difficulty (e.g. Hard, Insane, etc.)

    pub audio_file: String, // Audio file name
    pub hash: String,       // MD5 hash of the beatmap

    pub osu_file: String, // Name of the .osu file corresponding to this beatmap
    pub status: Byte, // Ranked status (0 = unknown, 1 = unsubmitted, 2 = pending/wip/graveyard, 3 = unused, 4 = ranked, 5 = approved, 6 = qualified, 7 = loved)

    pub hitcicrle_num: Short,
    pub sliders_num: Short,
    pub spinners_num: Short,
    pub modification_time: Long, // Last modification time, Windows ticks

    pub ar: Single,
    pub cs: Single,
    pub hp: Single,
    pub od: Single,
    pub slider_velocity: Double,

    pub sr_std: Vec<IntDoublePair>,
    pub sr_taiko: Vec<IntDoublePair>,
    pub sr_ctb: Vec<IntDoublePair>,
    pub sr_maina: Vec<IntDoublePair>,

    pub drain_time: Int,   // s
    pub total_time: Int,   // ms
    pub preview_time: Int, // ms

    pub timing_points_num: Vec<TimingPoint>,
    pub id_difficulty: Int,
    pub id_beatmap: Int,
    pub id_thread: Int,

    pub grade_std: Byte,
    pub grade_taiko: Byte,
    pub grade_ctb: Byte,
    pub grade_mania: Byte,

    pub offset_local: Short,
    pub stack_leniency: Single, // wtf is this

    pub mode: Byte,
    pub source: String, // Song source
    pub tags: String,   // Song tags

    pub offset_online: Short,
    pub font: String,      // Font used for the title of the song
    pub is_unplayed: Bool, // Is beatmap unplayed
    pub played_time: Long,
    pub is_osz2: Bool,
    pub folder_name: String,

    pub checked_time: Long, // Last time when beatmap was checked against osu! repository

    pub is_ignore_sound: Bool,
    pub is_ignore_skin: Bool,
    pub is_ignore_storyboard: Bool,
    pub is_ignore_video: Bool,
    pub is_ignore_override: Bool,

    pub unknown_int: Int, // Last modification time (?)
    pub mania_scroll_speed: Byte,
}
