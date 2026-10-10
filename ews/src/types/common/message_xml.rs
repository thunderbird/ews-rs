/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;

/// Semi-structured data for diagnosing or responding to an EWS error.
///
/// Because the possible contents of this field are not documented, this may not
/// precisely represent all potential messages. Known fields that are relevant
/// for programmatic error responses should be provided as additional variants
/// of this enum. All other responses are represented via the `Other` variant.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/messagexml>
// There appears to be two observed structurings of MessageXml elements:
// - one or more Value elements differentiated by Name attribute: <t:Value Name="Foo">value</t:Value>
// - one or more directly tagged elements: <t:Foo>value</t:Foo>
// In all observed samples, these are never mixed, and are only one layer
// deep. Because we have to distinguish types based on the tag *and* an
// attribute, most new variants will require manual Deserialize implementations,
// though quick_xml's impl_deserialize_for_internally_tagged_enum may work for
// this one day.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum MessageXml {
    ServerBusy(ServerBusy),
    /// Any elements not handled above, for debugging and troubleshooting purposes.
    // `Other` *must* come last, since it will match any single-layer XML.
    Other(MessageXmlElements),
}

/// One of the two observed kinds of MessageXml elements: a Value named via the @Name attribute.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct MessageXmlValue {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: String,
}

/// One of the two observed kinds of MessageXml elements: a tag with a single text value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageXmlTagged {
    pub name: String,
    pub value: String,
}

/// A tag with one or more XML attributes and no text content, e.g.
/// `<t:FieldURI FieldURI="calendar:IsMeeting"/>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageXmlAttributed {
    pub name: String,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageXmlElements {
    pub elements: Vec<MessageXmlElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageXmlElement {
    MessageXmlTagged(MessageXmlTagged),
    MessageXmlValue(MessageXmlValue),
    MessageXmlAttributed(MessageXmlAttributed),
}

/// Data associated with a [`ResponseCode::ErrorServerBusy`](crate::response::ResponseCode::ErrorServerBusy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerBusy {
    /// The duration in milliseconds to wait before making additional requests.
    pub back_off_milliseconds: u32,
}

struct ServerBusyVisitor {
    marker: PhantomData<fn() -> ServerBusy>,
}

impl<'de> Visitor<'de> for ServerBusyVisitor {
    type Value = ServerBusy;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Value element with Name=BackOffMilliseconds and an integer value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        while let Some(name) = access.next_key::<String>()? {
            if name.as_str() != "Value" {
                continue;
            }
            let value = access.next_value::<MessageXmlValue>()?;
            if value.name.as_str() == "BackOffMilliseconds" {
                let ms = value.value.parse::<u32>().map_err(de::Error::custom)?;
                return Ok(ServerBusy {
                    back_off_milliseconds: ms,
                });
            }
        }

        Err(de::Error::custom("no BackOffMilliseconds field"))
    }
}

impl<'de> Deserialize<'de> for ServerBusy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ServerBusyVisitor {
            marker: PhantomData,
        })
    }
}

struct MessageXmlElementsVisitor {
    marker: PhantomData<fn() -> MessageXmlElements>,
}

impl MessageXmlElementsVisitor {
    fn new() -> Self {
        MessageXmlElementsVisitor {
            marker: PhantomData,
        }
    }
}

/// An internal helper type used in the [`MessageXmlElementsVisitor`] to extract the
/// content of an element: its text, the [`TYPES_NS_URI`](crate::TYPES_NS_URI) XML
/// namespace declaration, or, if it has neither, its XML attributes (e.g.
/// `<t:FieldURI FieldURI="calendar:IsMeeting"/>`).
#[derive(Debug, Clone, PartialEq)]
enum Text {
    Content(String),
    TypesNsDeclaration,
    Attributed(Vec<(String, String)>),
}

struct TextVisitor;

impl<'de> Visitor<'de> for TextVisitor {
    type Value = Text;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("element text, a namespace declaration, or element attributes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "http://schemas.microsoft.com/exchange/services/2006/types" {
            Ok(Text::TypesNsDeclaration)
        } else {
            Ok(Text::Content(v.to_string()))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut attributes = vec![];

        while let Some(key) = access.next_key::<String>()? {
            if key == "$text" {
                let value = access.next_value::<String>()?;
                return Ok(Text::Content(value));
            } else if key == "http://schemas.microsoft.com/exchange/services/2006/types" {
                access.next_value::<de::IgnoredAny>()?;
                return Ok(Text::TypesNsDeclaration);
            } else if let Some(attribute_name) = key.strip_prefix('@') {
                let value = access.next_value::<String>()?;
                attributes.push((attribute_name.to_string(), value));
            } else {
                access.next_value::<de::IgnoredAny>()?;
            }
        }

        Ok(Text::Attributed(attributes))
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TextVisitor)
    }
}

impl<'de> Visitor<'de> for MessageXmlElementsVisitor {
    type Value = MessageXmlElements;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("non-recursive XML elements with @Name or no attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut elements = vec![];

        while let Some(name) = access.next_key::<String>()? {
            if name.as_str() == "Value" {
                let element = access.next_value::<MessageXmlValue>()?;
                elements.push(MessageXmlElement::MessageXmlValue(element));
            } else {
                match access.next_value::<Text>()? {
                    Text::Content(value) => {
                        elements.push(MessageXmlElement::MessageXmlTagged(MessageXmlTagged {
                            name,
                            value,
                        }));
                    }
                    Text::TypesNsDeclaration => {}
                    Text::Attributed(attributes) => {
                        elements.push(MessageXmlElement::MessageXmlAttributed(
                            MessageXmlAttributed { name, attributes },
                        ));
                    }
                }
            }
        }

        Ok(MessageXmlElements { elements })
    }
}

impl<'de> Deserialize<'de> for MessageXmlElements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MessageXmlElementsVisitor::new())
    }
}
