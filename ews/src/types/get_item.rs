/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{BaseItemId, ItemShape, Items, MESSAGES_NS_URI};

/// A request for the properties of one or more Exchange items, e.g. messages,
/// calendar events, or contacts.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getitem>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(GetItemResponseMessage)]
pub struct GetItem {
    /// A description of the information to be included in the response for each
    /// item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemshape>
    pub item_shape: ItemShape,

    /// The Exchange identifiers of the items which should be fetched.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids>
    pub item_ids: Vec<BaseItemId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct GetItemResponseMessage {
    pub items: Items,
}

#[cfg(test)]
mod tests {

    use time::{Date, OffsetDateTime, Time};

    use crate::{
        test_utils::{assert_deserialized_content, assert_serialized_content},
        ArrayOfRecipients, BaseShape, Body, BodyType, DateTime, ItemId, Mailbox, Message,
        MimeContent, RealItem, Recipient, ResponseClass, ResponseMessages, Sensitivity,
    };

    use super::*;

    #[test]
    fn test_serialize_get_item() {
        let get_item = GetItem {
            item_shape: ItemShape {
                base_shape: BaseShape::Default,
                include_mime_content: Some(true),
                additional_properties: None,
            },
            item_ids: vec![BaseItemId::ItemId {
                id: "AAAlAF".to_string(),
                change_key: Some("CQAAAB".to_string()),
            }],
        };

        let expected = r#"<GetItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"><ItemShape><t:BaseShape>Default</t:BaseShape><t:IncludeMimeContent>true</t:IncludeMimeContent></ItemShape><ItemIds><t:ItemId Id="AAAlAF" ChangeKey="CQAAAB"/></ItemIds></GetItem>"#;

        assert_serialized_content(&get_item, "GetItem", expected);
    }

    #[test]
    fn test_deserialize_get_item_response_message() {
        let content = r##"<GetItemResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                             xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                             xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
              <m:ResponseMessages>
                <m:GetItemResponseMessage ResponseClass="Success">
                  <m:ResponseCode>NoError</m:ResponseCode>
                  <m:Items>
                    <t:Message>
                      <t:MimeContent CharacterSet="UTF-8">UmVjZWl</t:MimeContent>
                      <t:ItemId Id="AAAlAFVz" ChangeKey="CQAAAB" />
                      <t:Subject />
                      <t:Sensitivity>Normal</t:Sensitivity>
                      <t:Body BodyType="HTML">
                        <html dir="ltr">
                          <head>
                            <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
                              <meta content="MSHTML 6.00.3790.2759" name="GENERATOR">
                                <style title="owaParaStyle">P { MARGIN-TOP: 0px; MARGIN-BOTTOM: 0px } </style>
                              </head>
                          <body ocsi="x">
                            <div dir="ltr">
                              <font face="Tahoma" color="#000000" size="2"></font>&amp;nbsp;
                            </div>
                          </body>
                        </html>
                      </t:Body>
                      <t:Size>881</t:Size>
                      <t:DateTimeSent>2006-10-28T01:37:06Z</t:DateTimeSent>
                      <t:DateTimeCreated>2006-10-28T01:37:06Z</t:DateTimeCreated>
                      <t:ResponseObjects>
                        <t:ReplyToItem />
                        <t:ReplyAllToItem />
                        <t:ForwardItem />
                      </t:ResponseObjects>
                      <t:HasAttachments>false</t:HasAttachments>
                      <t:ToRecipients>
                        <t:Mailbox>
                          <t:Name>User1</t:Name>
                          <t:EmailAddress>User1@example.com</t:EmailAddress>
                          <t:RoutingType>SMTP</t:RoutingType>
                        </t:Mailbox>
                      </t:ToRecipients>
                      <t:IsReadReceiptRequested>false</t:IsReadReceiptRequested>
                      <t:IsDeliveryReceiptRequested>false</t:IsDeliveryReceiptRequested>
                      <t:From>
                        <t:Mailbox>
                          <t:Name>User2</t:Name>
                          <t:EmailAddress>User2@example.com</t:EmailAddress>
                          <t:RoutingType>SMTP</t:RoutingType>
                        </t:Mailbox>
                      </t:From>
                      <t:IsRead>false</t:IsRead>
                    </t:Message>
                  </m:Items>
                </m:GetItemResponseMessage>
              </m:ResponseMessages>
            </GetItemResponse>"##;

        let expected = GetItemResponse {
            response_messages: ResponseMessages {
                response_messages: vec![ResponseClass::Success(GetItemResponseMessage {
                    items: Items {
                        inner: vec![RealItem::Message(Message {
                            mime_content: Some(MimeContent {
                                character_set: Some("UTF-8".to_string()),
                                content: "UmVjZWl".to_string(),
                            }),
                            item_id: Some(ItemId {
                                id: "AAAlAFVz".to_string(),
                                change_key: Some("CQAAAB".to_string()),
                            }),
                            subject: None,
                            sensitivity: Some(Sensitivity::Normal),
                            body: Some(Body {
                                body_type: BodyType::HTML,
                                is_truncated: Some(false),
                                content: Some(
                                    r##"<html dir="ltr"><head><meta http-equiv="Content-Type" content="text/html; charset=utf-8"><meta content="MSHTML 6.00.3790.2759" name="GENERATOR"><style title="owaParaStyle">P { MARGIN-TOP: 1px; MARGIN-BOTTOM: 0px } </style></head><body ocsi="x"><div dir="ltr"><font face="Tahoma" color="#000000" size="2"></font>&amp;nbsp;</div>"##.to_string(),
                                ),
                            }),
                            attachments: None,
                            size: Some(881),
                            date_time_sent: Some(DateTime(OffsetDateTime::new_utc(Date::from_calendar_date(2006, time::Month::October, 28).unwrap(), Time::from_hms(01, 37, 06).unwrap()))),
                            date_time_created: Some(DateTime(OffsetDateTime::new_utc(Date::from_calendar_date(2006, time::Month::October, 28).unwrap(), Time::from_hms(01, 37, 06).unwrap()))),
                            has_attachments: Some(false),
                            to_recipients: Some(ArrayOfRecipients(vec![Recipient { mailbox: Mailbox { name: Some("User1".to_string()), email_address: Some("User1@example.com".to_string()), routing_type: Some("SMTP".to_string()), mailbox_type: None, item_id: None } }])),
                            is_read_receipt_requested: Some(false),
                            is_delivery_receipt_requested: Some(false),
                            from: Some(Recipient { mailbox: Mailbox { name: Some("User2".to_string()), email_address: Some("User2@example.com".to_string()), routing_type: Some("SMTP".to_string()), mailbox_type: None, item_id: None } }),
                            is_read: Some(false),
                            ..Default::default()
                        })],
                    },
                })],
            },
        };
        assert_deserialized_content(content, expected);
    }
}
