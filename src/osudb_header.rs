use crate::osu_types::*;

pub struct OsudbHeader {
    pub version: Int,
    pub folder_count: Int,
    pub account_unlocked: Bool,
    pub unlocked_date: DateTime,
    pub playername: String,
    pub beatmaps_number: Int,
}

impl Default for OsudbHeader {
    fn default() -> Self {
        OsudbHeader {
            version: Default::default(),
            folder_count: Default::default(),
            account_unlocked: Default::default(),
            unlocked_date: Default::default(),
            playername: Default::default(),
            beatmaps_number: Default::default(),
        }
    }
}

impl OsudbHeader {
    pub fn version(&self) -> Int {
        self.version
    }

    pub fn folder_count(&self) -> Int {
        self.folder_count
    }

    pub fn playername(&self) -> String {
        self.playername.to_string()
    }

    pub fn beatmaps_number(&self) -> Int {
        self.beatmaps_number
    }
}
