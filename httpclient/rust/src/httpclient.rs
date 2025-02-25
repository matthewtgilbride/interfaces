// This file is generated automatically using wasmcloud/weld-codegen 0.4.2

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

pub const SMITHY_VERSION: &str = "1.0";

/// map data structure for holding http headers
///
pub type HeaderMap = std::collections::HashMap<String, HeaderValues>;

// Encode HeaderMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_header_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HeaderMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        encode_header_values(e, v)?;
    }
    Ok(())
}

// Decode HeaderMap from cbor input stream
#[doc(hidden)]
pub fn decode_header_map(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<HeaderMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, HeaderValues> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = decode_header_values(d)
                        .map_err(|e| format!("decoding 'HeaderValues': {}", e))?;
                    m.insert(k, v);
                }
            } else {
                return Err(RpcError::Deser("indefinite maps not supported".to_string()));
            }
            m
        }
    };
    Ok(__result)
}
pub type HeaderValues = Vec<String>;

// Encode HeaderValues as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_header_values<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HeaderValues,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.str(item)?;
    }
    Ok(())
}

// Decode HeaderValues from cbor input stream
#[doc(hidden)]
pub fn decode_header_values(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<HeaderValues, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<String> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.str()?.to_string())
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<String> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.str()?.to_string()),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// http request to be sent through the provider
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HttpRequest {
    /// http method, defaults to "GET"
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    /// optional headers. defaults to empty
    pub headers: HeaderMap,
    /// request body, defaults to empty
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

// Encode HttpRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_http_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HttpRequest,
) -> RpcResult<()> {
    e.array(4)?;
    e.str(&val.method)?;
    e.str(&val.url)?;
    encode_header_map(e, &val.headers)?;
    e.bytes(&val.body)?;
    Ok(())
}

// Decode HttpRequest from cbor input stream
#[doc(hidden)]
pub fn decode_http_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<HttpRequest, RpcError> {
    let __result = {
        let mut method: Option<String> = None;
        let mut url: Option<String> = None;
        let mut headers: Option<HeaderMap> = None;
        let mut body: Option<Vec<u8>> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct HttpRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HttpRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => method = Some(d.str()?.to_string()),
                    1 => url = Some(d.str()?.to_string()),
                    2 => {
                        headers = Some(
                            decode_header_map(d)
                                .map_err(|e| format!("decoding 'HeaderMap': {}", e))?,
                        )
                    }
                    3 => body = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HttpRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "method" => method = Some(d.str()?.to_string()),
                    "url" => url = Some(d.str()?.to_string()),
                    "headers" => {
                        headers = Some(
                            decode_header_map(d)
                                .map_err(|e| format!("decoding 'HeaderMap': {}", e))?,
                        )
                    }
                    "body" => body = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        }
        HttpRequest {
            method: if let Some(__x) = method {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpRequest.method (#0)".to_string(),
                ));
            },

            url: if let Some(__x) = url {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpRequest.url (#1)".to_string(),
                ));
            },

            headers: if let Some(__x) = headers {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpRequest.headers (#2)".to_string(),
                ));
            },

            body: if let Some(__x) = body {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpRequest.body (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// response from the http request
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HttpResponse {
    /// response status code
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: u16,
    /// Case is not guaranteed to be normalized, so
    /// actors checking response headers need to do their own
    /// case conversion.
    /// Example (rust):
    /// // check for 'Content-Type' header
    /// let content_type:Option<&Vec<String>> = header.iter()
    /// .map(|(k,_)| k.to_ascii_lowercase())
    /// .find(|(k,_)| k == "content-type")
    /// .map(|(_,v)| v);
    pub header: HeaderMap,
    /// response body
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

// Encode HttpResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_http_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HttpResponse,
) -> RpcResult<()> {
    e.array(3)?;
    e.u16(val.status_code)?;
    encode_header_map(e, &val.header)?;
    e.bytes(&val.body)?;
    Ok(())
}

// Decode HttpResponse from cbor input stream
#[doc(hidden)]
pub fn decode_http_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<HttpResponse, RpcError> {
    let __result = {
        let mut status_code: Option<u16> = None;
        let mut header: Option<HeaderMap> = None;
        let mut body: Option<Vec<u8>> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct HttpResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HttpResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => status_code = Some(d.u16()?),
                    1 => {
                        header = Some(
                            decode_header_map(d)
                                .map_err(|e| format!("decoding 'HeaderMap': {}", e))?,
                        )
                    }
                    2 => body = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HttpResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "statusCode" => status_code = Some(d.u16()?),
                    "header" => {
                        header = Some(
                            decode_header_map(d)
                                .map_err(|e| format!("decoding 'HeaderMap': {}", e))?,
                        )
                    }
                    "body" => body = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        }
        HttpResponse {
            status_code: if let Some(__x) = status_code {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpResponse.status_code (#0)".to_string(),
                ));
            },

            header: if let Some(__x) = header {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpResponse.header (#1)".to_string(),
                ));
            },

            body: if let Some(__x) = body {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HttpResponse.body (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// HttpClient - issue outgoing http requests via an external provider
/// To use this capability, the actor must be linked
/// with "wasmcloud:httpclient"
/// wasmbus.contractId: wasmcloud:httpclient
/// wasmbus.providerReceive
#[async_trait]
pub trait HttpClient {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:httpclient"
    }
    /// Issue outgoing http request
    async fn request(&self, ctx: &Context, arg: &HttpRequest) -> RpcResult<HttpResponse>;
}

/// HttpClientReceiver receives messages defined in the HttpClient service trait
/// HttpClient - issue outgoing http requests via an external provider
/// To use this capability, the actor must be linked
/// with "wasmcloud:httpclient"
#[doc(hidden)]
#[async_trait]
pub trait HttpClientReceiver: MessageDispatch + HttpClient {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Request" => {
                let value: HttpRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'HttpRequest': {}", e)))?;
                let resp = HttpClient::request(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "HttpClient.Request",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "HttpClient::{}",
                message.method
            ))),
        }
    }
}

/// HttpClientSender sends messages to a HttpClient service
/// HttpClient - issue outgoing http requests via an external provider
/// To use this capability, the actor must be linked
/// with "wasmcloud:httpclient"
/// client for sending HttpClient messages
#[derive(Debug)]
pub struct HttpClientSender<T: Transport> {
    transport: T,
}

impl<T: Transport> HttpClientSender<T> {
    /// Constructs a HttpClientSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl HttpClientSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a HttpClient provider
    /// implementing the 'wasmcloud:httpclient' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:httpclient", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a HttpClient provider
    /// implementing the 'wasmcloud:httpclient' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:httpclient", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> HttpClient for HttpClientSender<T> {
    #[allow(unused)]
    /// Issue outgoing http request
    async fn request(&self, ctx: &Context, arg: &HttpRequest) -> RpcResult<HttpResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "HttpClient.Request",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: HttpResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': HttpResponse", e)))?;
        Ok(value)
    }
}
