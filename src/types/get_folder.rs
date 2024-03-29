use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{BaseFolderId, Folder, FolderShape, ResponseClass};

#[derive(Debug, XmlSerialize)]
pub struct GetFolder {
    pub folder_shape: FolderShape,
    pub folder_ids: Vec<BaseFolderId>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFolderResponse {
    pub response_messages: ResponseMessages,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub get_folder_response_message: Vec<GetFolderResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFolderResponseMessage {
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,
    pub folders: Folders,
}

#[derive(Debug, Deserialize)]
pub struct Folders {
    #[serde(rename = "$value")]
    pub inner: Vec<Folder>,
}
