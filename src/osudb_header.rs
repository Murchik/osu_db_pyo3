use crate::osu_types::*;

pub struct OsudbHeader {
    pub version: Int,
    pub folder_count: Int,
    pub account_unlocked: Bool,
    pub unlocked_date: DateTime,
    pub playername: String,
    pub beatmaps_number: Int,
    pub permissons_level: Int,
}

impl Default for OsudbHeader {
    fn default() -> Self {
        OsudbHeader {
            version: 0,
            folder_count: 0,
            account_unlocked: 0,
            unlocked_date: 0,
            playername: "".to_string(),
            beatmaps_number: 0,
            permissons_level: 0,
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
