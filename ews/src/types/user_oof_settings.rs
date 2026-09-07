/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The `GetUserOofSettings`/`SetUserOofSettings` operation pair, used to read
//! and write a mailbox's Automatic Replies (Out of Office/OOF) settings.
//!
//! Unlike most operations in this crate, these do not follow the usual
//! `ResponseMessages`/`ResponseClass<T>` collection shape (see
//! [`ews_proc_macros::operation_response`] and [`crate::ResponseMessages`]):
//! each response carries a single, un-enclosed `<ResponseMessage>` rather than
//! a `<ResponseMessages>` collection, and [`GetUserOofSettingsResponse`]
//! additionally carries an `<OofSettings>` element as a *sibling* of that
//! response message rather than nested inside it. Because of this, the
//! [`Operation`], [`OperationResponse`] and
//! [`EnvelopeBodyContents`](crate::types::sealed::EnvelopeBodyContents) traits
//! are implemented by hand below instead of via the `#[operation_response]`
//! macro.
//!
//! See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getuseroofsettings-operation>
//! and <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setuseroofsettings-operation>

use serde::{Deserialize, Deserializer};
use time::{Date, Month, PrimitiveDateTime, Time};
use xml_struct::XmlSerialize;

use crate::{
    types::sealed::EnvelopeBodyContents, Operation, OperationResponse, ResponseClass,
    MESSAGES_NS_URI,
};

/// A request to get a mailbox user's Automatic Replies (Out of Office/OOF)
/// settings.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getuseroofsettingsrequest>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct GetUserOofSettingsRequest {
    /// The mailbox whose OOF settings are being requested.
    #[xml_struct(ns_prefix = "t")]
    pub mailbox: OofMailbox,
}

impl Operation for GetUserOofSettingsRequest {
    type Response = GetUserOofSettingsResponse;
    const NAME: &'static str = "GetUserOofSettingsRequest";
}

impl EnvelopeBodyContents for GetUserOofSettingsRequest {
    const NAME: &'static str = "GetUserOofSettingsRequest";
}

/// A response to a [`GetUserOofSettingsRequest`] operation.
///
/// Unlike most operation responses in this crate, this does not contain a
/// `ResponseMessages` collection. Instead, it carries a single
/// `<ResponseMessage>` element and a sibling `<OofSettings>` element (plus a
/// sibling `<AllowExternalOof>` element) directly.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getuseroofsettingsresponse>
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct GetUserOofSettingsResponse {
    /// The status of the request.
    pub response_message: ResponseClass<OofResponseMessage>,

    /// The user's OOF settings, present when the request succeeded.
    pub oof_settings: Option<OofSettings>,

    /// To whom external OOF messages are sent for this mailbox.
    ///
    /// This shares its possible values with [`ExternalAudience`], per
    /// <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/allowexternaloof>.
    pub allow_external_oof: Option<ExternalAudience>,
}

impl OperationResponse for GetUserOofSettingsResponse {
    type Message = OofResponseMessage;

    fn response_messages(&self) -> &[ResponseClass<Self::Message>] {
        std::slice::from_ref(&self.response_message)
    }

    fn into_response_messages(self) -> Vec<ResponseClass<Self::Message>> {
        vec![self.response_message]
    }
}

impl EnvelopeBodyContents for GetUserOofSettingsResponse {
    const NAME: &'static str = "GetUserOofSettingsResponse";
}

/// A request to set a mailbox user's Automatic Replies (Out of Office/OOF)
/// settings.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setuseroofsettingsrequest>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct SetUserOofSettingsRequest {
    /// The mailbox whose OOF settings are being set.
    #[xml_struct(ns_prefix = "t")]
    pub mailbox: OofMailbox,

    /// The OOF settings to apply.
    ///
    /// This is serialized as `<UserOofSettings>`, per the real schema, though
    /// it is represented by the same [`OofSettings`] structure used to
    /// deserialize a [`GetUserOofSettingsResponse`]'s `<OofSettings>` element.
    #[xml_struct(ns_prefix = "t")]
    pub user_oof_settings: OofSettings,
}

impl Operation for SetUserOofSettingsRequest {
    type Response = SetUserOofSettingsResponse;
    const NAME: &'static str = "SetUserOofSettingsRequest";
}

impl EnvelopeBodyContents for SetUserOofSettingsRequest {
    const NAME: &'static str = "SetUserOofSettingsRequest";
}

/// A response to a [`SetUserOofSettingsRequest`] operation.
///
/// Unlike most operation responses in this crate, this does not contain a
/// `ResponseMessages` collection, but rather a single `<ResponseMessage>`
/// element. Unlike [`GetUserOofSettingsResponse`], no `OofSettings` is echoed
/// back.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/setuseroofsettingsresponse>
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SetUserOofSettingsResponse {
    /// The status of the request.
    pub response_message: ResponseClass<OofResponseMessage>,
}

impl OperationResponse for SetUserOofSettingsResponse {
    type Message = OofResponseMessage;

    fn response_messages(&self) -> &[ResponseClass<Self::Message>] {
        std::slice::from_ref(&self.response_message)
    }

    fn into_response_messages(self) -> Vec<ResponseClass<Self::Message>> {
        vec![self.response_message]
    }
}

impl EnvelopeBodyContents for SetUserOofSettingsResponse {
    const NAME: &'static str = "SetUserOofSettingsResponse";
}

/// The (empty) payload of a successful or warning `<ResponseMessage>` for
/// [`GetUserOofSettingsResponse`]/[`SetUserOofSettingsResponse`].
///
/// Neither operation returns any data as part of the response message itself
/// beyond the standard `ResponseCode`/`MessageText`/etc. fields already
/// modeled by [`ResponseClass`]/[`crate::response::ResponseError`]; the actual
/// OOF data (for `GetUserOofSettings`) is carried in a sibling `<OofSettings>`
/// element instead. This type exists only to satisfy [`ResponseClass<T>`]'s
/// generic parameter.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsemessage>
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct OofResponseMessage {}

/// The mailbox identifying a user for [`GetUserOofSettingsRequest`] and
/// [`SetUserOofSettingsRequest`].
///
/// This is a distinct, narrower shape from the general-purpose
/// [`Mailbox`](crate::Mailbox) type used elsewhere in this crate: it carries
/// an `Address` element rather than `EmailAddress`, and has no
/// `MailboxType`/`ItemId` fields.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailbox-availability>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct OofMailbox {
    /// The display name of the mailbox user.
    #[xml_struct(ns_prefix = "t")]
    pub name: Option<String>,

    /// The email address of the mailbox user. Required.
    #[xml_struct(ns_prefix = "t")]
    pub address: String,

    /// The routing protocol for the address, e.g. `SMTP`.
    #[xml_struct(ns_prefix = "t")]
    pub routing_type: Option<String>,
}

/// The Out of Office (OOF) settings for a mailbox.
///
/// This structure is shared between [`GetUserOofSettingsResponse`] (where it
/// is serialized as `<OofSettings>`) and [`SetUserOofSettingsRequest`] (where
/// the very same shape is serialized as `<UserOofSettings>`); the element name
/// used comes from the containing field, not from this type.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/oofsettings>
/// and <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/useroofsettings>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct OofSettings {
    /// The user's OOF state.
    #[xml_struct(ns_prefix = "t")]
    pub oof_state: OofState,

    /// To whom external OOF messages are sent.
    #[xml_struct(ns_prefix = "t")]
    pub external_audience: ExternalAudience,

    /// The window during which OOF is active, meaningful only when
    /// `oof_state` is [`OofState::Scheduled`].
    ///
    /// This element is optional on the wire, but MUST be present when calling
    /// `SetUserOofSettings` with `oof_state` set to [`OofState::Scheduled`].
    #[xml_struct(ns_prefix = "t")]
    pub duration: Option<Duration>,

    /// The automatic reply sent to senders inside the user's organization.
    #[xml_struct(ns_prefix = "t")]
    pub internal_reply: Option<ReplyBody>,

    /// The automatic reply sent to senders outside the user's organization.
    #[xml_struct(ns_prefix = "t")]
    pub external_reply: Option<ReplyBody>,
}

/// A mailbox user's Out of Office (OOF) state.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/oofstate>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum OofState {
    Disabled,
    Enabled,
    Scheduled,
}

/// Determines which external senders receive an OOF response.
///
/// This same type is used for the [`AllowExternalOof`] element seen in
/// [`GetUserOofSettingsResponse`].
///
/// [`AllowExternalOof`]: https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/allowexternaloof
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/externalaudience>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum ExternalAudience {
    None,
    Known,
    All,
}

/// The window of time for which a [`Scheduled`](OofState::Scheduled) OOF
/// status is active.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/duration-useroofsettings>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Duration {
    /// The start of the OOF period.
    #[xml_struct(ns_prefix = "t")]
    pub start_time: OofDateTime,

    /// The end of the OOF period.
    #[xml_struct(ns_prefix = "t")]
    pub end_time: OofDateTime,
}

/// The automatic reply message sent to a class of senders (internal or
/// external).
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/internalreply>
/// and <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/externalreply>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ReplyBody {
    /// The text of the automatic reply.
    ///
    /// Per <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-availability>,
    /// this is a plain string value, not HTML markup.
    #[xml_struct(ns_prefix = "t")]
    pub message: Option<String>,
}

/// An `xs:dateTime` value as used by [`Duration`]'s `StartTime`/`EndTime`.
///
/// Unlike [`DateTime`](crate::DateTime) used elsewhere in this crate, these
/// values carry no UTC offset on the wire (e.g. `2005-10-05T00:00:00`, per
/// Microsoft's documented examples). Per `MS-OXWOOF`, they SHOULD represent
/// UTC, but nothing in the value itself says so; callers are responsible for
/// any conversion to/from the mailbox's local time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OofDateTime(pub PrimitiveDateTime);

impl OofDateTime {
    /// Parses an EWS `xs:dateTime` string of the form
    /// `YYYY-MM-DDTHH:MM:SS`, as used by the `Duration` element.
    ///
    /// Any trailing content beyond the seconds component (e.g. fractional
    /// seconds or a UTC offset, neither of which are part of the documented
    /// `Duration` schema, but which a nonconformant server might still send)
    /// is tolerated but ignored.
    fn parse(value: &str) -> Result<Self, String> {
        let prefix = value
            .get(0..19)
            .ok_or_else(|| format!("OOF dateTime `{value}` is too short"))?;
        let (date_part, time_part) = prefix.split_once('T').ok_or_else(|| {
            format!("OOF dateTime `{value}` is missing the `T` date/time separator")
        })?;

        let mut date_fields = date_part.split('-');
        let year: i32 = next_field(&mut date_fields, "year", value)?;
        let month: u8 = next_field(&mut date_fields, "month", value)?;
        let day: u8 = next_field(&mut date_fields, "day", value)?;

        let mut time_fields = time_part.split(':');
        let hour: u8 = next_field(&mut time_fields, "hour", value)?;
        let minute: u8 = next_field(&mut time_fields, "minute", value)?;
        let second: u8 = next_field(&mut time_fields, "second", value)?;

        let month = Month::try_from(month).map_err(|err| err.to_string())?;
        let date = Date::from_calendar_date(year, month, day).map_err(|err| err.to_string())?;
        let time = Time::from_hms(hour, minute, second).map_err(|err| err.to_string())?;

        Ok(Self(PrimitiveDateTime::new(date, time)))
    }
}

/// Parses the next `.`-delimited field of an OOF date/time component.
fn next_field<'a, T>(
    fields: &mut impl Iterator<Item = &'a str>,
    name: &str,
    original: &str,
) -> Result<T, String>
where
    T: std::str::FromStr,
{
    fields
        .next()
        .ok_or_else(|| format!("OOF dateTime `{original}` is missing its {name} component"))?
        .parse()
        .map_err(|_| format!("OOF dateTime `{original}` has an invalid {name} component"))
}

impl<'de> Deserialize<'de> for OofDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

impl XmlSerialize for OofDateTime {
    fn serialize_child_nodes<W>(
        &self,
        writer: &mut quick_xml::Writer<W>,
    ) -> Result<(), xml_struct::Error>
    where
        W: std::io::Write,
    {
        let formatted = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.0.year(),
            u8::from(self.0.month()),
            self.0.day(),
            self.0.hour(),
            self.0.minute(),
            self.0.second(),
        );

        formatted.serialize_child_nodes(writer)
    }
}

#[cfg(test)]
mod test {
    use time::{Date, Month, PrimitiveDateTime, Time};

    use crate::{
        test_utils::{assert_deserialized_content, assert_serialized_content, minify_xml},
        ResponseClass,
    };

    use super::{
        Duration, ExternalAudience, GetUserOofSettingsRequest, GetUserOofSettingsResponse,
        OofDateTime, OofMailbox, OofResponseMessage, OofSettings, OofState, ReplyBody,
        SetUserOofSettingsRequest, SetUserOofSettingsResponse,
    };

    fn oof_date_time(
        year: i32,
        month: Month,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> OofDateTime {
        OofDateTime(PrimitiveDateTime::new(
            Date::from_calendar_date(year, month, day).unwrap(),
            Time::from_hms(hour, minute, second).unwrap(),
        ))
    }

    fn mailbox() -> OofMailbox {
        OofMailbox {
            name: Some("David Alexander".to_string()),
            address: "someone@example.com".to_string(),
            routing_type: Some("SMTP".to_string()),
        }
    }

    // --- GetUserOofSettings ---

    #[test]
    fn test_serialize_get_user_oof_settings_request() {
        let request = GetUserOofSettingsRequest { mailbox: mailbox() };

        let expected = minify_xml(
            r#"
            <GetUserOofSettingsRequest xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
              <t:Mailbox>
                <t:Name>David Alexander</t:Name>
                <t:Address>someone@example.com</t:Address>
                <t:RoutingType>SMTP</t:RoutingType>
              </t:Mailbox>
            </GetUserOofSettingsRequest>"#,
        );

        assert_serialized_content(&request, "GetUserOofSettingsRequest", &expected);
    }

    #[test]
    fn test_deserialize_get_user_oof_settings_response_disabled() {
        // `OofState::Disabled` is the simplest shape: no `Duration` and no
        // reply messages are meaningful (though a server may still omit or
        // include empty ones; here we test omission).
        let content = r#"
            <GetUserOofSettingsResponse
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
                xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
              <ResponseMessage ResponseClass="Success">
                <ResponseCode>NoError</ResponseCode>
              </ResponseMessage>
              <OofSettings>
                <OofState>Disabled</OofState>
                <ExternalAudience>None</ExternalAudience>
              </OofSettings>
              <AllowExternalOof>None</AllowExternalOof>
            </GetUserOofSettingsResponse>"#;

        let expected = GetUserOofSettingsResponse {
            response_message: ResponseClass::Success(OofResponseMessage {}),
            oof_settings: Some(OofSettings {
                oof_state: OofState::Disabled,
                external_audience: ExternalAudience::None,
                duration: None,
                internal_reply: None,
                external_reply: None,
            }),
            allow_external_oof: Some(ExternalAudience::None),
        };

        assert_deserialized_content(content, expected);
    }

    #[test]
    fn test_deserialize_get_user_oof_settings_response_enabled() {
        let content = r#"
            <GetUserOofSettingsResponse
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
                xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
              <ResponseMessage ResponseClass="Success">
                <ResponseCode>NoError</ResponseCode>
              </ResponseMessage>
              <OofSettings>
                <OofState>Enabled</OofState>
                <ExternalAudience>All</ExternalAudience>
                <InternalReply>
                  <Message>I am out of office. This is my internal reply.</Message>
                </InternalReply>
                <ExternalReply>
                  <Message>I am out of office. This is my external reply.</Message>
                </ExternalReply>
              </OofSettings>
              <AllowExternalOof>All</AllowExternalOof>
            </GetUserOofSettingsResponse>"#;

        let expected = GetUserOofSettingsResponse {
            response_message: ResponseClass::Success(OofResponseMessage {}),
            oof_settings: Some(OofSettings {
                oof_state: OofState::Enabled,
                external_audience: ExternalAudience::All,
                duration: None,
                internal_reply: Some(ReplyBody {
                    message: Some("I am out of office. This is my internal reply.".to_string()),
                }),
                external_reply: Some(ReplyBody {
                    message: Some("I am out of office. This is my external reply.".to_string()),
                }),
            }),
            allow_external_oof: Some(ExternalAudience::All),
        };

        assert_deserialized_content(content, expected);
    }

    #[test]
    fn test_deserialize_get_user_oof_settings_response_scheduled() {
        // `OofState::Scheduled` is the shape most likely to have a subtle
        // bug, since it's the only one that nests a `Duration` element.
        // This XML is based on the real-life example in MS-OXWOOF's
        // GetUserOofSettings Response documentation.
        let content = r#"
            <GetUserOofSettingsResponse
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"
                xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
              <ResponseMessage ResponseClass="Success">
                <ResponseCode>NoError</ResponseCode>
              </ResponseMessage>
              <OofSettings>
                <OofState>Scheduled</OofState>
                <ExternalAudience>Known</ExternalAudience>
                <Duration>
                  <StartTime>2008-02-01T00:00:00</StartTime>
                  <EndTime>2008-02-02T00:00:00</EndTime>
                </Duration>
                <InternalReply>
                  <Message>I am out of office. This is my internal reply.</Message>
                </InternalReply>
                <ExternalReply>
                  <Message>I am out of office. This is my external reply.</Message>
                </ExternalReply>
              </OofSettings>
              <AllowExternalOof>Known</AllowExternalOof>
            </GetUserOofSettingsResponse>"#;

        let expected = GetUserOofSettingsResponse {
            response_message: ResponseClass::Success(OofResponseMessage {}),
            oof_settings: Some(OofSettings {
                oof_state: OofState::Scheduled,
                external_audience: ExternalAudience::Known,
                duration: Some(Duration {
                    start_time: oof_date_time(2008, Month::February, 1, 0, 0, 0),
                    end_time: oof_date_time(2008, Month::February, 2, 0, 0, 0),
                }),
                internal_reply: Some(ReplyBody {
                    message: Some("I am out of office. This is my internal reply.".to_string()),
                }),
                external_reply: Some(ReplyBody {
                    message: Some("I am out of office. This is my external reply.".to_string()),
                }),
            }),
            allow_external_oof: Some(ExternalAudience::Known),
        };

        assert_deserialized_content(content, expected);
    }

    // --- SetUserOofSettings ---

    #[test]
    fn test_serialize_set_user_oof_settings_request_disabled() {
        let request = SetUserOofSettingsRequest {
            mailbox: mailbox(),
            user_oof_settings: OofSettings {
                oof_state: OofState::Disabled,
                external_audience: ExternalAudience::None,
                duration: None,
                internal_reply: None,
                external_reply: None,
            },
        };

        let expected = minify_xml(
            r#"
            <SetUserOofSettingsRequest xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
              <t:Mailbox>
                <t:Name>David Alexander</t:Name>
                <t:Address>someone@example.com</t:Address>
                <t:RoutingType>SMTP</t:RoutingType>
              </t:Mailbox>
              <t:UserOofSettings>
                <t:OofState>Disabled</t:OofState>
                <t:ExternalAudience>None</t:ExternalAudience>
              </t:UserOofSettings>
            </SetUserOofSettingsRequest>"#,
        );

        assert_serialized_content(&request, "SetUserOofSettingsRequest", &expected);
    }

    #[test]
    fn test_serialize_set_user_oof_settings_request_enabled() {
        let request = SetUserOofSettingsRequest {
            mailbox: mailbox(),
            user_oof_settings: OofSettings {
                oof_state: OofState::Enabled,
                external_audience: ExternalAudience::All,
                duration: None,
                internal_reply: Some(ReplyBody {
                    message: Some("I am out of office.  This is my internal reply.".to_string()),
                }),
                external_reply: Some(ReplyBody {
                    message: Some("I am out of office. This is my external reply.".to_string()),
                }),
            },
        };

        let expected = minify_xml(
            r#"
            <SetUserOofSettingsRequest xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
              <t:Mailbox>
                <t:Name>David Alexander</t:Name>
                <t:Address>someone@example.com</t:Address>
                <t:RoutingType>SMTP</t:RoutingType>
              </t:Mailbox>
              <t:UserOofSettings>
                <t:OofState>Enabled</t:OofState>
                <t:ExternalAudience>All</t:ExternalAudience>
                <t:InternalReply>
                  <t:Message>I am out of office.  This is my internal reply.</t:Message>
                </t:InternalReply>
                <t:ExternalReply>
                  <t:Message>I am out of office. This is my external reply.</t:Message>
                </t:ExternalReply>
              </t:UserOofSettings>
            </SetUserOofSettingsRequest>"#,
        );

        assert_serialized_content(&request, "SetUserOofSettingsRequest", &expected);
    }

    #[test]
    fn test_serialize_set_user_oof_settings_request_scheduled() {
        // This XML is based on Microsoft's documented example for a
        // `SetUserOofSettings` request scheduling OOF for 10 days, with both
        // internal and external replies set.
        let request = SetUserOofSettingsRequest {
            mailbox: mailbox(),
            user_oof_settings: OofSettings {
                oof_state: OofState::Scheduled,
                external_audience: ExternalAudience::All,
                duration: Some(Duration {
                    start_time: oof_date_time(2005, Month::October, 5, 0, 0, 0),
                    end_time: oof_date_time(2005, Month::October, 25, 0, 0, 0),
                }),
                internal_reply: Some(ReplyBody {
                    message: Some("I am out of office.  This is my internal reply.".to_string()),
                }),
                external_reply: Some(ReplyBody {
                    message: Some("I am out of office. This is my external reply.".to_string()),
                }),
            },
        };

        let expected = minify_xml(
            r#"
            <SetUserOofSettingsRequest xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
              <t:Mailbox>
                <t:Name>David Alexander</t:Name>
                <t:Address>someone@example.com</t:Address>
                <t:RoutingType>SMTP</t:RoutingType>
              </t:Mailbox>
              <t:UserOofSettings>
                <t:OofState>Scheduled</t:OofState>
                <t:ExternalAudience>All</t:ExternalAudience>
                <t:Duration>
                  <t:StartTime>2005-10-05T00:00:00</t:StartTime>
                  <t:EndTime>2005-10-25T00:00:00</t:EndTime>
                </t:Duration>
                <t:InternalReply>
                  <t:Message>I am out of office.  This is my internal reply.</t:Message>
                </t:InternalReply>
                <t:ExternalReply>
                  <t:Message>I am out of office. This is my external reply.</t:Message>
                </t:ExternalReply>
              </t:UserOofSettings>
            </SetUserOofSettingsRequest>"#,
        );

        assert_serialized_content(&request, "SetUserOofSettingsRequest", &expected);
    }

    #[test]
    fn test_deserialize_set_user_oof_settings_response() {
        let content = r#"
            <SetUserOofSettingsResponse
                xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
              <ResponseMessage ResponseClass="Success">
                <ResponseCode>NoError</ResponseCode>
              </ResponseMessage>
            </SetUserOofSettingsResponse>"#;

        let expected = SetUserOofSettingsResponse {
            response_message: ResponseClass::Success(OofResponseMessage {}),
        };

        assert_deserialized_content(content, expected);
    }
}
