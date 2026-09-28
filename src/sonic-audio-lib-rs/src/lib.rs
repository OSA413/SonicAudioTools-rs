pub mod archives {
    pub mod cri {
        pub mod aax {
            pub mod archive_mode;
            pub mod archive;
            pub mod entry_flag;
            pub mod entry;
        }
        pub mod cpk {
            pub mod archive;
            pub mod mode;
            pub mod entry;
            // pub mod section;
        }
        pub mod mw {
            pub mod table {
                pub mod field_flag;
                pub mod field;
                pub mod header_encoding_type;
                pub mod header;
            }
        }
    }
}

pub mod has_flag;