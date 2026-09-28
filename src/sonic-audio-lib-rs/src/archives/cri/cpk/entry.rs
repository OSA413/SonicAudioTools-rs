use chrono::{NaiveDateTime};

pub struct CriCpkEntry {
    pub update_date_time: NaiveDateTime,
    pub directory_name: String,
    pub name: String,
    pub id: u32,
    pub comment: String,
    pub is_compressed: bool,
    pub uncompressed_length: u32,
    pub length: u32,
    pub position: u32,
}
