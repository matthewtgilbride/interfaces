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

/// A portion of a file. The `isLast` field indicates whether this chunk
/// is the last in a stream. The `offset` field indicates the 0-based offset
/// from the start of the file for this chunk.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Chunk {
    #[serde(rename = "objectId")]
    pub object_id: ObjectId,
    #[serde(rename = "containerId")]
    pub container_id: ContainerId,
    /// bytes in this chunk
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub bytes: Vec<u8>,
    /// The byte offset within the object for this chunk
    #[serde(default)]
    pub offset: u64,
    /// true if this is the last chunk
    #[serde(rename = "isLast")]
    #[serde(default)]
    pub is_last: bool,
}

// Encode Chunk as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_chunk<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Chunk,
) -> RpcResult<()> {
    e.array(5)?;
    encode_object_id(e, &val.object_id)?;
    encode_container_id(e, &val.container_id)?;
    e.bytes(&val.bytes)?;
    e.u64(val.offset)?;
    e.bool(val.is_last)?;
    Ok(())
}

// Decode Chunk from cbor input stream
#[doc(hidden)]
pub fn decode_chunk(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Chunk, RpcError> {
    let __result = {
        let mut object_id: Option<ObjectId> = None;
        let mut container_id: Option<ContainerId> = None;
        let mut bytes: Option<Vec<u8>> = None;
        let mut offset: Option<u64> = None;
        let mut is_last: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Chunk, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser("decoding struct Chunk: indefinite array not supported".to_string())
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    1 => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    2 => bytes = Some(d.bytes()?.to_vec()),
                    3 => offset = Some(d.u64()?),
                    4 => is_last = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser("decoding struct Chunk: indefinite map not supported".to_string())
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "objectId" => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    "containerId" => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    "bytes" => bytes = Some(d.bytes()?.to_vec()),
                    "offset" => offset = Some(d.u64()?),
                    "isLast" => is_last = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        Chunk {
            object_id: if let Some(__x) = object_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Chunk.object_id (#0)".to_string(),
                ));
            },

            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Chunk.container_id (#1)".to_string(),
                ));
            },

            bytes: if let Some(__x) = bytes {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Chunk.bytes (#2)".to_string(),
                ));
            },

            offset: if let Some(__x) = offset {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Chunk.offset (#3)".to_string(),
                ));
            },

            is_last: if let Some(__x) = is_last {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Chunk.is_last (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Response from actor after receiving a download chunk.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChunkResponse {
    /// If set and `true`, the sender will stop sending chunks,
    #[serde(rename = "cancelDownload")]
    #[serde(default)]
    pub cancel_download: bool,
}

// Encode ChunkResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_chunk_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ChunkResponse,
) -> RpcResult<()> {
    e.array(1)?;
    e.bool(val.cancel_download)?;
    Ok(())
}

// Decode ChunkResponse from cbor input stream
#[doc(hidden)]
pub fn decode_chunk_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ChunkResponse, RpcError> {
    let __result = {
        let mut cancel_download: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ChunkResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ChunkResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => cancel_download = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ChunkResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "cancelDownload" => cancel_download = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        ChunkResponse {
            cancel_download: if let Some(__x) = cancel_download {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ChunkResponse.cancel_download (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Name of a container
pub type ContainerId = String;

// Encode ContainerId as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_container_id<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ContainerId,
) -> RpcResult<()> {
    e.str(val)?;
    Ok(())
}

// Decode ContainerId from cbor input stream
#[doc(hidden)]
pub fn decode_container_id(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ContainerId, RpcError> {
    let __result = { d.str()?.to_string() };
    Ok(__result)
}
/// list of container names
pub type ContainerIds = Vec<ContainerId>;

// Encode ContainerIds as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_container_ids<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ContainerIds,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_container_id(e, item)?;
    }
    Ok(())
}

// Decode ContainerIds from cbor input stream
#[doc(hidden)]
pub fn decode_container_ids(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ContainerIds, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ContainerId> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_container_id(d).map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ContainerId> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_container_id(d)
                            .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Metadata for a container.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContainerMetadata {
    /// Container name
    #[serde(rename = "containerId")]
    pub container_id: ContainerId,
    /// Creation date, if available
    #[serde(rename = "createdAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

// Encode ContainerMetadata as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_container_metadata<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ContainerMetadata,
) -> RpcResult<()> {
    e.array(2)?;
    encode_container_id(e, &val.container_id)?;
    if let Some(val) = val.created_at.as_ref() {
        e.i64(val.sec)?;
        e.u32(val.nsec)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ContainerMetadata from cbor input stream
#[doc(hidden)]
pub fn decode_container_metadata(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ContainerMetadata, RpcError> {
    let __result = {
        let mut container_id: Option<ContainerId> = None;
        let mut created_at: Option<Option<Timestamp>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ContainerMetadata, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ContainerMetadata: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    1 => {
                        created_at = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(wasmbus_rpc::Timestamp {
                                sec: d.i64()?,
                                nsec: d.u32()?,
                            }))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ContainerMetadata: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "containerId" => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    "createdAt" => {
                        created_at = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(wasmbus_rpc::Timestamp {
                                sec: d.i64()?,
                                nsec: d.u32()?,
                            }))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ContainerMetadata {
            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ContainerMetadata.container_id (#0)".to_string(),
                ));
            },
            created_at: created_at.unwrap(),
        }
    };
    Ok(__result)
}
/// Combination of container id and object id
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContainerObject {
    #[serde(rename = "containerId")]
    pub container_id: ContainerId,
    #[serde(rename = "objectId")]
    pub object_id: ObjectId,
}

// Encode ContainerObject as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_container_object<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ContainerObject,
) -> RpcResult<()> {
    e.array(2)?;
    encode_container_id(e, &val.container_id)?;
    encode_object_id(e, &val.object_id)?;
    Ok(())
}

// Decode ContainerObject from cbor input stream
#[doc(hidden)]
pub fn decode_container_object(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ContainerObject, RpcError> {
    let __result = {
        let mut container_id: Option<ContainerId> = None;
        let mut object_id: Option<ObjectId> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ContainerObject, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ContainerObject: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    1 => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ContainerObject: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "containerId" => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    "objectId" => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        }
        ContainerObject {
            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ContainerObject.container_id (#0)".to_string(),
                ));
            },

            object_id: if let Some(__x) = object_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ContainerObject.object_id (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// list of container metadata objects
pub type ContainersInfo = Vec<ContainerMetadata>;

// Encode ContainersInfo as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_containers_info<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ContainersInfo,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_container_metadata(e, item)?;
    }
    Ok(())
}

// Decode ContainersInfo from cbor input stream
#[doc(hidden)]
pub fn decode_containers_info(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ContainersInfo, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ContainerMetadata> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_container_metadata(d)
                        .map_err(|e| format!("decoding 'ContainerMetadata': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ContainerMetadata> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_container_metadata(d)
                            .map_err(|e| format!("decoding 'ContainerMetadata': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Parameter to GetObject
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetObjectRequest {
    /// object to download
    #[serde(rename = "objectId")]
    pub object_id: ObjectId,
    /// object's container
    #[serde(rename = "containerId")]
    pub container_id: ContainerId,
    /// Requested start of object to retrieve.
    /// The first byte is at offset 0. Range values are inclusive.
    /// If rangeStart is beyond the end of the file,
    /// an empty chunk will be returned with isLast == true
    #[serde(rename = "rangeStart")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_start: Option<u64>,
    /// Requested end of object to retrieve. Defaults to the object's size.
    /// It is not an error for rangeEnd to be greater than the object size.
    /// Range values are inclusive.
    #[serde(rename = "rangeEnd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_end: Option<u64>,
}

// Encode GetObjectRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_get_object_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &GetObjectRequest,
) -> RpcResult<()> {
    e.array(4)?;
    encode_object_id(e, &val.object_id)?;
    encode_container_id(e, &val.container_id)?;
    if let Some(val) = val.range_start.as_ref() {
        e.u64(*val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.range_end.as_ref() {
        e.u64(*val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode GetObjectRequest from cbor input stream
#[doc(hidden)]
pub fn decode_get_object_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<GetObjectRequest, RpcError> {
    let __result = {
        let mut object_id: Option<ObjectId> = None;
        let mut container_id: Option<ContainerId> = None;
        let mut range_start: Option<Option<u64>> = Some(None);
        let mut range_end: Option<Option<u64>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct GetObjectRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetObjectRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    1 => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    2 => {
                        range_start = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u64()?))
                        }
                    }
                    3 => {
                        range_end = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u64()?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetObjectRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "objectId" => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    "containerId" => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    "rangeStart" => {
                        range_start = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u64()?))
                        }
                    }
                    "rangeEnd" => {
                        range_end = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u64()?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        GetObjectRequest {
            object_id: if let Some(__x) = object_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetObjectRequest.object_id (#0)".to_string(),
                ));
            },

            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetObjectRequest.container_id (#1)".to_string(),
                ));
            },
            range_start: range_start.unwrap(),
            range_end: range_end.unwrap(),
        }
    };
    Ok(__result)
}
/// Response to GetObject
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetObjectResponse {
    /// indication whether the request was successful
    #[serde(default)]
    pub success: bool,
    /// If success is false, this may contain an error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The provider may begin the download by returning a first chunk
    #[serde(rename = "initialChunk")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_chunk: Option<Chunk>,
    /// Length of the content. (for multi-part downloads, this may not
    /// be the same as the length of the initial chunk)
    #[serde(rename = "contentLength")]
    #[serde(default)]
    pub content_length: u64,
    /// A standard MIME type describing the format of the object data.
    #[serde(rename = "contentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Specifies what content encodings have been applied to the object
    /// and thus what decoding mechanisms must be applied to obtain the media-type
    #[serde(rename = "contentEncoding")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
}

// Encode GetObjectResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_get_object_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &GetObjectResponse,
) -> RpcResult<()> {
    e.array(6)?;
    e.bool(val.success)?;
    if let Some(val) = val.error.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.initial_chunk.as_ref() {
        encode_chunk(e, val)?;
    } else {
        e.null()?;
    }
    e.u64(val.content_length)?;
    if let Some(val) = val.content_type.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.content_encoding.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode GetObjectResponse from cbor input stream
#[doc(hidden)]
pub fn decode_get_object_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<GetObjectResponse, RpcError> {
    let __result = {
        let mut success: Option<bool> = None;
        let mut error: Option<Option<String>> = Some(None);
        let mut initial_chunk: Option<Option<Chunk>> = Some(None);
        let mut content_length: Option<u64> = None;
        let mut content_type: Option<Option<String>> = Some(None);
        let mut content_encoding: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct GetObjectResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetObjectResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => success = Some(d.bool()?),
                    1 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => {
                        initial_chunk = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_chunk(d).map_err(|e| format!("decoding 'Chunk': {}", e))?,
                            ))
                        }
                    }
                    3 => content_length = Some(d.u64()?),
                    4 => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    5 => {
                        content_encoding = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetObjectResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "success" => success = Some(d.bool()?),
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "initialChunk" => {
                        initial_chunk = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_chunk(d).map_err(|e| format!("decoding 'Chunk': {}", e))?,
                            ))
                        }
                    }
                    "contentLength" => content_length = Some(d.u64()?),
                    "contentType" => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "contentEncoding" => {
                        content_encoding = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        GetObjectResponse {
            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetObjectResponse.success (#0)".to_string(),
                ));
            },
            error: error.unwrap(),
            initial_chunk: initial_chunk.unwrap(),

            content_length: if let Some(__x) = content_length {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetObjectResponse.content_length (#3)".to_string(),
                ));
            },
            content_type: content_type.unwrap(),
            content_encoding: content_encoding.unwrap(),
        }
    };
    Ok(__result)
}
/// Result of input item
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemResult {
    #[serde(default)]
    pub key: String,
    /// whether the item succeeded or failed
    #[serde(default)]
    pub success: bool,
    /// optional error message for failures
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// Encode ItemResult as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_item_result<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ItemResult,
) -> RpcResult<()> {
    e.array(3)?;
    e.str(&val.key)?;
    e.bool(val.success)?;
    if let Some(val) = val.error.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ItemResult from cbor input stream
#[doc(hidden)]
pub fn decode_item_result(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ItemResult, RpcError> {
    let __result = {
        let mut key: Option<String> = None;
        let mut success: Option<bool> = None;
        let mut error: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ItemResult, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ItemResult: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => key = Some(d.str()?.to_string()),
                    1 => success = Some(d.bool()?),
                    2 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ItemResult: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "key" => key = Some(d.str()?.to_string()),
                    "success" => success = Some(d.bool()?),
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ItemResult {
            key: if let Some(__x) = key {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ItemResult.key (#0)".to_string(),
                ));
            },

            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ItemResult.success (#1)".to_string(),
                ));
            },
            error: error.unwrap(),
        }
    };
    Ok(__result)
}
/// Parameter to list_objects.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListObjectsRequest {
    /// Name of the container to search
    #[serde(rename = "containerId")]
    #[serde(default)]
    pub container_id: String,
    /// Request object names starting with this value. (Optional)
    #[serde(rename = "startWith")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_with: Option<String>,
    /// Continuation token passed in ListObjectsResponse.
    /// If set, `startWith` is ignored. (Optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    /// Last item to return (inclusive terminator) (Optional)
    #[serde(rename = "endWith")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_with: Option<String>,
    /// Optionally, stop returning items before returning this value.
    /// (exclusive terminator)
    /// If startFrom is "a" and endBefore is "b", and items are ordered
    /// alphabetically, then only items beginning with "a" would be returned.
    /// (Optional)
    #[serde(rename = "endBefore")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_before: Option<String>,
    /// maximum number of items to return. If not specified, provider
    /// will return an initial set of up to 1000 items. if maxItems > 1000,
    /// the provider implementation may return fewer items than requested.
    /// (Optional)
    #[serde(rename = "maxItems")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u32>,
}

// Encode ListObjectsRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_list_objects_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ListObjectsRequest,
) -> RpcResult<()> {
    e.array(6)?;
    e.str(&val.container_id)?;
    if let Some(val) = val.start_with.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.continuation.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.end_with.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.end_before.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.max_items.as_ref() {
        e.u32(*val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ListObjectsRequest from cbor input stream
#[doc(hidden)]
pub fn decode_list_objects_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ListObjectsRequest, RpcError> {
    let __result = {
        let mut container_id: Option<String> = None;
        let mut start_with: Option<Option<String>> = Some(None);
        let mut continuation: Option<Option<String>> = Some(None);
        let mut end_with: Option<Option<String>> = Some(None);
        let mut end_before: Option<Option<String>> = Some(None);
        let mut max_items: Option<Option<u32>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ListObjectsRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ListObjectsRequest: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => container_id = Some(d.str()?.to_string()),
                    1 => {
                        start_with = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => {
                        continuation = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    3 => {
                        end_with = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    4 => {
                        end_before = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    5 => {
                        max_items = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u32()?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ListObjectsRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "containerId" => container_id = Some(d.str()?.to_string()),
                    "startWith" => {
                        start_with = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "continuation" => {
                        continuation = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "endWith" => {
                        end_with = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "endBefore" => {
                        end_before = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "maxItems" => {
                        max_items = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u32()?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ListObjectsRequest {
            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ListObjectsRequest.container_id (#0)".to_string(),
                ));
            },
            start_with: start_with.unwrap(),
            continuation: continuation.unwrap(),
            end_with: end_with.unwrap(),
            end_before: end_before.unwrap(),
            max_items: max_items.unwrap(),
        }
    };
    Ok(__result)
}
/// Respose to list_objects.
/// If `isLast` is false, the list was truncated by the provider,
/// and the remainder of the objects can be requested with another
/// request using the `continuation` token.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListObjectsResponse {
    /// set of objects returned
    pub objects: ObjectsInfo,
    /// Indicates if the item list is complete, or the last item
    /// in a multi-part response.
    #[serde(rename = "isLast")]
    #[serde(default)]
    pub is_last: bool,
    /// If `isLast` is false, this value can be used in the `continuation` field
    /// of a `ListObjectsRequest`.
    /// Clients should not attempt to interpret this field: it may or may not
    /// be a real key or object name, and may be obfuscated by the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

// Encode ListObjectsResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_list_objects_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ListObjectsResponse,
) -> RpcResult<()> {
    e.array(3)?;
    encode_objects_info(e, &val.objects)?;
    e.bool(val.is_last)?;
    if let Some(val) = val.continuation.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ListObjectsResponse from cbor input stream
#[doc(hidden)]
pub fn decode_list_objects_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ListObjectsResponse, RpcError> {
    let __result = {
        let mut objects: Option<ObjectsInfo> = None;
        let mut is_last: Option<bool> = None;
        let mut continuation: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ListObjectsResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ListObjectsResponse: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        objects = Some(
                            decode_objects_info(d)
                                .map_err(|e| format!("decoding 'ObjectsInfo': {}", e))?,
                        )
                    }
                    1 => is_last = Some(d.bool()?),
                    2 => {
                        continuation = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ListObjectsResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "objects" => {
                        objects = Some(
                            decode_objects_info(d)
                                .map_err(|e| format!("decoding 'ObjectsInfo': {}", e))?,
                        )
                    }
                    "isLast" => is_last = Some(d.bool()?),
                    "continuation" => {
                        continuation = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ListObjectsResponse {
            objects: if let Some(__x) = objects {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ListObjectsResponse.objects (#0)".to_string(),
                ));
            },

            is_last: if let Some(__x) = is_last {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ListObjectsResponse.is_last (#1)".to_string(),
                ));
            },
            continuation: continuation.unwrap(),
        }
    };
    Ok(__result)
}
/// result for an operation on a list of inputs
pub type MultiResult = Vec<ItemResult>;

// Encode MultiResult as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_multi_result<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &MultiResult,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_item_result(e, item)?;
    }
    Ok(())
}

// Decode MultiResult from cbor input stream
#[doc(hidden)]
pub fn decode_multi_result(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<MultiResult, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ItemResult> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_item_result(d).map_err(|e| format!("decoding 'ItemResult': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ItemResult> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_item_result(d)
                            .map_err(|e| format!("decoding 'ItemResult': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Name of an object within a container
pub type ObjectId = String;

// Encode ObjectId as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_object_id<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ObjectId,
) -> RpcResult<()> {
    e.str(val)?;
    Ok(())
}

// Decode ObjectId from cbor input stream
#[doc(hidden)]
pub fn decode_object_id(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ObjectId, RpcError> {
    let __result = { d.str()?.to_string() };
    Ok(__result)
}
/// list of object names
pub type ObjectIds = Vec<ObjectId>;

// Encode ObjectIds as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_object_ids<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ObjectIds,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_object_id(e, item)?;
    }
    Ok(())
}

// Decode ObjectIds from cbor input stream
#[doc(hidden)]
pub fn decode_object_ids(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ObjectIds, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ObjectId> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_object_id(d).map_err(|e| format!("decoding 'ObjectId': {}", e))?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ObjectId> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_object_id(d).map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObjectMetadata {
    /// Object identifier that is unique within its container.
    /// Naming of objects is determined by the capability provider.
    /// An object id could be a path, hash of object contents, or some other unique identifier.
    #[serde(rename = "objectId")]
    pub object_id: ObjectId,
    /// container of the object
    #[serde(rename = "containerId")]
    pub container_id: ContainerId,
    /// size of the object in bytes
    #[serde(rename = "contentLength")]
    #[serde(default)]
    pub content_length: u64,
    /// date object was last modified
    #[serde(rename = "lastModified")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Timestamp>,
    /// A MIME type of the object
    /// see http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17
    /// Provider implementations _may_ return None for this field for metadata
    /// returned from ListObjects
    #[serde(rename = "contentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Specifies what content encodings have been applied to the object
    /// and thus what decoding mechanisms must be applied to obtain the media-type
    /// referenced by the contentType field. For more information,
    /// see http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11.
    /// Provider implementations _may_ return None for this field for metadata
    /// returned from ListObjects
    #[serde(rename = "contentEncoding")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
}

// Encode ObjectMetadata as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_object_metadata<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ObjectMetadata,
) -> RpcResult<()> {
    e.array(6)?;
    encode_object_id(e, &val.object_id)?;
    encode_container_id(e, &val.container_id)?;
    e.u64(val.content_length)?;
    if let Some(val) = val.last_modified.as_ref() {
        e.i64(val.sec)?;
        e.u32(val.nsec)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.content_type.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.content_encoding.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ObjectMetadata from cbor input stream
#[doc(hidden)]
pub fn decode_object_metadata(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ObjectMetadata, RpcError> {
    let __result = {
        let mut object_id: Option<ObjectId> = None;
        let mut container_id: Option<ContainerId> = None;
        let mut content_length: Option<u64> = None;
        let mut last_modified: Option<Option<Timestamp>> = Some(None);
        let mut content_type: Option<Option<String>> = Some(None);
        let mut content_encoding: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ObjectMetadata, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ObjectMetadata: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    1 => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    2 => content_length = Some(d.u64()?),
                    3 => {
                        last_modified = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(wasmbus_rpc::Timestamp {
                                sec: d.i64()?,
                                nsec: d.u32()?,
                            }))
                        }
                    }
                    4 => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    5 => {
                        content_encoding = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ObjectMetadata: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "objectId" => {
                        object_id = Some(
                            decode_object_id(d)
                                .map_err(|e| format!("decoding 'ObjectId': {}", e))?,
                        )
                    }
                    "containerId" => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    "contentLength" => content_length = Some(d.u64()?),
                    "lastModified" => {
                        last_modified = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(wasmbus_rpc::Timestamp {
                                sec: d.i64()?,
                                nsec: d.u32()?,
                            }))
                        }
                    }
                    "contentType" => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "contentEncoding" => {
                        content_encoding = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ObjectMetadata {
            object_id: if let Some(__x) = object_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ObjectMetadata.object_id (#0)".to_string(),
                ));
            },

            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ObjectMetadata.container_id (#1)".to_string(),
                ));
            },

            content_length: if let Some(__x) = content_length {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ObjectMetadata.content_length (#2)".to_string(),
                ));
            },
            last_modified: last_modified.unwrap(),
            content_type: content_type.unwrap(),
            content_encoding: content_encoding.unwrap(),
        }
    };
    Ok(__result)
}
/// list of object metadata objects
pub type ObjectsInfo = Vec<ObjectMetadata>;

// Encode ObjectsInfo as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_objects_info<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ObjectsInfo,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_object_metadata(e, item)?;
    }
    Ok(())
}

// Decode ObjectsInfo from cbor input stream
#[doc(hidden)]
pub fn decode_objects_info(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ObjectsInfo, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ObjectMetadata> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_object_metadata(d)
                        .map_err(|e| format!("decoding 'ObjectMetadata': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ObjectMetadata> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_object_metadata(d)
                            .map_err(|e| format!("decoding 'ObjectMetadata': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Parameter to PutChunk operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PutChunkRequest {
    /// upload chunk from the file.
    /// if chunk.isLast is set, this will be the last chunk uploaded
    pub chunk: Chunk,
    /// This value should be set to the `streamId` returned from the initial PutObject.
    #[serde(rename = "streamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// If set, the receiving provider should cancel the upload process
    /// and remove the file.
    #[serde(rename = "cancelAndRemove")]
    #[serde(default)]
    pub cancel_and_remove: bool,
}

// Encode PutChunkRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_put_chunk_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PutChunkRequest,
) -> RpcResult<()> {
    e.array(3)?;
    encode_chunk(e, &val.chunk)?;
    if let Some(val) = val.stream_id.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.bool(val.cancel_and_remove)?;
    Ok(())
}

// Decode PutChunkRequest from cbor input stream
#[doc(hidden)]
pub fn decode_put_chunk_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<PutChunkRequest, RpcError> {
    let __result = {
        let mut chunk: Option<Chunk> = None;
        let mut stream_id: Option<Option<String>> = Some(None);
        let mut cancel_and_remove: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct PutChunkRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct PutChunkRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        chunk =
                            Some(decode_chunk(d).map_err(|e| format!("decoding 'Chunk': {}", e))?)
                    }
                    1 => {
                        stream_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => cancel_and_remove = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct PutChunkRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "chunk" => {
                        chunk =
                            Some(decode_chunk(d).map_err(|e| format!("decoding 'Chunk': {}", e))?)
                    }
                    "streamId" => {
                        stream_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "cancelAndRemove" => cancel_and_remove = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        PutChunkRequest {
            chunk: if let Some(__x) = chunk {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field PutChunkRequest.chunk (#0)".to_string(),
                ));
            },
            stream_id: stream_id.unwrap(),

            cancel_and_remove: if let Some(__x) = cancel_and_remove {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field PutChunkRequest.cancel_and_remove (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Parameter for PutObject operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PutObjectRequest {
    /// File path and initial data
    pub chunk: Chunk,
    /// A MIME type of the object
    /// see http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17
    #[serde(rename = "contentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Specifies what content encodings have been applied to the object
    /// and thus what decoding mechanisms must be applied to obtain the media-type
    /// referenced by the contentType field. For more information,
    /// see http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11.
    #[serde(rename = "contentEncoding")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
}

// Encode PutObjectRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_put_object_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PutObjectRequest,
) -> RpcResult<()> {
    e.array(3)?;
    encode_chunk(e, &val.chunk)?;
    if let Some(val) = val.content_type.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.content_encoding.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode PutObjectRequest from cbor input stream
#[doc(hidden)]
pub fn decode_put_object_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<PutObjectRequest, RpcError> {
    let __result = {
        let mut chunk: Option<Chunk> = None;
        let mut content_type: Option<Option<String>> = Some(None);
        let mut content_encoding: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct PutObjectRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct PutObjectRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        chunk =
                            Some(decode_chunk(d).map_err(|e| format!("decoding 'Chunk': {}", e))?)
                    }
                    1 => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => {
                        content_encoding = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct PutObjectRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "chunk" => {
                        chunk =
                            Some(decode_chunk(d).map_err(|e| format!("decoding 'Chunk': {}", e))?)
                    }
                    "contentType" => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "contentEncoding" => {
                        content_encoding = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        PutObjectRequest {
            chunk: if let Some(__x) = chunk {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field PutObjectRequest.chunk (#0)".to_string(),
                ));
            },
            content_type: content_type.unwrap(),
            content_encoding: content_encoding.unwrap(),
        }
    };
    Ok(__result)
}
/// Response to PutObject operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PutObjectResponse {
    /// If this is a multipart upload, `streamId` must be returned
    /// with subsequent PutChunk requests
    #[serde(rename = "streamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

// Encode PutObjectResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_put_object_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PutObjectResponse,
) -> RpcResult<()> {
    e.array(1)?;
    if let Some(val) = val.stream_id.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode PutObjectResponse from cbor input stream
#[doc(hidden)]
pub fn decode_put_object_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<PutObjectResponse, RpcError> {
    let __result = {
        let mut stream_id: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct PutObjectResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct PutObjectResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        stream_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct PutObjectResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "streamId" => {
                        stream_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        PutObjectResponse {
            stream_id: stream_id.unwrap(),
        }
    };
    Ok(__result)
}
/// parameter to removeObjects
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RemoveObjectsRequest {
    /// name of container
    #[serde(rename = "containerId")]
    pub container_id: ContainerId,
    /// list of object names to be removed
    pub objects: ObjectIds,
}

// Encode RemoveObjectsRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_remove_objects_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RemoveObjectsRequest,
) -> RpcResult<()> {
    e.array(2)?;
    encode_container_id(e, &val.container_id)?;
    encode_object_ids(e, &val.objects)?;
    Ok(())
}

// Decode RemoveObjectsRequest from cbor input stream
#[doc(hidden)]
pub fn decode_remove_objects_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RemoveObjectsRequest, RpcError> {
    let __result = {
        let mut container_id: Option<ContainerId> = None;
        let mut objects: Option<ObjectIds> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RemoveObjectsRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct RemoveObjectsRequest: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    1 => {
                        objects = Some(
                            decode_object_ids(d)
                                .map_err(|e| format!("decoding 'ObjectIds': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct RemoveObjectsRequest: indefinite map not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "containerId" => {
                        container_id = Some(
                            decode_container_id(d)
                                .map_err(|e| format!("decoding 'ContainerId': {}", e))?,
                        )
                    }
                    "objects" => {
                        objects = Some(
                            decode_object_ids(d)
                                .map_err(|e| format!("decoding 'ObjectIds': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        }
        RemoveObjectsRequest {
            container_id: if let Some(__x) = container_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RemoveObjectsRequest.container_id (#0)".to_string(),
                ));
            },

            objects: if let Some(__x) = objects {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RemoveObjectsRequest.objects (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// The BlobStore service, provider side
/// wasmbus.contractId: wasmcloud:blobstore
/// wasmbus.providerReceive
#[async_trait]
pub trait Blobstore {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:blobstore"
    }
    /// Returns whether the container exists
    async fn container_exists(&self, ctx: &Context, arg: &ContainerId) -> RpcResult<bool>;
    /// Creates a container by name, returning success if it worked
    /// Note that container names may not be globally unique - just unique within the
    /// "namespace" of the connecting actor and linkdef
    async fn create_container(&self, ctx: &Context, arg: &ContainerId) -> RpcResult<()>;
    /// Retrieves information about the container.
    /// Returns error if the container id is invalid or not found.
    async fn get_container_info(
        &self,
        ctx: &Context,
        arg: &ContainerId,
    ) -> RpcResult<ContainerMetadata>;
    /// Returns list of container ids
    async fn list_containers(&self, ctx: &Context) -> RpcResult<ContainersInfo>;
    /// Empty and remove the container(s)
    /// The MultiResult list contains one entry for each container
    /// that was not successfully removed, with the 'key' value representing the container name.
    /// If the MultiResult list is empty, all container removals succeeded.
    async fn remove_containers(&self, ctx: &Context, arg: &ContainerIds) -> RpcResult<MultiResult>;
    /// Returns whether the object exists
    async fn object_exists(&self, ctx: &Context, arg: &ContainerObject) -> RpcResult<bool>;
    /// Retrieves information about the object.
    /// Returns error if the object id is invalid or not found.
    async fn get_object_info(
        &self,
        ctx: &Context,
        arg: &ContainerObject,
    ) -> RpcResult<ObjectMetadata>;
    /// Lists the objects in the container.
    /// If the container exists and is empty, the returned `objects` list is empty.
    /// Parameters of the request may be used to limit the object names returned
    /// with an optional start value, end value, and maximum number of items.
    /// The provider may limit the number of items returned. If the list is truncated,
    /// the response contains a `continuation` token that may be submitted in
    /// a subsequent ListObjects request.
    ///
    /// Optional object metadata fields (i.e., `contentType` and `contentEncoding`) may not be
    /// filled in for ListObjects response. To get complete object metadata, use GetObjectInfo.
    async fn list_objects(
        &self,
        ctx: &Context,
        arg: &ListObjectsRequest,
    ) -> RpcResult<ListObjectsResponse>;
    /// Removes the objects. In the event any of the objects cannot be removed,
    /// the operation continues until all requested deletions have been attempted.
    /// The MultiRequest includes a list of errors, one for each deletion request
    /// that did not succeed. If the list is empty, all removals succeeded.
    async fn remove_objects(
        &self,
        ctx: &Context,
        arg: &RemoveObjectsRequest,
    ) -> RpcResult<MultiResult>;
    /// Requests to start upload of a file/blob to the Blobstore.
    /// It is recommended to keep chunks under 1MB to avoid exceeding nats default message size
    async fn put_object(
        &self,
        ctx: &Context,
        arg: &PutObjectRequest,
    ) -> RpcResult<PutObjectResponse>;
    /// Requests to retrieve an object. If the object is large, the provider
    /// may split the response into multiple parts
    /// It is recommended to keep chunks under 1MB to avoid exceeding nats default message size
    async fn get_object(
        &self,
        ctx: &Context,
        arg: &GetObjectRequest,
    ) -> RpcResult<GetObjectResponse>;
    /// Uploads a file chunk to a blobstore. This must be called AFTER PutObject
    /// It is recommended to keep chunks under 1MB to avoid exceeding nats default message size
    async fn put_chunk(&self, ctx: &Context, arg: &PutChunkRequest) -> RpcResult<()>;
}

/// BlobstoreReceiver receives messages defined in the Blobstore service trait
/// The BlobStore service, provider side
#[doc(hidden)]
#[async_trait]
pub trait BlobstoreReceiver: MessageDispatch + Blobstore {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "ContainerExists" => {
                let value: ContainerId =
                    wasmbus_rpc::common::decode(&message.arg, &decode_container_id)
                        .map_err(|e| RpcError::Deser(format!("'ContainerId': {}", e)))?;
                let resp = Blobstore::container_exists(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                e.bool(resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.ContainerExists",
                    arg: Cow::Owned(buf),
                })
            }
            "CreateContainer" => {
                let value: ContainerId =
                    wasmbus_rpc::common::decode(&message.arg, &decode_container_id)
                        .map_err(|e| RpcError::Deser(format!("'ContainerId': {}", e)))?;
                let _resp = Blobstore::create_container(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Blobstore.CreateContainer",
                    arg: Cow::Owned(buf),
                })
            }
            "GetContainerInfo" => {
                let value: ContainerId =
                    wasmbus_rpc::common::decode(&message.arg, &decode_container_id)
                        .map_err(|e| RpcError::Deser(format!("'ContainerId': {}", e)))?;
                let resp = Blobstore::get_container_info(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_container_metadata(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.GetContainerInfo",
                    arg: Cow::Owned(buf),
                })
            }
            "ListContainers" => {
                let resp = Blobstore::list_containers(self, ctx).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_containers_info(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.ListContainers",
                    arg: Cow::Owned(buf),
                })
            }
            "RemoveContainers" => {
                let value: ContainerIds =
                    wasmbus_rpc::common::decode(&message.arg, &decode_container_ids)
                        .map_err(|e| RpcError::Deser(format!("'ContainerIds': {}", e)))?;
                let resp = Blobstore::remove_containers(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_multi_result(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.RemoveContainers",
                    arg: Cow::Owned(buf),
                })
            }
            "ObjectExists" => {
                let value: ContainerObject =
                    wasmbus_rpc::common::decode(&message.arg, &decode_container_object)
                        .map_err(|e| RpcError::Deser(format!("'ContainerObject': {}", e)))?;
                let resp = Blobstore::object_exists(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                e.bool(resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.ObjectExists",
                    arg: Cow::Owned(buf),
                })
            }
            "GetObjectInfo" => {
                let value: ContainerObject =
                    wasmbus_rpc::common::decode(&message.arg, &decode_container_object)
                        .map_err(|e| RpcError::Deser(format!("'ContainerObject': {}", e)))?;
                let resp = Blobstore::get_object_info(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_object_metadata(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.GetObjectInfo",
                    arg: Cow::Owned(buf),
                })
            }
            "ListObjects" => {
                let value: ListObjectsRequest =
                    wasmbus_rpc::common::decode(&message.arg, &decode_list_objects_request)
                        .map_err(|e| RpcError::Deser(format!("'ListObjectsRequest': {}", e)))?;
                let resp = Blobstore::list_objects(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_list_objects_response(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.ListObjects",
                    arg: Cow::Owned(buf),
                })
            }
            "RemoveObjects" => {
                let value: RemoveObjectsRequest =
                    wasmbus_rpc::common::decode(&message.arg, &decode_remove_objects_request)
                        .map_err(|e| RpcError::Deser(format!("'RemoveObjectsRequest': {}", e)))?;
                let resp = Blobstore::remove_objects(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_multi_result(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.RemoveObjects",
                    arg: Cow::Owned(buf),
                })
            }
            "PutObject" => {
                let value: PutObjectRequest =
                    wasmbus_rpc::common::decode(&message.arg, &decode_put_object_request)
                        .map_err(|e| RpcError::Deser(format!("'PutObjectRequest': {}", e)))?;
                let resp = Blobstore::put_object(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_put_object_response(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.PutObject",
                    arg: Cow::Owned(buf),
                })
            }
            "GetObject" => {
                let value: GetObjectRequest =
                    wasmbus_rpc::common::decode(&message.arg, &decode_get_object_request)
                        .map_err(|e| RpcError::Deser(format!("'GetObjectRequest': {}", e)))?;
                let resp = Blobstore::get_object(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_get_object_response(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "Blobstore.GetObject",
                    arg: Cow::Owned(buf),
                })
            }
            "PutChunk" => {
                let value: PutChunkRequest =
                    wasmbus_rpc::common::decode(&message.arg, &decode_put_chunk_request)
                        .map_err(|e| RpcError::Deser(format!("'PutChunkRequest': {}", e)))?;
                let _resp = Blobstore::put_chunk(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Blobstore.PutChunk",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Blobstore::{}",
                message.method
            ))),
        }
    }
}

/// BlobstoreSender sends messages to a Blobstore service
/// The BlobStore service, provider side
/// client for sending Blobstore messages
#[derive(Debug)]
pub struct BlobstoreSender<T: Transport> {
    transport: T,
}

impl<T: Transport> BlobstoreSender<T> {
    /// Constructs a BlobstoreSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl BlobstoreSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Blobstore provider
    /// implementing the 'wasmcloud:blobstore' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:blobstore", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Blobstore provider
    /// implementing the 'wasmcloud:blobstore' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:blobstore", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Blobstore for BlobstoreSender<T> {
    #[allow(unused)]
    /// Returns whether the container exists
    async fn container_exists(&self, ctx: &Context, arg: &ContainerId) -> RpcResult<bool> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_container_id(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.ContainerExists",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::decode(&resp, &decode_boolean)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Creates a container by name, returning success if it worked
    /// Note that container names may not be globally unique - just unique within the
    /// "namespace" of the connecting actor and linkdef
    async fn create_container(&self, ctx: &Context, arg: &ContainerId) -> RpcResult<()> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_container_id(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.CreateContainer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }

    #[allow(unused)]
    /// Retrieves information about the container.
    /// Returns error if the container id is invalid or not found.
    async fn get_container_info(
        &self,
        ctx: &Context,
        arg: &ContainerId,
    ) -> RpcResult<ContainerMetadata> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_container_id(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.GetContainerInfo",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ContainerMetadata =
            wasmbus_rpc::common::decode(&resp, &decode_container_metadata)
                .map_err(|e| RpcError::Deser(format!("'{}': ContainerMetadata", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Returns list of container ids
    async fn list_containers(&self, ctx: &Context) -> RpcResult<ContainersInfo> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.ListContainers",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ContainersInfo = wasmbus_rpc::common::decode(&resp, &decode_containers_info)
            .map_err(|e| RpcError::Deser(format!("'{}': ContainersInfo", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Empty and remove the container(s)
    /// The MultiResult list contains one entry for each container
    /// that was not successfully removed, with the 'key' value representing the container name.
    /// If the MultiResult list is empty, all container removals succeeded.
    async fn remove_containers(&self, ctx: &Context, arg: &ContainerIds) -> RpcResult<MultiResult> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_container_ids(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.RemoveContainers",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: MultiResult = wasmbus_rpc::common::decode(&resp, &decode_multi_result)
            .map_err(|e| RpcError::Deser(format!("'{}': MultiResult", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Returns whether the object exists
    async fn object_exists(&self, ctx: &Context, arg: &ContainerObject) -> RpcResult<bool> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_container_object(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.ObjectExists",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::decode(&resp, &decode_boolean)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Retrieves information about the object.
    /// Returns error if the object id is invalid or not found.
    async fn get_object_info(
        &self,
        ctx: &Context,
        arg: &ContainerObject,
    ) -> RpcResult<ObjectMetadata> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_container_object(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.GetObjectInfo",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ObjectMetadata = wasmbus_rpc::common::decode(&resp, &decode_object_metadata)
            .map_err(|e| RpcError::Deser(format!("'{}': ObjectMetadata", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Lists the objects in the container.
    /// If the container exists and is empty, the returned `objects` list is empty.
    /// Parameters of the request may be used to limit the object names returned
    /// with an optional start value, end value, and maximum number of items.
    /// The provider may limit the number of items returned. If the list is truncated,
    /// the response contains a `continuation` token that may be submitted in
    /// a subsequent ListObjects request.
    ///
    /// Optional object metadata fields (i.e., `contentType` and `contentEncoding`) may not be
    /// filled in for ListObjects response. To get complete object metadata, use GetObjectInfo.
    async fn list_objects(
        &self,
        ctx: &Context,
        arg: &ListObjectsRequest,
    ) -> RpcResult<ListObjectsResponse> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_list_objects_request(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.ListObjects",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ListObjectsResponse =
            wasmbus_rpc::common::decode(&resp, &decode_list_objects_response)
                .map_err(|e| RpcError::Deser(format!("'{}': ListObjectsResponse", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Removes the objects. In the event any of the objects cannot be removed,
    /// the operation continues until all requested deletions have been attempted.
    /// The MultiRequest includes a list of errors, one for each deletion request
    /// that did not succeed. If the list is empty, all removals succeeded.
    async fn remove_objects(
        &self,
        ctx: &Context,
        arg: &RemoveObjectsRequest,
    ) -> RpcResult<MultiResult> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_remove_objects_request(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.RemoveObjects",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: MultiResult = wasmbus_rpc::common::decode(&resp, &decode_multi_result)
            .map_err(|e| RpcError::Deser(format!("'{}': MultiResult", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests to start upload of a file/blob to the Blobstore.
    /// It is recommended to keep chunks under 1MB to avoid exceeding nats default message size
    async fn put_object(
        &self,
        ctx: &Context,
        arg: &PutObjectRequest,
    ) -> RpcResult<PutObjectResponse> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_put_object_request(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.PutObject",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: PutObjectResponse =
            wasmbus_rpc::common::decode(&resp, &decode_put_object_response)
                .map_err(|e| RpcError::Deser(format!("'{}': PutObjectResponse", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests to retrieve an object. If the object is large, the provider
    /// may split the response into multiple parts
    /// It is recommended to keep chunks under 1MB to avoid exceeding nats default message size
    async fn get_object(
        &self,
        ctx: &Context,
        arg: &GetObjectRequest,
    ) -> RpcResult<GetObjectResponse> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_get_object_request(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.GetObject",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: GetObjectResponse =
            wasmbus_rpc::common::decode(&resp, &decode_get_object_response)
                .map_err(|e| RpcError::Deser(format!("'{}': GetObjectResponse", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Uploads a file chunk to a blobstore. This must be called AFTER PutObject
    /// It is recommended to keep chunks under 1MB to avoid exceeding nats default message size
    async fn put_chunk(&self, ctx: &Context, arg: &PutChunkRequest) -> RpcResult<()> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_put_chunk_request(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.PutChunk",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
}

/// The BlobStore service, actor side
/// wasmbus.contractId: wasmcloud:blobstore
/// wasmbus.actorReceive
#[async_trait]
pub trait ChunkReceiver {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:blobstore"
    }
    /// Receives a file chunk from a blobstore.
    /// A blobstore provider invokes this operation on actors in response to the GetObject request.
    /// If the response sets cancelDownload, the provider will stop downloading chunks
    async fn receive_chunk(&self, ctx: &Context, arg: &Chunk) -> RpcResult<ChunkResponse>;
}

/// ChunkReceiverReceiver receives messages defined in the ChunkReceiver service trait
/// The BlobStore service, actor side
#[doc(hidden)]
#[async_trait]
pub trait ChunkReceiverReceiver: MessageDispatch + ChunkReceiver {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "ReceiveChunk" => {
                let value: Chunk = wasmbus_rpc::common::decode(&message.arg, &decode_chunk)
                    .map_err(|e| RpcError::Deser(format!("'Chunk': {}", e)))?;
                let resp = ChunkReceiver::receive_chunk(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_chunk_response(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "ChunkReceiver.ReceiveChunk",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "ChunkReceiver::{}",
                message.method
            ))),
        }
    }
}

/// ChunkReceiverSender sends messages to a ChunkReceiver service
/// The BlobStore service, actor side
/// client for sending ChunkReceiver messages
#[derive(Debug)]
pub struct ChunkReceiverSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ChunkReceiverSender<T> {
    /// Constructs a ChunkReceiverSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ChunkReceiverSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ChunkReceiverSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> ChunkReceiver
    for ChunkReceiverSender<T>
{
    #[allow(unused)]
    /// Receives a file chunk from a blobstore.
    /// A blobstore provider invokes this operation on actors in response to the GetObject request.
    /// If the response sets cancelDownload, the provider will stop downloading chunks
    async fn receive_chunk(&self, ctx: &Context, arg: &Chunk) -> RpcResult<ChunkResponse> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_chunk(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ChunkReceiver.ReceiveChunk",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ChunkResponse = wasmbus_rpc::common::decode(&resp, &decode_chunk_response)
            .map_err(|e| RpcError::Deser(format!("'{}': ChunkResponse", e)))?;
        Ok(value)
    }
}
