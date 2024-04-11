/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, Event},
    Reader, Writer,
};
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{Error, SOAP_NS_URI, TYPES_NS_URI};

/// A SOAP envelope wrapping an EWS operation.
#[derive(Debug)]
pub struct Envelope<B> {
    pub body: B,
}

impl<B> Envelope<B>
where
    B: XmlSerialize,
{
    /// Serializes the SOAP envelope as a complete XML document.
    pub fn as_xml_document(&self) -> Result<Vec<u8>, Error> {
        let mut writer = {
            let inner: Vec<u8> = Default::default();
            Writer::new(inner)
        };

        // All EWS examples use XML 1.0 with UTF-8, so stick to that for now.
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;

        // To get around having to make `Envelope` itself implement
        // `XmlSerialize`
        writer.write_event(Event::Start(
            BytesStart::new("soap:Envelope")
                .with_attributes([("xmlns:soap", SOAP_NS_URI), ("xmlns:t", TYPES_NS_URI)]),
        ))?;

        self.body.serialize_as_element(&mut writer, "soap:Body")?;

        writer.write_event(Event::End(BytesEnd::new("soap:Envelope")))?;

        Ok(writer.into_inner())
    }
}

impl<B> Envelope<B>
where
    B: for<'de> Deserialize<'de>,
{
    /// Populates an [`Envelope`] from raw XML.
    pub fn from_xml_document(document: &[u8]) -> Result<Self, Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct DummyEnvelope<T> {
            body: DummyBody<T>,
        }

        #[derive(Deserialize)]
        struct DummyBody<T> {
            #[serde(rename = "$value")]
            inner: T,
        }

        let fault = extract_maybe_fault(document)?;
        if let Some(fault) = fault {
            return Err(Error::RequestFault(Box::new(fault)));
        }

        let envelope: DummyEnvelope<B> = quick_xml::de::from_reader(document)?;

        Ok(Envelope {
            body: envelope.body.inner,
        })
    }
}

fn extract_maybe_fault(document: &[u8]) -> Result<Option<Fault>, Error> {
    let mut reader = TargetedReader::from_bytes(document);

    let mut envelope_reader = reader
        .maybe_get_subreader_for_tag("Envelope")?
        .ok_or_else(|| unexpected_response(document))?;
    let mut body_reader = envelope_reader
        .maybe_get_subreader_for_tag("Body")?
        .ok_or_else(|| unexpected_response(document))?;

    let fault_reader = body_reader.maybe_get_subreader_for_tag("Fault")?;
    let fault = if let Some(mut reader) = fault_reader {
        let mut fault_code = None;
        let mut fault_string = None;
        let mut fault_actor = None;
        let mut detail = None;

        while let Some((name, subreader)) = reader.maybe_get_next_subreader()? {
            match name.as_slice() {
                b"faultcode" => {
                    fault_code.replace(subreader.to_string()?);
                }

                b"faultstring" => {
                    fault_string.replace(subreader.to_string()?);
                }

                b"faultactor" => {
                    fault_actor.replace(subreader.to_string()?);
                }

                b"detail" => {
                    detail.replace(parse_detail_field(subreader)?);
                }

                _ => {
                    // This implies that Microsoft is breaking the SOAP spec. We
                    // don't want to error, but we should log anyhow.
                    log::warn!(
                        "encountered unexpected element {} in soap:Fault",
                        subreader.reader.decoder().decode(&name)?
                    )
                }
            }
        }

        let fault_code = fault_code.ok_or_else(|| unexpected_response(document))?;
        let fault_string = fault_string.ok_or_else(|| unexpected_response(document))?;

        Some(Fault {
            fault_code,
            fault_string,
            fault_actor,
            detail,
        })
    } else {
        None
    };

    Ok(fault)
}

fn parse_detail_field(mut reader: TargetedReader) -> Result<FaultDetail, Error> {
    let mut detail = FaultDetail::default();

    while let Some((name, subreader)) = reader.maybe_get_next_subreader()? {
        match name.as_slice() {
            b"ResponseCode" => {
                detail.response_code.replace(subreader.to_string()?);
            }

            b"Message" => {
                detail.message.replace(subreader.to_string()?);
            }

            b"MessageXml" => {
                detail.message_xml.replace(parse_message_xml(subreader)?);
            }

            _ => {
                // If we've already stored a copy of the full content, we don't
                // need to replace it.
                if detail.content.is_none() {
                    detail.content.replace(reader.to_string()?);
                }
            }
        }
    }

    Ok(detail)
}

fn parse_message_xml(mut reader: TargetedReader) -> Result<MessageXml, Error> {
    let back_off_milliseconds = loop {
        match reader.reader.read_event()? {
            Event::Start(start) => {
                if start.local_name().as_ref() == b"Value" {
                    let is_back_off = start.attributes().any(|attr_result| match attr_result {
                        Ok(attr) => {
                            attr.key.local_name().as_ref() == b"Name"
                                && attr.value.as_ref() == b"BackOffMilliseconds"
                        }
                        Err(_) => false,
                    });

                    if is_back_off {
                        let text = reader.reader.read_text(start.name())?;
                        if let Ok(value) = text.parse::<usize>() {
                            break Some(value);
                        }
                    }
                }
            }

            Event::Eof => break None,

            _ => continue,
        }
    };

    Ok(MessageXml {
        content: reader.to_string()?,
        back_off_milliseconds,
    })
}

fn unexpected_response(document: &[u8]) -> Error {
    Error::UnexpectedResponse(Vec::from(document))
}

struct TargetedReader<'content> {
    reader: Reader<&'content [u8]>,
    content: &'content [u8],
}

impl<'content> TargetedReader<'content> {
    fn from_bytes(content: &'content [u8]) -> Self {
        Self {
            reader: Reader::from_reader(content),
            content,
        }
    }

    fn maybe_get_subreader_for_tag(&mut self, local_name: &str) -> Result<Option<Self>, Error> {
        loop {
            match self.reader.read_event()? {
                Event::Start(start) => {
                    if start.local_name().as_ref() == local_name.as_bytes() {
                        return self.get_subreader_for_start(&start).map(Some);
                    }
                }

                Event::Eof => break,

                _ => continue,
            }
        }

        Ok(None)
    }

    fn maybe_get_next_subreader(&mut self) -> Result<Option<(Vec<u8>, Self)>, Error> {
        loop {
            match self.reader.read_event()? {
                Event::Start(start) => {
                    let reader = self.get_subreader_for_start(&start)?;
                    let local_name = start.local_name();

                    return Ok(Some((local_name.as_ref().to_owned(), reader)));
                }

                Event::Eof => break,

                _ => continue,
            }
        }

        Ok(None)
    }

    fn get_subreader_for_start(&mut self, start: &BytesStart<'content>) -> Result<Self, Error> {
        let span = self.reader.read_to_end(start.name())?;
        let content = &self.content[span];

        // Notably, in doing this, we throw away any encoding information we may
        // have had from the original reader. However, Microsoft _appears_ to
        // send all responses as UTF-8. We'll encounter bigger problems
        // elsewhere if we run into a non-UTF-8 document, most notably that we
        // currently don't enable the `encoding` feature for quick-xml.
        return Ok(Self::from_bytes(content));
    }

    fn to_string(&self) -> Result<String, Error> {
        Ok(self.reader.decoder().decode(self.content)?.into_owned())
    }
}

///
#[derive(Debug, PartialEq)]
pub struct Fault {
    /// An error code indicating the fault in the original request.
    // While `faultcode` is defined in the SOAP spec as a `QName`, we avoid
    // using `quick_xml::name::QName` as it borrows from the input and does not
    // allow for containing a string representation. We could use the `QName`
    // type to parse the contents of the field and store them in our own type if
    // we found value in this field beyond debug output.
    pub fault_code: String,

    ///
    pub fault_string: String,
    pub fault_actor: Option<String>,
    pub detail: Option<FaultDetail>,
}

#[derive(Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct FaultDetail {
    pub response_code: Option<String>,
    pub message: Option<String>,
    pub message_xml: Option<MessageXml>,
    pub content: Option<String>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct MessageXml {
    pub content: String,
    pub back_off_milliseconds: Option<usize>,
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::Envelope;

    #[test]
    fn envelope_with_schema_fault() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?><s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/"><s:Body><s:Fault><faultcode xmlns:a="http://schemas.microsoft.com/exchange/services/2006/types">a:ErrorSchemaValidation</faultcode><faultstring xml:lang="en-US">The request failed schema validation: The 'Id' attribute is invalid - The value 'invalidparentid' is invalid according to its datatype 'http://schemas.microsoft.com/exchange/services/2006/types:DistinguishedFolderIdNameType' - The Enumeration constraint failed.</faultstring><detail><e:ResponseCode xmlns:e="http://schemas.microsoft.com/exchange/services/2006/errors">ErrorSchemaValidation</e:ResponseCode><e:Message xmlns:e="http://schemas.microsoft.com/exchange/services/2006/errors">The request failed schema validation.</e:Message><t:MessageXml xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"><t:LineNumber>2</t:LineNumber><t:LinePosition>630</t:LinePosition><t:Violation>The 'Id' attribute is invalid - The value 'invalidparentid' is invalid according to its datatype 'http://schemas.microsoft.com/exchange/services/2006/types:DistinguishedFolderIdNameType' - The Enumeration constraint failed.</t:Violation></t:MessageXml></detail></s:Fault></s:Body></s:Envelope>"#;
        let err = <Envelope<()>>::from_xml_document(xml.as_bytes())
            .expect_err("should return error when body contains fault");

        if let Error::RequestFault(fault) = err {
            assert_eq!(
                fault.fault_code, "a:ErrorSchemaValidation",
                "fault code should match original document"
            );
            assert_eq!(fault.fault_string, "The request failed schema validation: The 'Id' attribute is invalid - The value 'invalidparentid' is invalid according to its datatype 'http://schemas.microsoft.com/exchange/services/2006/types:DistinguishedFolderIdNameType' - The Enumeration constraint failed.", "fault string should match original document");
            assert!(
                fault.fault_actor.is_none(),
                "fault actor should not be present"
            );

            let detail = fault.detail.expect("fault detail should be present");
            assert_eq!(
                detail.response_code,
                Some("ErrorSchemaValidation".to_string()),
                "response code should match original document"
            );
            assert_eq!(
                detail.message,
                Some("The request failed schema validation.".to_string()),
                "error message should match original document"
            );

            let message_xml = detail.message_xml.expect("message XML should be present");
            assert_eq!(&message_xml.content, "<t:LineNumber>2</t:LineNumber><t:LinePosition>630</t:LinePosition><t:Violation>The 'Id' attribute is invalid - The value 'invalidparentid' is invalid according to its datatype 'http://schemas.microsoft.com/exchange/services/2006/types:DistinguishedFolderIdNameType' - The Enumeration constraint failed.</t:Violation>", "message XML content should contain full body of MessageXml tag");
            assert!(
                message_xml.back_off_milliseconds.is_none(),
                "back off milliseconds should not be present"
            );
        } else {
            panic!("error should be request fault");
        }
    }

    #[test]
    fn envelope_with_server_busy_fault() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?><s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/"><s:Body><s:Fault><faultcode xmlns:a="http://schemas.microsoft.com/exchange/services/2006/types">a:ErrorServerBusy</faultcode><faultstring xml:lang="en-US">I made this up because I don't have real testing data. 🙃</faultstring><detail><e:ResponseCode xmlns:e="http://schemas.microsoft.com/exchange/services/2006/errors">ErrorServerBusy</e:ResponseCode><e:Message xmlns:e="http://schemas.microsoft.com/exchange/services/2006/errors">Who really knows?</e:Message><t:MessageXml xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"><t:Value Name="BackOffMilliseconds">25</t:Value></t:MessageXml></detail></s:Fault></s:Body></s:Envelope>"#;
        let err = <Envelope<()>>::from_xml_document(xml.as_bytes())
            .expect_err("should return error when body contains fault");

        if let Error::RequestFault(fault) = err {
            assert_eq!(
                fault.fault_code, "a:ErrorServerBusy",
                "fault code should match original document"
            );
            assert!(
                fault.fault_actor.is_none(),
                "fault actor should not be present"
            );

            let detail = fault.detail.expect("fault detail should be present");
            assert_eq!(
                detail.response_code,
                Some("ErrorServerBusy".to_string()),
                "response code should match original document"
            );

            let message_xml = detail.message_xml.expect("message XML should be present");
            assert_eq!(message_xml.back_off_milliseconds, Some(25));
        } else {
            panic!("error should be request fault");
        }
    }
}
