/**
 * parse.rs
 * 
 * This module contains the logic for parsing Iceberg metadata files,
 * including manifest lists, manifest files, and data files.
 */
pub mod data_file;
pub mod manifest_list;
pub mod manifest;
pub mod metadata;

pub use metadata::TableMetadata;
pub use manifest_list::ManifestList;
pub use manifest::ManifestFile;
pub use data_file::DataFileRecord;