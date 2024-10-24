/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::types::common::{BaseItemId, PathToElement, Message, MessageDisposition};
use crate::{
    types::sealed::EnvelopeBodyContents, Items, Operation,
    OperationResponse, ResponseClass, ResponseCode,
};
use serde::Deserialize;
use xml_struct::XmlSerialize;

/// Identifies the type of conflict resolution to try during an update. The default value is AutoResolve.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#conflictresolution-attribute>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize)]
#[xml_struct(text)]
pub enum ConflictResolution {
    NeverOverwrite,
    AutoResolve,
    AlwaysOverwrite,
}

/// Represents a change to an individual item, including the item ID and updates.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange>
#[derive(Debug, Deserialize, XmlSerialize)]
pub struct ItemChange {
    #[serde(rename = "t:ItemId")]
    pub item_id: BaseItemId, // Represents the <t:ItemId> element with Id and ChangeKey.

    #[serde(rename = "t:Updates")]
    pub updates: Updates,    // Represents the <t:Updates> element containing the changes.
}

/// Represents a list of item changes without an explicit container tag.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges>
#[derive(Debug, Deserialize, XmlSerialize)]
pub struct ItemChanges {
    #[serde(rename = "t:ItemChange")]
    pub item_changes: Vec<ItemChange>,
}

/// Struct representing the field update operation.
///
/// This struct contains details of the field that needs to be updated.
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setitemfield>
#[derive(Debug, Deserialize, XmlSerialize)]
pub struct SetItemField {
    #[serde(rename = "t:FieldURI")]
    pub field_uri: PathToElement,  // Reference to the field being updated.
    #[serde(rename = "t:Message")]
    pub message: Message,          // The new value for the specified field.
}

/// Struct representing updates to be applied to an item.
///
/// This struct is used to create an UpdateItem request.
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-item>
#[derive(Debug, Deserialize, XmlSerialize)]
pub struct Updates {
    #[serde(rename = "t:SetItemField")]
    pub set_item_field: SetItemField,
}

/// Represents the UpdateItem operation for interacting with the EWS server.
#[derive(Debug, Deserialize, XmlSerialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateItem {
    /// Describes how the item will be handled after it is updated.
    /// The MessageDisposition attribute is required for message items, including meeting
    /// messages such as meeting cancellations, meeting requests, and meeting responses.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#messagedisposition-attribute>
    #[xml_struct(attribute)]
    pub message_disposition: Option<MessageDisposition>,

    /// Identifies the type of conflict resolution to try during an update.
    /// The default value is AutoResolve.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#conflictresolution-attribute>
    #[xml_struct(attribute)]
    pub conflict_resolution: Option<ConflictResolution>,

    /// Contains an array of ItemChange elements that identify items and
    /// the updates to apply to the items.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges>
    pub item_changes: ItemChanges, // Represents the list of item changes to be included in the request.
}

impl UpdateItem {

    /// Adds another `ItemChange` to the `UpdateItem` request.
    pub fn add_item_change(&mut self, item_change: ItemChange) {
        self.item_changes.item_changes.push(item_change);
    }
}

impl Operation for UpdateItem {
    type Response = UpdateItemResponse; // Define the appropriate response type if needed.
}

impl EnvelopeBodyContents for UpdateItem {
    fn name() -> &'static str {
        "UpdateItem"
    }
}

/// A response to an [`UpdateItem`] request.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitemresponse>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateItemResponse {
    pub response_messages: ResponseMessages,
}


impl OperationResponse for UpdateItemResponse {}

impl EnvelopeBodyContents for UpdateItemResponse {
    fn name() -> &'static str {
        "UpdateItemResponse"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub update_item_response_message: Vec<UpdateItemResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateItemResponseMessage {
    /// The status of the corresponding request, i.e. whether it succeeded or
    /// resulted in an error.
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,

    pub response_code: Option<ResponseCode>,

    pub message_text: Option<String>,

    pub items: Items,
}
