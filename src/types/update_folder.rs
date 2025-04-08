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
#[derive(Clone, Debug, XmlSerialize)]
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

/// A collection of changes to be performed on a folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderchange>.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct FolderChange {
    /// The folder to be updated.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid>.
    pub folder_id: BaseFolderId,

    /// The update to be performed on the folder.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-folder>.
    pub folder_update: Updates,
}

/// An operation to update a given property of a specified folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updatefolder>.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct UpdateFolder {
    pub folder_changes: Vec<FolderChange>,
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
#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub update_folder_response_message: Vec<UpdateFolderResponseMessage>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFolderResponseMessage {
    /// The status of the corresponding request, i.e. whether it succeeded or
    /// resulted in an error.
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,

    pub response_code: Option<ResponseCode>,

    pub message_text: Option<String>,
}
