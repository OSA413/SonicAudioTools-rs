use chrono::{NaiveDateTime};

pub struct CriCpkEntry {
    update_date_time: NaiveDateTime,
    pub directory_name: String,
    pub name: String,
    pub id: u32,
    pub comment: String,
    pub is_compressed: bool,
    pub uncompressed_length: i64,
}
