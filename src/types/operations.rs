use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    get_folder::{GetFolder, GetFolderResponse},
    sync_folder_hierarchy::{SyncFolderHierarchy, SyncFolderHierarchyResponse},
};

#[derive(XmlSerialize)]
pub enum Operation {
    GetFolder(GetFolder),
    SyncFolderHierarchy(SyncFolderHierarchy),
}

#[derive(Deserialize)]
pub enum OperationResponse {
    GetFolderResponse(GetFolderResponse),
    SyncFolderHierarchyResponse(SyncFolderHierarchyResponse),
}
