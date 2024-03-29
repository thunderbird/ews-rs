use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{BaseFolderId, Folder, FolderId, FolderShape, ResponseClass, MESSAGES_NS_URI};

#[derive(Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct SyncFolderHierarchy {
    pub folder_shape: FolderShape,
    pub sync_folder_id: Option<BaseFolderId>,
    pub sync_state: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncFolderHierarchyResponse {
    pub response_messages: ResponseMessages,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub sync_folder_hierarchy_response_message: Vec<SyncFolderHierarchyResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncFolderHierarchyResponseMessage {
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,
    pub sync_state: String,
    pub includes_last_folder_in_range: bool,
    pub changes: Changes,
}

#[derive(Debug, Deserialize)]
pub struct Changes {
    #[serde(default, rename = "$value")]
    pub inner: Vec<Change>,
}

#[derive(Debug, Deserialize)]
pub enum Change {
    Create {
        #[serde(rename = "$value")]
        folder: Folder,
    },
    Update {
        #[serde(rename = "$value")]
        folder: Folder,
    },
    Delete(FolderId),
}
