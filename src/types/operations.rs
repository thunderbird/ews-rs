use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    get_folder::{GetFolder, GetFolderResponse},
    sync_folder_hierarchy::{SyncFolderHierarchy, SyncFolderHierarchyResponse}, MESSAGES_NS_URI,
};

#[derive(XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub enum Operation {
    GetFolder(GetFolder),
    SyncFolderHierarchy(SyncFolderHierarchy),
}

#[derive(Deserialize)]
pub enum OperationResponse {
    GetFolderResponse(GetFolderResponse),
    SyncFolderHierarchyResponse(SyncFolderHierarchyResponse),
}
