/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    types::sealed::EnvelopeBodyContents, BaseItemId, ItemShape, Operation, OperationResponse,
    RealItem, ResponseClass, ResponseCode, MESSAGES_NS_URI,
};

#[derive(Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct GetItem {
    pub item_shape: ItemShape,
    pub item_ids: Vec<BaseItemId>,
}

impl Operation for GetItem {
    type Response = GetItemResponse;
}

impl EnvelopeBodyContents for GetItem {
    fn name() -> &'static str {
        "GetItem"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetItemResponse {
    pub response_messages: ResponseMessages,
}

impl OperationResponse for GetItemResponse {}

impl EnvelopeBodyContents for GetItemResponse {
    fn name() -> &'static str {
        "GetItemResponse"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub get_item_response_message: Vec<GetItemResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetItemResponseMessage {
    /// The success value of the corresponding request.
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,

    pub response_code: Option<ResponseCode>,

    pub message_text: Option<String>,

    pub items: Items,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    #[serde(rename = "$value")]
    pub inner: Vec<RealItem>,
}
