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

        /// The new value of the specified field, wrapped in an element
        /// identifying the type of the item being updated.
        #[xml_struct(flatten)]
        value: ItemChangeFieldValue,
    },
}

impl ItemChangeDescription {
    /// Creates a [`SetItemField`] update for a [`Message`] item.
    ///
    /// [`SetItemField`]: Self::SetItemField
    pub fn new_message_field(field_uri: impl Into<String>, message: Message) -> Self {
        ItemChangeDescription::SetItemField {
            field_uri: PathToElement::new_field_uri(field_uri),
            value: ItemChangeFieldValue::Message(message),
        }
    }

    /// Creates a [`SetItemField`] update for a calendar item.
    ///
    /// [`SetItemField`]: Self::SetItemField
    pub fn new_calendar_item_field(field_uri: impl Into<String>, calendar_item: Message) -> Self {
        ItemChangeDescription::SetItemField {
            field_uri: PathToElement::new_field_uri(field_uri),
            value: ItemChangeFieldValue::CalendarItem(calendar_item),
        }
    }
}

/// The new value of a field being set via [`ItemChangeDescription::SetItemField`].
///
/// EWS requires this value to be wrapped in an element matching the type of
/// the item being updated (e.g. `CalendarItem` for a calendar item), rather
/// than always using `Message`; the server rejects a request for a calendar
/// item's field wrapped in `Message` with `ErrorIncorrectUpdatePropertyCount`.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setitemfield>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(variant_ns_prefix = "t")]
pub enum ItemChangeFieldValue {
    Message(Message),
    CalendarItem(Message),
}

#[cfg(test)]
mod test {
    use quick_xml::{events::Event, Reader, Writer};
    use time::OffsetDateTime;
    use xml_struct::XmlSerialize;

    use crate::{
        test_utils::{assert_serialized_content, minify_xml},
        BaseItemId, DateTime, Message, MessageDisposition, SendMeetingInvitationsOrCancellations,
    };

    use super::{ItemChange, ItemChangeDescription, ItemChangeInner, UpdateItem, Updates};

    fn update_item_with_updates(updates: Updates) -> UpdateItem {
        UpdateItem {
            message_disposition: MessageDisposition::SaveOnly,
            conflict_resolution: None,
            send_meeting_invitations_or_cancellations: None,
            item_changes: vec![ItemChange {
                item_change: ItemChangeInner {
                    item_id: BaseItemId::ItemId {
                        id: "id".to_string(),
                        change_key: None,
                    },
                    updates,
                },
            }],
        }
    }

    fn update_item_with_change(change: ItemChangeDescription) -> UpdateItem {
        update_item_with_updates(Updates {
            inner: vec![change],
        })
    }

    /// Counts the number of direct child elements of the first element named
    /// `parent_tag` found in `xml`.
    ///
    /// Used to assert, independently of the exact XML shape, that a
    /// `SetItemField` change wraps exactly one property, per EWS's
    /// requirement (violating it produces
    /// `ErrorIncorrectUpdatePropertyCount`).
    fn count_direct_children(xml: &str, parent_tag: &str) -> usize {
        let mut reader = Reader::from_str(xml);
        let mut depth: i32 = -1;
        let mut parent_depth = None;
        let mut count = 0;

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    depth += 1;

                    if parent_depth.is_none() && e.name().as_ref() == parent_tag.as_bytes() {
                        parent_depth = Some(depth);
                    } else if parent_depth == Some(depth - 1) {
                        count += 1;
                    }
                }
                Event::Empty(e) => {
                    if parent_depth == Some(depth) {
                        count += 1;
                    } else if parent_depth.is_none() && e.name().as_ref() == parent_tag.as_bytes() {
                        // A self-closing `parent_tag` has no children.
                        return 0;
                    }
                }
                Event::End(_) => {
                    if parent_depth == Some(depth) {
                        return count;
                    }

                    depth -= 1;
                }
                Event::Eof => panic!("did not find element {parent_tag} in {xml}"),
                _ => {}
            }
        }
    }

    #[test]
    fn test_serialize_set_item_field_message() {
        let request = update_item_with_change(ItemChangeDescription::new_message_field(
            "calendar:End",
            Message::default(),
        ));

        let expected = minify_xml(
            r#"
            <UpdateItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" MessageDisposition="SaveOnly">
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

    #[test]
    fn test_serialize_set_item_field_calendar_item() {
        let request = update_item_with_change(ItemChangeDescription::new_calendar_item_field(
            "calendar:End",
            Message::default(),
        ));

        let expected = minify_xml(
            r#"
            <UpdateItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" MessageDisposition="SaveOnly">
              <ItemChanges>
                <t:ItemChange>
                  <t:ItemId Id="id"/>
                  <t:Updates>
                    <t:SetItemField>
                      <t:FieldURI FieldURI="calendar:End"/>
                      <t:CalendarItem></t:CalendarItem>
                    </t:SetItemField>
                  </t:Updates>
                </t:ItemChange>
              </ItemChanges>
            </UpdateItem>"#,
        );

        assert_serialized_content(&request, "UpdateItem", &expected);
    }

    /// Guards the EWS invariant that a `SetItemField` change may wrap only
    /// one changed property (violating it produces
    /// `ErrorIncorrectUpdatePropertyCount`).
    #[test]
    fn test_new_calendar_item_field_wraps_exactly_one_property() {
        let change = ItemChangeDescription::new_calendar_item_field(
            "calendar:End",
            Message {
                end: Some(DateTime(OffsetDateTime::UNIX_EPOCH)),
                ..Default::default()
            },
        );

        let mut writer: Writer<Vec<u8>> = Writer::new(Vec::new());
        change
            .serialize_as_element(&mut writer, "ItemChangeDescription")
            .unwrap();
        let xml = String::from_utf8(writer.into_inner()).unwrap();

        assert_eq!(
            count_direct_children(&xml, "t:CalendarItem"),
            1,
            "SetItemField must wrap exactly one property, got: {xml}"
        );
    }

    #[test]
    fn test_serialize_updates_with_multiple_set_item_field_calendar_item_changes() {
        let request = update_item_with_updates(Updates {
            inner: vec![
                ItemChangeDescription::new_calendar_item_field(
                    "calendar:End",
                    Message {
                        end: Some(DateTime(OffsetDateTime::UNIX_EPOCH)),
                        ..Default::default()
                    },
                ),
                ItemChangeDescription::new_calendar_item_field(
                    "calendar:Location",
                    Message {
                        location: Some("Room 1".to_string()),
                        ..Default::default()
                    },
                ),
            ],
        });

        let expected = minify_xml(
            r#"
            <UpdateItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" MessageDisposition="SaveOnly">
              <ItemChanges>
                <t:ItemChange>
                  <t:ItemId Id="id"/>
                  <t:Updates>
                    <t:SetItemField>
                      <t:FieldURI FieldURI="calendar:End"/>
                      <t:CalendarItem><t:End>__END_PLACEHOLDER__</t:End></t:CalendarItem>
                    </t:SetItemField>
                    <t:SetItemField>
                      <t:FieldURI FieldURI="calendar:Location"/>
                      <t:CalendarItem><t:Location>Room 1</t:Location></t:CalendarItem>
                    </t:SetItemField>
                  </t:Updates>
                </t:ItemChange>
              </ItemChanges>
            </UpdateItem>"#,
        )
        .replace(
            "__END_PLACEHOLDER__",
            &DateTime(OffsetDateTime::UNIX_EPOCH)
                .0
                .format(&time::format_description::well_known::Iso8601::DEFAULT)
                .unwrap(),
        );

        assert_serialized_content(&request, "UpdateItem", &expected);
    }

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
                        inner: vec![ItemChangeDescription::new_message_field(
                            "calendar:End",
                            Message::default(),
                        )],
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
