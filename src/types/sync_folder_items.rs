/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    types::sealed::EnvelopeBodyContents, BaseFolderId, ItemId, ItemShape, Operation,
    OperationResponse, RealItem, ResponseClass, MESSAGES_NS_URI,
};

#[derive(Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct SyncFolderItems {
    pub item_shape: ItemShape,
    pub sync_folder_id: BaseFolderId,
    pub sync_state: Option<String>,
    pub ignore: Option<Ignore>,
    pub max_changes_returned: u16,
    pub sync_scope: Option<SyncScope>,
}

impl Operation for SyncFolderItems {
    type Response = SyncFolderItemsResponse;
}

impl EnvelopeBodyContents for SyncFolderItems {
    fn name() -> &'static str {
        "SyncFolderItems"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncFolderItemsResponse {
    pub response_messages: ResponseMessages,
}

impl OperationResponse for SyncFolderItemsResponse {}

impl EnvelopeBodyContents for SyncFolderItemsResponse {
    fn name() -> &'static str {
        "SyncFolderItemsResponse"
    }
}

/// A collection of responses for individual entities within a request.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessages>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMessages {
    pub sync_folder_items_response_message: Vec<SyncFolderItemsResponseMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncFolderItemsResponseMessage {
    /// The success value of the corresponding request.
    #[serde(rename = "@ResponseClass")]
    pub response_class: ResponseClass,

    pub sync_state: String,

    pub includes_last_item_in_range: bool,

    pub changes: Changes,
}

#[derive(Debug, XmlSerialize)]
pub struct Ignore {
    item_id: Vec<ItemId>,
}

#[derive(Clone, Copy, Debug, XmlSerialize)]
pub enum SyncScope {
    NormalItems,
    NormalAndAssociatedItems,
}

#[derive(Debug, Deserialize)]
pub struct Changes {
    #[serde(default, rename = "$value")]
    pub inner: Vec<Change>,
}

/// A server-side change to an item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/changes-items>
#[derive(Debug, Deserialize)]
pub enum Change {
    /// A creation of an item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/create-itemsync>
    Create {
        /// The state of the item upon creation.
        #[serde(rename = "$value")]
        item: RealItem,
    },

    /// An update to an item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/update-itemsync>
    Update {
        /// The updated state of the item.
        #[serde(rename = "$value")]
        item: RealItem,
    },

    /// A deletion of an item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/delete-itemsync>
    #[serde(rename_all = "PascalCase")]
    Delete {
        /// The EWS ID for the deleted item.
        item_id: ItemId,
    },

    #[serde(rename_all = "PascalCase")]
    ReadFlagChange {
        item_id: ItemId,
        is_read: bool,
    },
}
