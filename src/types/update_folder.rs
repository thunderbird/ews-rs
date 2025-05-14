/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    types::sealed::EnvelopeBodyContents, BaseFolderId, Operation, OperationResponse, ResponseClass,
    ResponseCode, MESSAGES_NS_URI,
};

/// The unique identifier of an update to be performed on a folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-folder>
#[derive(Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum Updates {
    /// Not implemented in EWS API, but stll an option
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/appendtofolderfield>
    AppendToFolderField,

    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setfolderfield>
    SetFolderField,

    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deletefolderfield>
    DeleteFolderField,
}

#[derive(Debug, XmlSerialize)]
pub struct FolderChanges {
    //#[xml_struct(flatten)]
    pub folder_change: Vec<FolderChange>,
}

/// A collection of changes to be performed on a folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderchange>.
#[derive(Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct FolderChange {
    /// The folder to be updated.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid>.
    #[xml_struct(flatten)]
    pub folder_id: BaseFolderId,

    /// The update to be performed on the folder.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-folder>.
    pub updates: Updates,
}

/// An operation to update a given property of a specified folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updatefolder>.
#[derive(Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct UpdateFolder {
    pub folder_changes: FolderChanges,
}

impl Operation for UpdateFolder {
    type Response = UpdateFolderResponse;
}

impl EnvelopeBodyContents for UpdateFolder {
    fn name() -> &'static str {
        "UpdateFolder"
    }
}

/// A response to a [`UpdateFolder`] request.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updatefolderresponsemessage>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFolderResponse {
    pub response_messages: ResponseMessages,
}

impl OperationResponse for UpdateFolderResponse {}

impl EnvelopeBodyContents for UpdateFolderResponse {
    fn name() -> &'static str {
        "UpdateFolderResponse"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub update_folder_response_message: Vec<UpdateFolderResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFolderResponseMessage {
    /// The status of the corresponding request, i.e. whether it succeeded or
    /// resulted in an error.
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,

    pub response_code: Option<ResponseCode>,

    pub message_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BaseFolderId;
    use crate::Error;
    use quick_xml::Writer;

    #[test]
    fn test_serialization() {
        let folder_changes = FolderChanges {
            folder_change: vec![FolderChange {
                folder_id: BaseFolderId::FolderId {
                    id: "123".to_string(),
                    change_key: None,
                },
                updates: Updates::SetFolderField,
            }],
        };
        // Serialize into XML.
        let mut writer = {
            let inner: Vec<u8> = Default::default();
            Writer::new(inner)
        };
        folder_changes
            .serialize_as_element(&mut writer, "FolderChanges")
            .unwrap();

        // Read the contents of the `Writer`'s buffer.
        let buf = writer.into_inner();
        let actual = std::str::from_utf8(buf.as_slice())
            .map_err(|e| Error::UnexpectedResponse(e.to_string().into_bytes()))
            .unwrap();

        println!("{}", actual);
    }
}
