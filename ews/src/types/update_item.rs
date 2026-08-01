/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::types::common::{
    BaseItemId, Message, MessageDisposition, PathToElement, SendMeetingInvitationsOrCancellations,
};
use crate::{Items, MESSAGES_NS_URI};

/// A request to update properties of one or more Exchange items.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(UpdateItemResponseMessage)]
pub struct UpdateItem {
    /// The action the Exchange server will take upon updating this item.
    ///
    /// This field is required for and only applicable to [`Message`] items.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#messagedisposition-attribute>
    #[xml_struct(attribute)]
    pub message_disposition: MessageDisposition,

    /// The method the Exchange server will use to resolve conflicts between
    /// updates.
    ///
    /// If omitted, the server will default to [`AutoResolve`].
    ///
    /// [`AutoResolve`]: `ConflictResolution::AutoResolve`
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#conflictresolution-attribute>
    #[xml_struct(attribute)]
    pub conflict_resolution: Option<ConflictResolution>,

    /// Whether/how meeting invitations or cancellations are sent to
    /// attendees.
    ///
    /// Required when updating calendar items, otherwise it has no effect.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#sendmeetinginvitationsorcancellations-attribute>
    #[xml_struct(attribute)]
    pub send_meeting_invitations_or_cancellations: Option<SendMeetingInvitationsOrCancellations>,

    /// A list of items and their corresponding updates.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchanges>
    pub item_changes: Vec<ItemChange>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateItemResponseMessage {
    pub items: Items,
}

/// The method used by the Exchange server to resolve conflicts between item
/// updates.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#conflictresolution-attribute>
#[derive(Clone, Copy, Debug, Default, XmlSerialize)]
#[xml_struct(text)]
pub enum ConflictResolution {
    /// Conflicts will cause the update to fail and return an error.
    NeverOverwrite,

    /// The Exchange server will attempt to resolve any conflicts automatically.
    #[default]
    AutoResolve,

    /// Conflicting fields will be overwritten with the contents of the update.
    AlwaysOverwrite,
}

#[derive(Clone, Debug, XmlSerialize)]
pub struct ItemChange {
    #[xml_struct(ns_prefix = "t")]
    pub item_change: ItemChangeInner,
}

/// One or more updates to a single item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemchange>
#[derive(Clone, Debug, XmlSerialize)]
pub struct ItemChangeInner {
    /// The ID of the item to be updated.
    #[xml_struct(flatten, ns_prefix = "t")]
    pub item_id: BaseItemId,

    /// The changes to make to the item, including appending, setting, or
    /// deleting fields.
    #[xml_struct(ns_prefix = "t")]
    pub updates: Updates,
}

/// A list of changes to fields, with each element representing a single change.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updates-item>
#[derive(Clone, Debug, XmlSerialize)]
pub struct Updates {
    #[xml_struct(flatten, ns_prefix = "t")]
    pub inner: Vec<ItemChangeDescription>,
}

/// An individual change to a single field.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(variant_ns_prefix = "t")]
pub enum ItemChangeDescription {
    /// An update setting the value of a single field.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setitemfield>
    SetItemField {
        /// The field to be updated.
        #[xml_struct(flatten, ns_prefix = "t")]
        field_uri: PathToElement,

        /// The new value of the specified field.
        #[xml_struct(ns_prefix = "t")]
        message: Message,
    },
}

#[cfg(test)]
mod test {
    use crate::{
        test_utils::{assert_serialized_content, minify_xml},
        BaseItemId, MessageDisposition, PathToElement, SendMeetingInvitationsOrCancellations,
    };

    use super::{ItemChange, ItemChangeDescription, ItemChangeInner, UpdateItem, Updates};

    #[test]
    fn test_serialize_update_item_send_meeting_invitations_or_cancellations() {
        let request = UpdateItem {
            message_disposition: MessageDisposition::SaveOnly,
            conflict_resolution: None,
            send_meeting_invitations_or_cancellations: Some(
                SendMeetingInvitationsOrCancellations::SendToNone,
            ),
            item_changes: vec![ItemChange {
                item_change: ItemChangeInner {
                    item_id: BaseItemId::ItemId {
                        id: "id".to_string(),
                        change_key: None,
                    },
                    updates: Updates {
                        inner: vec![ItemChangeDescription::SetItemField {
                            field_uri: PathToElement::FieldURI {
                                field_URI: "calendar:End".to_string(),
                            },
                            message: Default::default(),
                        }],
                    },
                },
            }],
        };

        let expected = minify_xml(
            r#"
            <UpdateItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" MessageDisposition="SaveOnly" SendMeetingInvitationsOrCancellations="SendToNone">
              <ItemChanges>
                <t:ItemChange>
                  <t:ItemId Id="id"/>
                  <t:Updates>
                    <t:SetItemField>
                      <t:FieldURI FieldURI="calendar:End"/>
                      <t:Message></t:Message>
                    </t:SetItemField>
                  </t:Updates>
                </t:ItemChange>
              </ItemChanges>
            </UpdateItem>"#,
        );

        assert_serialized_content(&request, "UpdateItem", &expected);
    }
}
