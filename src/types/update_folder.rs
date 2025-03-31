/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    types::sealed::EnvelopeBodyContents, BaseFolderId, DeleteType, Operation, OperationResponse,
    ResponseClass, ResponseCode, MESSAGES_NS_URI,
};

/// A request to rename an existing folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updatefolder-operation>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct UpdateFolder {
    /// The folder to rename.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid>
    pub folder_id: BaseFolderId,
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
