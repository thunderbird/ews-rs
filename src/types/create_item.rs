/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    types::sealed::EnvelopeBodyContents, BaseFolderId, Items, Operation, OperationResponse,
    RealItem, ResponseClass, ResponseCode, MESSAGES_NS_URI,
};

/// Describes how an item is handled once it has been created, if it's a message
/// item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem#messagedisposition-attribute>
#[derive(Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum MessageDisposition {
    SaveOnly,
    SendOnly,
    SendAndSaveCopy,
}

/// A request to create (and optionally send) one or more Exchange item(s).
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem>
#[derive(Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct CreateItem {
    /// Describes how an item is handled once it has been created, if it's a
    /// message item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem#messagedisposition-attribute>
    #[xml_struct(attribute)]
    pub message_disposition: MessageDisposition,

    /// The folder in which to store an item once it's been created, if it's a
    /// message item.
    ///
    /// This is ignored if the message disposition is SendOnly.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/saveditemfolderid>
    pub saved_item_folder_id: Option<BaseFolderId>,

    /// The item(s) to create.
    pub items: Items,
}

impl Operation for CreateItem {
    type Response = CreateItemResponse;
}

impl EnvelopeBodyContents for CreateItem {
    fn name() -> &'static str {
        "CreateItem"
    }
}

/// A response to a [`CreateItem`] request.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitemresponse>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateItemResponse {
    pub response_messages: ResponseMessages,
}

impl OperationResponse for CreateItemResponse {}

impl EnvelopeBodyContents for CreateItemResponse {
    fn name() -> &'static str {
        "CreateItemResponse"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub create_item_response_message: Vec<CreateItemResponseResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateItemResponseResponseMessage {
    /// The status of the corresponding request, i.e. whether it succeeded or
    /// resulted in an error.
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,

    pub response_code: Option<ResponseCode>,

    pub message_text: Option<String>,

    pub items: Items,
}
