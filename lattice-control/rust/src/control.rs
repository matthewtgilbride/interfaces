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

/// One of a potential list of responses to an actor auction
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActorAuctionAck {
    /// The original actor reference used for the auction
    #[serde(default)]
    pub actor_ref: String,
    /// The host ID of the "bidder" for this auction.
    #[serde(default)]
    pub host_id: String,
}

// Encode ActorAuctionAck as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_auction_ack<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorAuctionAck,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("actorRef")?;
    e.str(&val.actor_ref)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    Ok(())
}

// Decode ActorAuctionAck from cbor input stream
#[doc(hidden)]
pub fn decode_actor_auction_ack(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorAuctionAck, RpcError> {
    let __result = {
        let mut actor_ref: Option<String> = None;
        let mut host_id: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ActorAuctionAck, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorAuctionAck: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_ref = Some(d.str()?.to_string()),
                    1 => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorAuctionAck: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorRef" => actor_ref = Some(d.str()?.to_string()),
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        ActorAuctionAck {
            actor_ref: if let Some(__x) = actor_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorAuctionAck.actor_ref (#0)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorAuctionAck.host_id (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type ActorAuctionAcks = Vec<ActorAuctionAck>;

// Encode ActorAuctionAcks as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_auction_acks<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorAuctionAcks,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_actor_auction_ack(e, item)?;
    }
    Ok(())
}

// Decode ActorAuctionAcks from cbor input stream
#[doc(hidden)]
pub fn decode_actor_auction_acks(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorAuctionAcks, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ActorAuctionAck> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_actor_auction_ack(d)
                        .map_err(|e| format!("decoding 'ActorAuctionAck': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ActorAuctionAck> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_actor_auction_ack(d)
                            .map_err(|e| format!("decoding 'ActorAuctionAck': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// A request to locate suitable hosts for a given actor
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActorAuctionRequest {
    /// The reference for this actor. Can be any one of the acceptable forms
    /// of uniquely identifying an actor.
    #[serde(default)]
    pub actor_ref: String,
    /// The set of constraints to which any candidate host must conform
    pub constraints: ConstraintMap,
}

// Encode ActorAuctionRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_auction_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorAuctionRequest,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("actorRef")?;
    e.str(&val.actor_ref)?;
    e.str("constraints")?;
    encode_constraint_map(e, &val.constraints)?;
    Ok(())
}

// Decode ActorAuctionRequest from cbor input stream
#[doc(hidden)]
pub fn decode_actor_auction_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorAuctionRequest, RpcError> {
    let __result = {
        let mut actor_ref: Option<String> = None;
        let mut constraints: Option<ConstraintMap> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ActorAuctionRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorAuctionRequest: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_ref = Some(d.str()?.to_string()),
                    1 => {
                        constraints = Some(
                            decode_constraint_map(d)
                                .map_err(|e| format!("decoding 'ConstraintMap': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorAuctionRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorRef" => actor_ref = Some(d.str()?.to_string()),
                    "constraints" => {
                        constraints = Some(
                            decode_constraint_map(d)
                                .map_err(|e| format!("decoding 'ConstraintMap': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        }
        ActorAuctionRequest {
            actor_ref: if let Some(__x) = actor_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorAuctionRequest.actor_ref (#0)".to_string(),
                ));
            },

            constraints: if let Some(__x) = constraints {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorAuctionRequest.constraints (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A summary description of an actor within a host inventory
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActorDescription {
    /// Actor's 56-character unique ID
    #[serde(default)]
    pub id: String,
    /// Image reference for this actor, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// The individual instances of this actor that are running
    pub instances: ActorInstances,
    /// Name of this actor, if one exists
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// Encode ActorDescription as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_description<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorDescription,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("id")?;
    e.str(&val.id)?;
    if let Some(val) = val.image_ref.as_ref() {
        e.str("imageRef")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("instances")?;
    encode_actor_instances(e, &val.instances)?;
    if let Some(val) = val.name.as_ref() {
        e.str("name")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ActorDescription from cbor input stream
#[doc(hidden)]
pub fn decode_actor_description(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorDescription, RpcError> {
    let __result = {
        let mut id: Option<String> = None;
        let mut image_ref: Option<Option<String>> = Some(None);
        let mut instances: Option<ActorInstances> = None;
        let mut name: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ActorDescription, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorDescription: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.str()?.to_string()),
                    1 => {
                        image_ref = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => {
                        instances = Some(
                            decode_actor_instances(d)
                                .map_err(|e| format!("decoding 'ActorInstances': {}", e))?,
                        )
                    }
                    3 => {
                        name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
                    "decoding struct ActorDescription: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.str()?.to_string()),
                    "imageRef" => {
                        image_ref = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "instances" => {
                        instances = Some(
                            decode_actor_instances(d)
                                .map_err(|e| format!("decoding 'ActorInstances': {}", e))?,
                        )
                    }
                    "name" => {
                        name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
        ActorDescription {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorDescription.id (#0)".to_string(),
                ));
            },
            image_ref: image_ref.unwrap(),

            instances: if let Some(__x) = instances {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorDescription.instances (#2)".to_string(),
                ));
            },
            name: name.unwrap(),
        }
    };
    Ok(__result)
}
pub type ActorDescriptions = Vec<ActorDescription>;

// Encode ActorDescriptions as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_descriptions<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorDescriptions,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_actor_description(e, item)?;
    }
    Ok(())
}

// Decode ActorDescriptions from cbor input stream
#[doc(hidden)]
pub fn decode_actor_descriptions(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorDescriptions, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ActorDescription> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_actor_description(d)
                        .map_err(|e| format!("decoding 'ActorDescription': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ActorDescription> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_actor_description(d)
                            .map_err(|e| format!("decoding 'ActorDescription': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActorInstance {
    /// The annotations that were used in the start request that produced
    /// this actor instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// This instance's unique ID (guid)
    #[serde(default)]
    pub instance_id: String,
    /// The revision number for this actor instance
    #[serde(default)]
    pub revision: i32,
}

// Encode ActorInstance as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_instance<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorInstance,
) -> RpcResult<()> {
    e.map(3)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    e.str("instanceId")?;
    e.str(&val.instance_id)?;
    e.str("revision")?;
    e.i32(val.revision)?;
    Ok(())
}

// Decode ActorInstance from cbor input stream
#[doc(hidden)]
pub fn decode_actor_instance(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorInstance, RpcError> {
    let __result = {
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut instance_id: Option<String> = None;
        let mut revision: Option<i32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ActorInstance, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorInstance: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    1 => instance_id = Some(d.str()?.to_string()),
                    2 => revision = Some(d.i32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ActorInstance: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "instanceId" => instance_id = Some(d.str()?.to_string()),
                    "revision" => revision = Some(d.i32()?),
                    _ => d.skip()?,
                }
            }
        }
        ActorInstance {
            annotations: annotations.unwrap(),

            instance_id: if let Some(__x) = instance_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorInstance.instance_id (#1)".to_string(),
                ));
            },

            revision: if let Some(__x) = revision {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ActorInstance.revision (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type ActorInstances = Vec<ActorInstance>;

// Encode ActorInstances as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_actor_instances<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ActorInstances,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_actor_instance(e, item)?;
    }
    Ok(())
}

// Decode ActorInstances from cbor input stream
#[doc(hidden)]
pub fn decode_actor_instances(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ActorInstances, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ActorInstance> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_actor_instance(d)
                        .map_err(|e| format!("decoding 'ActorInstance': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ActorInstance> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_actor_instance(d)
                            .map_err(|e| format!("decoding 'ActorInstance': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
pub type AnnotationMap = std::collections::HashMap<String, String>;

// Encode AnnotationMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_annotation_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &AnnotationMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        e.str(v)?;
    }
    Ok(())
}

// Decode AnnotationMap from cbor input stream
#[doc(hidden)]
pub fn decode_annotation_map(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<AnnotationMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, String> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = d.str()?.to_string();
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
pub type ClaimsList = Vec<ClaimsMap>;

// Encode ClaimsList as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_claims_list<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ClaimsList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_claims_map(e, item)?;
    }
    Ok(())
}

// Decode ClaimsList from cbor input stream
#[doc(hidden)]
pub fn decode_claims_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ClaimsList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ClaimsMap> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_claims_map(d).map_err(|e| format!("decoding 'ClaimsMap': {}", e))?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ClaimsMap> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_claims_map(d).map_err(|e| format!("decoding 'ClaimsMap': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
pub type ClaimsMap = std::collections::HashMap<String, String>;

// Encode ClaimsMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_claims_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ClaimsMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        e.str(v)?;
    }
    Ok(())
}

// Decode ClaimsMap from cbor input stream
#[doc(hidden)]
pub fn decode_claims_map(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ClaimsMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, String> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = d.str()?.to_string();
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
pub type ConfigurationString = String;

// Encode ConfigurationString as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_configuration_string<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ConfigurationString,
) -> RpcResult<()> {
    e.str(val)?;
    Ok(())
}

// Decode ConfigurationString from cbor input stream
#[doc(hidden)]
pub fn decode_configuration_string(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ConfigurationString, RpcError> {
    let __result = { d.str()?.to_string() };
    Ok(__result)
}
pub type ConstraintMap = std::collections::HashMap<String, String>;

// Encode ConstraintMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_constraint_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ConstraintMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        e.str(v)?;
    }
    Ok(())
}

// Decode ConstraintMap from cbor input stream
#[doc(hidden)]
pub fn decode_constraint_map(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ConstraintMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, String> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = d.str()?.to_string();
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
/// Standard response for control interface operations
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CtlOperationAck {
    #[serde(default)]
    pub accepted: bool,
    #[serde(default)]
    pub error: String,
}

// Encode CtlOperationAck as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_ctl_operation_ack<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CtlOperationAck,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("accepted")?;
    e.bool(val.accepted)?;
    e.str("error")?;
    e.str(&val.error)?;
    Ok(())
}

// Decode CtlOperationAck from cbor input stream
#[doc(hidden)]
pub fn decode_ctl_operation_ack(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CtlOperationAck, RpcError> {
    let __result = {
        let mut accepted: Option<bool> = None;
        let mut error: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct CtlOperationAck, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct CtlOperationAck: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => accepted = Some(d.bool()?),
                    1 => error = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct CtlOperationAck: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "accepted" => accepted = Some(d.bool()?),
                    "error" => error = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        CtlOperationAck {
            accepted: if let Some(__x) = accepted {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CtlOperationAck.accepted (#0)".to_string(),
                ));
            },

            error: if let Some(__x) = error {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CtlOperationAck.error (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A response containing the full list of known claims within the lattice
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetClaimsResponse {
    pub claims: ClaimsList,
}

// Encode GetClaimsResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_get_claims_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &GetClaimsResponse,
) -> RpcResult<()> {
    e.map(1)?;
    e.str("claims")?;
    encode_claims_list(e, &val.claims)?;
    Ok(())
}

// Decode GetClaimsResponse from cbor input stream
#[doc(hidden)]
pub fn decode_get_claims_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<GetClaimsResponse, RpcError> {
    let __result = {
        let mut claims: Option<ClaimsList> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct GetClaimsResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetClaimsResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        claims = Some(
                            decode_claims_list(d)
                                .map_err(|e| format!("decoding 'ClaimsList': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetClaimsResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "claims" => {
                        claims = Some(
                            decode_claims_list(d)
                                .map_err(|e| format!("decoding 'ClaimsList': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        }
        GetClaimsResponse {
            claims: if let Some(__x) = claims {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetClaimsResponse.claims (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A summary representation of a host
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Host {
    #[serde(default)]
    pub id: String,
    /// uptime in seconds
    #[serde(default)]
    pub uptime_seconds: u64,
}

// Encode Host as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_host<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Host,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("id")?;
    e.str(&val.id)?;
    e.str("uptimeSeconds")?;
    e.u64(val.uptime_seconds)?;
    Ok(())
}

// Decode Host from cbor input stream
#[doc(hidden)]
pub fn decode_host(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Host, RpcError> {
    let __result = {
        let mut id: Option<String> = None;
        let mut uptime_seconds: Option<u64> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Host, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser("decoding struct Host: indefinite array not supported".to_string())
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.str()?.to_string()),
                    1 => uptime_seconds = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser("decoding struct Host: indefinite map not supported".to_string())
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.str()?.to_string()),
                    "uptimeSeconds" => uptime_seconds = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        }
        Host {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field Host.id (#0)".to_string()));
            },

            uptime_seconds: if let Some(__x) = uptime_seconds {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Host.uptime_seconds (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Describes the known contents of a given host at the time of
/// a query
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostInventory {
    /// Actors running on this host.
    pub actors: ActorDescriptions,
    /// The host's unique ID
    #[serde(default)]
    pub host_id: String,
    /// The host's labels
    pub labels: LabelsMap,
    /// Providers running on this host
    pub providers: ProviderDescriptions,
}

// Encode HostInventory as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_host_inventory<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HostInventory,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("actors")?;
    encode_actor_descriptions(e, &val.actors)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    e.str("labels")?;
    encode_labels_map(e, &val.labels)?;
    e.str("providers")?;
    encode_provider_descriptions(e, &val.providers)?;
    Ok(())
}

// Decode HostInventory from cbor input stream
#[doc(hidden)]
pub fn decode_host_inventory(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<HostInventory, RpcError> {
    let __result = {
        let mut actors: Option<ActorDescriptions> = None;
        let mut host_id: Option<String> = None;
        let mut labels: Option<LabelsMap> = None;
        let mut providers: Option<ProviderDescriptions> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct HostInventory, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HostInventory: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        actors = Some(
                            decode_actor_descriptions(d)
                                .map_err(|e| format!("decoding 'ActorDescriptions': {}", e))?,
                        )
                    }
                    1 => host_id = Some(d.str()?.to_string()),
                    2 => {
                        labels = Some(
                            decode_labels_map(d)
                                .map_err(|e| format!("decoding 'LabelsMap': {}", e))?,
                        )
                    }
                    3 => {
                        providers = Some(
                            decode_provider_descriptions(d)
                                .map_err(|e| format!("decoding 'ProviderDescriptions': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HostInventory: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actors" => {
                        actors = Some(
                            decode_actor_descriptions(d)
                                .map_err(|e| format!("decoding 'ActorDescriptions': {}", e))?,
                        )
                    }
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "labels" => {
                        labels = Some(
                            decode_labels_map(d)
                                .map_err(|e| format!("decoding 'LabelsMap': {}", e))?,
                        )
                    }
                    "providers" => {
                        providers = Some(
                            decode_provider_descriptions(d)
                                .map_err(|e| format!("decoding 'ProviderDescriptions': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        }
        HostInventory {
            actors: if let Some(__x) = actors {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostInventory.actors (#0)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostInventory.host_id (#1)".to_string(),
                ));
            },

            labels: if let Some(__x) = labels {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostInventory.labels (#2)".to_string(),
                ));
            },

            providers: if let Some(__x) = providers {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostInventory.providers (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type Hosts = Vec<Host>;

// Encode Hosts as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_hosts<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Hosts,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_host(e, item)?;
    }
    Ok(())
}

// Decode Hosts from cbor input stream
#[doc(hidden)]
pub fn decode_hosts(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Hosts, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Host> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_host(d).map_err(|e| format!("decoding 'Host': {}", e))?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Host> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => {
                        arr.push(decode_host(d).map_err(|e| format!("decoding 'Host': {}", e))?)
                    }
                }
            }
            arr
        }
    };
    Ok(__result)
}
pub type LabelsMap = std::collections::HashMap<String, String>;

// Encode LabelsMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_labels_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &LabelsMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        e.str(v)?;
    }
    Ok(())
}

// Decode LabelsMap from cbor input stream
#[doc(hidden)]
pub fn decode_labels_map(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<LabelsMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, String> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = d.str()?.to_string();
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
/// A list of link definitions
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LinkDefinitionList {
    pub links: wasmbus_rpc::core::ActorLinks,
}

// Encode LinkDefinitionList as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_link_definition_list<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &LinkDefinitionList,
) -> RpcResult<()> {
    e.map(1)?;
    e.str("links")?;
    wasmbus_rpc::core::encode_actor_links(e, &val.links)?;
    Ok(())
}

// Decode LinkDefinitionList from cbor input stream
#[doc(hidden)]
pub fn decode_link_definition_list(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<LinkDefinitionList, RpcError> {
    let __result = {
        let mut links: Option<wasmbus_rpc::core::ActorLinks> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct LinkDefinitionList, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct LinkDefinitionList: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        links = Some(wasmbus_rpc::core::decode_actor_links(d).map_err(|e| {
                            format!("decoding 'wasmbus_rpc::core::ActorLinks': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct LinkDefinitionList: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "links" => {
                        links = Some(wasmbus_rpc::core::decode_actor_links(d).map_err(|e| {
                            format!("decoding 'wasmbus_rpc::core::ActorLinks': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        LinkDefinitionList {
            links: if let Some(__x) = links {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LinkDefinitionList.links (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// One of a potential list of responses to a provider auction
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderAuctionAck {
    /// The host ID of the "bidder" for this auction
    #[serde(default)]
    pub host_id: String,
    /// The link name provided for the auction
    #[serde(default)]
    pub link_name: String,
    /// The original provider ref provided for the auction
    #[serde(default)]
    pub provider_ref: String,
}

// Encode ProviderAuctionAck as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_provider_auction_ack<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ProviderAuctionAck,
) -> RpcResult<()> {
    e.map(3)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    e.str("linkName")?;
    e.str(&val.link_name)?;
    e.str("providerRef")?;
    e.str(&val.provider_ref)?;
    Ok(())
}

// Decode ProviderAuctionAck from cbor input stream
#[doc(hidden)]
pub fn decode_provider_auction_ack(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ProviderAuctionAck, RpcError> {
    let __result = {
        let mut host_id: Option<String> = None;
        let mut link_name: Option<String> = None;
        let mut provider_ref: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ProviderAuctionAck, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ProviderAuctionAck: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => host_id = Some(d.str()?.to_string()),
                    1 => link_name = Some(d.str()?.to_string()),
                    2 => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ProviderAuctionAck: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "providerRef" => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        ProviderAuctionAck {
            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderAuctionAck.host_id (#0)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderAuctionAck.link_name (#1)".to_string(),
                ));
            },

            provider_ref: if let Some(__x) = provider_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderAuctionAck.provider_ref (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type ProviderAuctionAcks = Vec<ProviderAuctionAck>;

// Encode ProviderAuctionAcks as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_provider_auction_acks<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ProviderAuctionAcks,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_provider_auction_ack(e, item)?;
    }
    Ok(())
}

// Decode ProviderAuctionAcks from cbor input stream
#[doc(hidden)]
pub fn decode_provider_auction_acks(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ProviderAuctionAcks, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ProviderAuctionAck> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_provider_auction_ack(d)
                        .map_err(|e| format!("decoding 'ProviderAuctionAck': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ProviderAuctionAck> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_provider_auction_ack(d)
                            .map_err(|e| format!("decoding 'ProviderAuctionAck': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// A request to locate a suitable host for a capability provider. The
/// provider's unique identity (reference + link name) is used to rule
/// out sites on which the provider is already running.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderAuctionRequest {
    /// The set of constraints to which a suitable target host must conform
    pub constraints: ConstraintMap,
    /// The link name of the provider
    #[serde(default)]
    pub link_name: String,
    /// The reference for the provider. Can be any one of the accepted
    /// forms of uniquely identifying a provider
    #[serde(default)]
    pub provider_ref: String,
}

// Encode ProviderAuctionRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_provider_auction_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ProviderAuctionRequest,
) -> RpcResult<()> {
    e.map(3)?;
    e.str("constraints")?;
    encode_constraint_map(e, &val.constraints)?;
    e.str("linkName")?;
    e.str(&val.link_name)?;
    e.str("providerRef")?;
    e.str(&val.provider_ref)?;
    Ok(())
}

// Decode ProviderAuctionRequest from cbor input stream
#[doc(hidden)]
pub fn decode_provider_auction_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ProviderAuctionRequest, RpcError> {
    let __result = {
        let mut constraints: Option<ConstraintMap> = None;
        let mut link_name: Option<String> = None;
        let mut provider_ref: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ProviderAuctionRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ProviderAuctionRequest: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        constraints = Some(
                            decode_constraint_map(d)
                                .map_err(|e| format!("decoding 'ConstraintMap': {}", e))?,
                        )
                    }
                    1 => link_name = Some(d.str()?.to_string()),
                    2 => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ProviderAuctionRequest: indefinite map not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "constraints" => {
                        constraints = Some(
                            decode_constraint_map(d)
                                .map_err(|e| format!("decoding 'ConstraintMap': {}", e))?,
                        )
                    }
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "providerRef" => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        ProviderAuctionRequest {
            constraints: if let Some(__x) = constraints {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderAuctionRequest.constraints (#0)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderAuctionRequest.link_name (#1)".to_string(),
                ));
            },

            provider_ref: if let Some(__x) = provider_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderAuctionRequest.provider_ref (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A summary description of a capability provider within a host inventory
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderDescription {
    /// Provider's unique 56-character ID
    #[serde(default)]
    pub id: String,
    /// Image reference for this provider, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Provider's link name
    #[serde(default)]
    pub link_name: String,
    /// Name of the provider, if one exists
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The revision of the provider
    #[serde(default)]
    pub revision: i32,
}

// Encode ProviderDescription as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_provider_description<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ProviderDescription,
) -> RpcResult<()> {
    e.map(5)?;
    e.str("id")?;
    e.str(&val.id)?;
    if let Some(val) = val.image_ref.as_ref() {
        e.str("imageRef")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("linkName")?;
    e.str(&val.link_name)?;
    if let Some(val) = val.name.as_ref() {
        e.str("name")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("revision")?;
    e.i32(val.revision)?;
    Ok(())
}

// Decode ProviderDescription from cbor input stream
#[doc(hidden)]
pub fn decode_provider_description(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ProviderDescription, RpcError> {
    let __result = {
        let mut id: Option<String> = None;
        let mut image_ref: Option<Option<String>> = Some(None);
        let mut link_name: Option<String> = None;
        let mut name: Option<Option<String>> = Some(None);
        let mut revision: Option<i32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ProviderDescription, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ProviderDescription: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.str()?.to_string()),
                    1 => {
                        image_ref = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => link_name = Some(d.str()?.to_string()),
                    3 => {
                        name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    4 => revision = Some(d.i32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ProviderDescription: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.str()?.to_string()),
                    "imageRef" => {
                        image_ref = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "name" => {
                        name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "revision" => revision = Some(d.i32()?),
                    _ => d.skip()?,
                }
            }
        }
        ProviderDescription {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderDescription.id (#0)".to_string(),
                ));
            },
            image_ref: image_ref.unwrap(),

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderDescription.link_name (#2)".to_string(),
                ));
            },
            name: name.unwrap(),

            revision: if let Some(__x) = revision {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProviderDescription.revision (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type ProviderDescriptions = Vec<ProviderDescription>;

// Encode ProviderDescriptions as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_provider_descriptions<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ProviderDescriptions,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_provider_description(e, item)?;
    }
    Ok(())
}

// Decode ProviderDescriptions from cbor input stream
#[doc(hidden)]
pub fn decode_provider_descriptions(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ProviderDescriptions, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ProviderDescription> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_provider_description(d)
                        .map_err(|e| format!("decoding 'ProviderDescription': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ProviderDescription> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_provider_description(d)
                            .map_err(|e| format!("decoding 'ProviderDescription': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistryCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// If supplied, token authentication will be used for the registry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// If supplied, username and password will be used for HTTP Basic authentication
    #[serde(rename = "userName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

// Encode RegistryCredential as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_registry_credential<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RegistryCredential,
) -> RpcResult<()> {
    e.map(3)?;
    if let Some(val) = val.password.as_ref() {
        e.str("password")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.token.as_ref() {
        e.str("token")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.user_name.as_ref() {
        e.str("userName")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode RegistryCredential from cbor input stream
#[doc(hidden)]
pub fn decode_registry_credential(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RegistryCredential, RpcError> {
    let __result = {
        let mut password: Option<Option<String>> = Some(None);
        let mut token: Option<Option<String>> = Some(None);
        let mut user_name: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RegistryCredential, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct RegistryCredential: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        password = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        token = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => {
                        user_name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
                    "decoding struct RegistryCredential: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "password" => {
                        password = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "token" => {
                        token = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "userName" => {
                        user_name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
        RegistryCredential {
            password: password.unwrap(),
            token: token.unwrap(),
            user_name: user_name.unwrap(),
        }
    };
    Ok(__result)
}
/// A set of credentials to be used for fetching from specific registries
pub type RegistryCredentialMap = std::collections::HashMap<String, RegistryCredential>;

// Encode RegistryCredentialMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_registry_credential_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RegistryCredentialMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        encode_registry_credential(e, v)?;
    }
    Ok(())
}

// Decode RegistryCredentialMap from cbor input stream
#[doc(hidden)]
pub fn decode_registry_credential_map(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RegistryCredentialMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, RegistryCredential> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = decode_registry_credential(d)
                        .map_err(|e| format!("decoding 'RegistryCredential': {}", e))?;
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
/// A request to remove a link definition and detach the relevant actor
/// from the given provider
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RemoveLinkDefinitionRequest {
    /// The actor's public key. This cannot be an image reference
    #[serde(default)]
    pub actor_id: String,
    /// The provider contract
    #[serde(default)]
    pub contract_id: String,
    /// The provider's link name
    #[serde(default)]
    pub link_name: String,
}

// Encode RemoveLinkDefinitionRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_remove_link_definition_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RemoveLinkDefinitionRequest,
) -> RpcResult<()> {
    e.map(3)?;
    e.str("actorId")?;
    e.str(&val.actor_id)?;
    e.str("contractId")?;
    e.str(&val.contract_id)?;
    e.str("linkName")?;
    e.str(&val.link_name)?;
    Ok(())
}

// Decode RemoveLinkDefinitionRequest from cbor input stream
#[doc(hidden)]
pub fn decode_remove_link_definition_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RemoveLinkDefinitionRequest, RpcError> {
    let __result = {
        let mut actor_id: Option<String> = None;
        let mut contract_id: Option<String> = None;
        let mut link_name: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RemoveLinkDefinitionRequest, expected array or map"
                        .to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct RemoveLinkDefinitionRequest: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_id = Some(d.str()?.to_string()),
                    1 => contract_id = Some(d.str()?.to_string()),
                    2 => link_name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct RemoveLinkDefinitionRequest: indefinite map not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorId" => actor_id = Some(d.str()?.to_string()),
                    "contractId" => contract_id = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        RemoveLinkDefinitionRequest {
            actor_id: if let Some(__x) = actor_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RemoveLinkDefinitionRequest.actor_id (#0)".to_string(),
                ));
            },

            contract_id: if let Some(__x) = contract_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RemoveLinkDefinitionRequest.contract_id (#1)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RemoveLinkDefinitionRequest.link_name (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScaleActorCommand {
    /// Public Key ID of the actor to scale
    #[serde(default)]
    pub actor_id: String,
    /// Reference for the actor. Can be any of the acceptable forms of unique identification
    #[serde(default)]
    pub actor_ref: String,
    /// Optional set of annotations used to describe the nature of this actor scale command. For
    /// example, autonomous agents may wish to "tag" scale requests as part of a given deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// The target number of actors
    #[serde(default)]
    pub count: u16,
    /// Host ID on which to scale this actor
    #[serde(default)]
    pub host_id: String,
}

// Encode ScaleActorCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_scale_actor_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ScaleActorCommand,
) -> RpcResult<()> {
    e.map(5)?;
    e.str("actorId")?;
    e.str(&val.actor_id)?;
    e.str("actorRef")?;
    e.str(&val.actor_ref)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    e.str("count")?;
    e.u16(val.count)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    Ok(())
}

// Decode ScaleActorCommand from cbor input stream
#[doc(hidden)]
pub fn decode_scale_actor_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ScaleActorCommand, RpcError> {
    let __result = {
        let mut actor_id: Option<String> = None;
        let mut actor_ref: Option<String> = None;
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut count: Option<u16> = None;
        let mut host_id: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ScaleActorCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ScaleActorCommand: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_id = Some(d.str()?.to_string()),
                    1 => actor_ref = Some(d.str()?.to_string()),
                    2 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    3 => count = Some(d.u16()?),
                    4 => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ScaleActorCommand: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorId" => actor_id = Some(d.str()?.to_string()),
                    "actorRef" => actor_ref = Some(d.str()?.to_string()),
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "count" => count = Some(d.u16()?),
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        ScaleActorCommand {
            actor_id: if let Some(__x) = actor_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ScaleActorCommand.actor_id (#0)".to_string(),
                ));
            },

            actor_ref: if let Some(__x) = actor_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ScaleActorCommand.actor_ref (#1)".to_string(),
                ));
            },
            annotations: annotations.unwrap(),

            count: if let Some(__x) = count {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ScaleActorCommand.count (#3)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ScaleActorCommand.host_id (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A command sent to a specific host instructing it to start the actor
/// indicated by the reference.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartActorCommand {
    /// Reference for the actor. Can be any of the acceptable forms of unique identification
    #[serde(default)]
    pub actor_ref: String,
    /// Optional set of annotations used to describe the nature of this actor start command. For
    /// example, autonomous agents may wish to "tag" start requests as part of a given deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// The number of actors to start
    /// A zero value will be interpreted as 1.
    #[serde(default)]
    pub count: u16,
    /// Host ID on which this actor should start
    #[serde(default)]
    pub host_id: String,
}

// Encode StartActorCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_start_actor_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StartActorCommand,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("actorRef")?;
    e.str(&val.actor_ref)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    e.str("count")?;
    e.u16(val.count)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    Ok(())
}

// Decode StartActorCommand from cbor input stream
#[doc(hidden)]
pub fn decode_start_actor_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StartActorCommand, RpcError> {
    let __result = {
        let mut actor_ref: Option<String> = None;
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut count: Option<u16> = None;
        let mut host_id: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StartActorCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StartActorCommand: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_ref = Some(d.str()?.to_string()),
                    1 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    2 => count = Some(d.u16()?),
                    3 => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StartActorCommand: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorRef" => actor_ref = Some(d.str()?.to_string()),
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "count" => count = Some(d.u16()?),
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        StartActorCommand {
            actor_ref: if let Some(__x) = actor_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartActorCommand.actor_ref (#0)".to_string(),
                ));
            },
            annotations: annotations.unwrap(),

            count: if let Some(__x) = count {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartActorCommand.count (#2)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartActorCommand.host_id (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A command sent to a host requesting a capability provider be started with the
/// given link name and optional configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartProviderCommand {
    /// Optional set of annotations used to describe the nature of this provider start command. For
    /// example, autonomous agents may wish to "tag" start requests as part of a given deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// Optional provider configuration in the form of an opaque string. Many
    /// providers prefer base64-encoded JSON here, though that data should never
    /// exceed 500KB
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationString>,
    /// The host ID on which to start the provider
    #[serde(default)]
    pub host_id: String,
    /// The link name of the provider to be started
    #[serde(default)]
    pub link_name: String,
    /// The image reference of the provider to be started
    #[serde(default)]
    pub provider_ref: String,
}

// Encode StartProviderCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_start_provider_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StartProviderCommand,
) -> RpcResult<()> {
    e.map(5)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.configuration.as_ref() {
        e.str("configuration")?;
        encode_configuration_string(e, val)?;
    } else {
        e.null()?;
    }
    e.str("hostId")?;
    e.str(&val.host_id)?;
    e.str("linkName")?;
    e.str(&val.link_name)?;
    e.str("providerRef")?;
    e.str(&val.provider_ref)?;
    Ok(())
}

// Decode StartProviderCommand from cbor input stream
#[doc(hidden)]
pub fn decode_start_provider_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StartProviderCommand, RpcError> {
    let __result = {
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut configuration: Option<Option<ConfigurationString>> = Some(None);
        let mut host_id: Option<String> = None;
        let mut link_name: Option<String> = None;
        let mut provider_ref: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StartProviderCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StartProviderCommand: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    1 => {
                        configuration = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_configuration_string(d).map_err(|e| {
                                format!("decoding 'ConfigurationString': {}", e)
                            })?))
                        }
                    }
                    2 => host_id = Some(d.str()?.to_string()),
                    3 => link_name = Some(d.str()?.to_string()),
                    4 => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StartProviderCommand: indefinite map not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "configuration" => {
                        configuration = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_configuration_string(d).map_err(|e| {
                                format!("decoding 'ConfigurationString': {}", e)
                            })?))
                        }
                    }
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "providerRef" => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        StartProviderCommand {
            annotations: annotations.unwrap(),
            configuration: configuration.unwrap(),

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartProviderCommand.host_id (#2)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartProviderCommand.link_name (#3)".to_string(),
                ));
            },

            provider_ref: if let Some(__x) = provider_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartProviderCommand.provider_ref (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A command sent to a host to request that instances of a given actor
/// be terminated on that host
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StopActorCommand {
    /// Reference for this actor. Can be any of the means of uniquely identifying
    /// an actor
    #[serde(default)]
    pub actor_ref: String,
    /// Optional set of annotations used to describe the nature of this
    /// stop request. If supplied, the only instances of this actor with these
    /// annotations will be stopped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// The number of actors to stop
    /// A zero value means stop all actors
    #[serde(default)]
    pub count: u16,
    /// The ID of the target host
    #[serde(default)]
    pub host_id: String,
}

// Encode StopActorCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_stop_actor_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StopActorCommand,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("actorRef")?;
    e.str(&val.actor_ref)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    e.str("count")?;
    e.u16(val.count)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    Ok(())
}

// Decode StopActorCommand from cbor input stream
#[doc(hidden)]
pub fn decode_stop_actor_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StopActorCommand, RpcError> {
    let __result = {
        let mut actor_ref: Option<String> = None;
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut count: Option<u16> = None;
        let mut host_id: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StopActorCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StopActorCommand: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_ref = Some(d.str()?.to_string()),
                    1 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    2 => count = Some(d.u16()?),
                    3 => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StopActorCommand: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorRef" => actor_ref = Some(d.str()?.to_string()),
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "count" => count = Some(d.u16()?),
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        StopActorCommand {
            actor_ref: if let Some(__x) = actor_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopActorCommand.actor_ref (#0)".to_string(),
                ));
            },
            annotations: annotations.unwrap(),

            count: if let Some(__x) = count {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopActorCommand.count (#2)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopActorCommand.host_id (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A command sent to request that the given host purge and stop
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StopHostCommand {
    /// The ID of the target host
    #[serde(default)]
    pub host_id: String,
    /// An optional timeout, in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

// Encode StopHostCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_stop_host_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StopHostCommand,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    if let Some(val) = val.timeout.as_ref() {
        e.str("timeout")?;
        e.u64(*val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode StopHostCommand from cbor input stream
#[doc(hidden)]
pub fn decode_stop_host_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StopHostCommand, RpcError> {
    let __result = {
        let mut host_id: Option<String> = None;
        let mut timeout: Option<Option<u64>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StopHostCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StopHostCommand: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => host_id = Some(d.str()?.to_string()),
                    1 => {
                        timeout = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
                    "decoding struct StopHostCommand: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "timeout" => {
                        timeout = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
        StopHostCommand {
            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopHostCommand.host_id (#0)".to_string(),
                ));
            },
            timeout: timeout.unwrap(),
        }
    };
    Ok(__result)
}
/// A request to stop the given provider on the indicated host
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StopProviderCommand {
    /// Optional set of annotations used to describe the nature of this
    /// stop request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// Contract ID of the capability provider
    #[serde(default)]
    pub contract_id: String,
    /// Host ID on which to stop the provider
    #[serde(default)]
    pub host_id: String,
    /// Link name for this provider
    #[serde(default)]
    pub link_name: String,
    /// Reference for the capability provider. Can be any of the forms of
    /// uniquely identifying a provider
    #[serde(default)]
    pub provider_ref: String,
}

// Encode StopProviderCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_stop_provider_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StopProviderCommand,
) -> RpcResult<()> {
    e.map(5)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    e.str("contractId")?;
    e.str(&val.contract_id)?;
    e.str("hostId")?;
    e.str(&val.host_id)?;
    e.str("linkName")?;
    e.str(&val.link_name)?;
    e.str("providerRef")?;
    e.str(&val.provider_ref)?;
    Ok(())
}

// Decode StopProviderCommand from cbor input stream
#[doc(hidden)]
pub fn decode_stop_provider_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StopProviderCommand, RpcError> {
    let __result = {
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut contract_id: Option<String> = None;
        let mut host_id: Option<String> = None;
        let mut link_name: Option<String> = None;
        let mut provider_ref: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StopProviderCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StopProviderCommand: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    1 => contract_id = Some(d.str()?.to_string()),
                    2 => host_id = Some(d.str()?.to_string()),
                    3 => link_name = Some(d.str()?.to_string()),
                    4 => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct StopProviderCommand: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "contractId" => contract_id = Some(d.str()?.to_string()),
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "providerRef" => provider_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        StopProviderCommand {
            annotations: annotations.unwrap(),

            contract_id: if let Some(__x) = contract_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopProviderCommand.contract_id (#1)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopProviderCommand.host_id (#2)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopProviderCommand.link_name (#3)".to_string(),
                ));
            },

            provider_ref: if let Some(__x) = provider_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StopProviderCommand.provider_ref (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A command instructing a specific host to perform a live update
/// on the indicated actor by supplying a new image reference. Note that
/// live updates are only possible through image references
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdateActorCommand {
    /// The actor's 56-character unique ID
    #[serde(default)]
    pub actor_id: String,
    /// Optional set of annotations used to describe the nature of this
    /// update request. Only actor instances that have matching annotations
    /// will be upgraded, allowing for instance isolation by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationMap>,
    /// The host ID of the host to perform the live update
    #[serde(default)]
    pub host_id: String,
    /// The new image reference of the upgraded version of this actor
    #[serde(default)]
    pub new_actor_ref: String,
}

// Encode UpdateActorCommand as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_update_actor_command<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &UpdateActorCommand,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("actorId")?;
    e.str(&val.actor_id)?;
    if let Some(val) = val.annotations.as_ref() {
        e.str("annotations")?;
        encode_annotation_map(e, val)?;
    } else {
        e.null()?;
    }
    e.str("hostId")?;
    e.str(&val.host_id)?;
    e.str("newActorRef")?;
    e.str(&val.new_actor_ref)?;
    Ok(())
}

// Decode UpdateActorCommand from cbor input stream
#[doc(hidden)]
pub fn decode_update_actor_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<UpdateActorCommand, RpcError> {
    let __result = {
        let mut actor_id: Option<String> = None;
        let mut annotations: Option<Option<AnnotationMap>> = Some(None);
        let mut host_id: Option<String> = None;
        let mut new_actor_ref: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct UpdateActorCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct UpdateActorCommand: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_id = Some(d.str()?.to_string()),
                    1 => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    2 => host_id = Some(d.str()?.to_string()),
                    3 => new_actor_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct UpdateActorCommand: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorId" => actor_id = Some(d.str()?.to_string()),
                    "annotations" => {
                        annotations = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(
                                decode_annotation_map(d)
                                    .map_err(|e| format!("decoding 'AnnotationMap': {}", e))?,
                            ))
                        }
                    }
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "newActorRef" => new_actor_ref = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        UpdateActorCommand {
            actor_id: if let Some(__x) = actor_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field UpdateActorCommand.actor_id (#0)".to_string(),
                ));
            },
            annotations: annotations.unwrap(),

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field UpdateActorCommand.host_id (#2)".to_string(),
                ));
            },

            new_actor_ref: if let Some(__x) = new_actor_ref {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field UpdateActorCommand.new_actor_ref (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Lattice Controller - Describes the interface used for actors
/// to communicate with a lattice controller, enabling developers
/// to deploy actors that can manipulate the lattice in which they're
/// running.
/// wasmbus.contractId: wasmcloud:latticecontrol
/// wasmbus.providerReceive
#[async_trait]
pub trait LatticeController {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:latticecontrol"
    }
    /// Seek out a list of suitable hosts for a capability provider given
    /// a set of host label constraints. Hosts on which this provider is already
    /// running will not be among the successful "bidders" in this auction.
    async fn auction_provider(
        &self,
        ctx: &Context,
        arg: &ProviderAuctionRequest,
    ) -> RpcResult<ProviderAuctionAcks>;
    /// Seek out a list of suitable hosts for an actor given a set of host
    /// label constraints.
    async fn auction_actor(
        &self,
        ctx: &Context,
        arg: &ActorAuctionRequest,
    ) -> RpcResult<ActorAuctionAcks>;
    /// Queries the list of hosts currently visible to the lattice. This is
    /// a "gather" operation and so can be influenced by short timeouts,
    /// network partition events, etc.
    async fn get_hosts(&self, ctx: &Context) -> RpcResult<Hosts>;
    /// Queries for the contents of a host given the supplied 56-character unique ID
    async fn get_host_inventory<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<HostInventory>;
    /// Queries the lattice for the list of known/cached claims by taking the response
    /// from the first host that answers the query.
    async fn get_claims(&self, ctx: &Context) -> RpcResult<GetClaimsResponse>;
    /// Instructs a given host to scale the indicated actor
    async fn scale_actor(
        &self,
        ctx: &Context,
        arg: &ScaleActorCommand,
    ) -> RpcResult<CtlOperationAck>;
    /// Instructs a given host to start the indicated actor
    async fn start_actor(
        &self,
        ctx: &Context,
        arg: &StartActorCommand,
    ) -> RpcResult<CtlOperationAck>;
    /// Publish a link definition into the lattice, allowing it to be cached and
    /// delivered to the appropriate capability provider instances
    async fn advertise_link(
        &self,
        ctx: &Context,
        arg: &wasmbus_rpc::core::LinkDefinition,
    ) -> RpcResult<CtlOperationAck>;
    /// Requests the removal of a link definition. The definition will be removed
    /// from the cache and the relevant capability providers will be given a chance
    /// to de-provision any used resources
    async fn remove_link(
        &self,
        ctx: &Context,
        arg: &RemoveLinkDefinitionRequest,
    ) -> RpcResult<CtlOperationAck>;
    /// Queries all current link definitions in the lattice. The first host
    /// that receives this response will reply with the contents of the distributed
    /// cache
    async fn get_links(&self, ctx: &Context) -> RpcResult<LinkDefinitionList>;
    /// Requests that a specific host perform a live update on the indicated
    /// actor
    async fn update_actor(
        &self,
        ctx: &Context,
        arg: &UpdateActorCommand,
    ) -> RpcResult<CtlOperationAck>;
    /// Requests that the given host start the indicated capability provider
    async fn start_provider(
        &self,
        ctx: &Context,
        arg: &StartProviderCommand,
    ) -> RpcResult<CtlOperationAck>;
    /// Requests that the given capability provider be stopped on the indicated host
    async fn stop_provider(
        &self,
        ctx: &Context,
        arg: &StopProviderCommand,
    ) -> RpcResult<CtlOperationAck>;
    /// Requests that an actor be stopped on the given host
    async fn stop_actor(&self, ctx: &Context, arg: &StopActorCommand)
        -> RpcResult<CtlOperationAck>;
    /// Requests that the given host be stopped
    async fn stop_host(&self, ctx: &Context, arg: &StopHostCommand) -> RpcResult<CtlOperationAck>;
    /// Instructs all listening hosts to use the enclosed credential map for
    /// authentication to secure artifact (OCI/bindle) registries. Any host that
    /// receives this message will _delete_ its previous credential map and replace
    /// it with the enclosed. The credential map for a lattice can be purged by sending
    /// this message with an empty map
    async fn set_registry_credentials(
        &self,
        ctx: &Context,
        arg: &RegistryCredentialMap,
    ) -> RpcResult<()>;
}

/// LatticeControllerReceiver receives messages defined in the LatticeController service trait
/// Lattice Controller - Describes the interface used for actors
/// to communicate with a lattice controller, enabling developers
/// to deploy actors that can manipulate the lattice in which they're
/// running.
#[doc(hidden)]
#[async_trait]
pub trait LatticeControllerReceiver: MessageDispatch + LatticeController {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "AuctionProvider" => {
                let value: ProviderAuctionRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ProviderAuctionRequest': {}", e)))?;
                let resp = LatticeController::auction_provider(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.AuctionProvider",
                    arg: Cow::Owned(buf),
                })
            }
            "AuctionActor" => {
                let value: ActorAuctionRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ActorAuctionRequest': {}", e)))?;
                let resp = LatticeController::auction_actor(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.AuctionActor",
                    arg: Cow::Owned(buf),
                })
            }
            "GetHosts" => {
                let resp = LatticeController::get_hosts(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.GetHosts",
                    arg: Cow::Owned(buf),
                })
            }
            "GetHostInventory" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = LatticeController::get_host_inventory(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.GetHostInventory",
                    arg: Cow::Owned(buf),
                })
            }
            "GetClaims" => {
                let resp = LatticeController::get_claims(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.GetClaims",
                    arg: Cow::Owned(buf),
                })
            }
            "ScaleActor" => {
                let value: ScaleActorCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ScaleActorCommand': {}", e)))?;
                let resp = LatticeController::scale_actor(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.ScaleActor",
                    arg: Cow::Owned(buf),
                })
            }
            "StartActor" => {
                let value: StartActorCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StartActorCommand': {}", e)))?;
                let resp = LatticeController::start_actor(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.StartActor",
                    arg: Cow::Owned(buf),
                })
            }
            "AdvertiseLink" => {
                let value: wasmbus_rpc::core::LinkDefinition =
                    wasmbus_rpc::common::deserialize(&message.arg)
                        .map_err(|e| RpcError::Deser(format!("'LinkDefinition': {}", e)))?;
                let resp = LatticeController::advertise_link(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.AdvertiseLink",
                    arg: Cow::Owned(buf),
                })
            }
            "RemoveLink" => {
                let value: RemoveLinkDefinitionRequest =
                    wasmbus_rpc::common::deserialize(&message.arg).map_err(|e| {
                        RpcError::Deser(format!("'RemoveLinkDefinitionRequest': {}", e))
                    })?;
                let resp = LatticeController::remove_link(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.RemoveLink",
                    arg: Cow::Owned(buf),
                })
            }
            "GetLinks" => {
                let resp = LatticeController::get_links(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.GetLinks",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdateActor" => {
                let value: UpdateActorCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'UpdateActorCommand': {}", e)))?;
                let resp = LatticeController::update_actor(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.UpdateActor",
                    arg: Cow::Owned(buf),
                })
            }
            "StartProvider" => {
                let value: StartProviderCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StartProviderCommand': {}", e)))?;
                let resp = LatticeController::start_provider(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.StartProvider",
                    arg: Cow::Owned(buf),
                })
            }
            "StopProvider" => {
                let value: StopProviderCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StopProviderCommand': {}", e)))?;
                let resp = LatticeController::stop_provider(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.StopProvider",
                    arg: Cow::Owned(buf),
                })
            }
            "StopActor" => {
                let value: StopActorCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StopActorCommand': {}", e)))?;
                let resp = LatticeController::stop_actor(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.StopActor",
                    arg: Cow::Owned(buf),
                })
            }
            "StopHost" => {
                let value: StopHostCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StopHostCommand': {}", e)))?;
                let resp = LatticeController::stop_host(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "LatticeController.StopHost",
                    arg: Cow::Owned(buf),
                })
            }
            "SetRegistryCredentials" => {
                let value: RegistryCredentialMap =
                    wasmbus_rpc::common::deserialize(&message.arg)
                        .map_err(|e| RpcError::Deser(format!("'RegistryCredentialMap': {}", e)))?;
                let _resp = LatticeController::set_registry_credentials(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "LatticeController.SetRegistryCredentials",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "LatticeController::{}",
                message.method
            ))),
        }
    }
}

/// LatticeControllerSender sends messages to a LatticeController service
/// Lattice Controller - Describes the interface used for actors
/// to communicate with a lattice controller, enabling developers
/// to deploy actors that can manipulate the lattice in which they're
/// running.
/// client for sending LatticeController messages
#[derive(Debug)]
pub struct LatticeControllerSender<T: Transport> {
    transport: T,
}

impl<T: Transport> LatticeControllerSender<T> {
    /// Constructs a LatticeControllerSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl LatticeControllerSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a LatticeController provider
    /// implementing the 'wasmcloud:latticecontrol' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:latticecontrol",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a LatticeController provider
    /// implementing the 'wasmcloud:latticecontrol' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:latticecontrol",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> LatticeController
    for LatticeControllerSender<T>
{
    #[allow(unused)]
    /// Seek out a list of suitable hosts for a capability provider given
    /// a set of host label constraints. Hosts on which this provider is already
    /// running will not be among the successful "bidders" in this auction.
    async fn auction_provider(
        &self,
        ctx: &Context,
        arg: &ProviderAuctionRequest,
    ) -> RpcResult<ProviderAuctionAcks> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.AuctionProvider",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ProviderAuctionAcks = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': ProviderAuctionAcks", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Seek out a list of suitable hosts for an actor given a set of host
    /// label constraints.
    async fn auction_actor(
        &self,
        ctx: &Context,
        arg: &ActorAuctionRequest,
    ) -> RpcResult<ActorAuctionAcks> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.AuctionActor",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ActorAuctionAcks = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': ActorAuctionAcks", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Queries the list of hosts currently visible to the lattice. This is
    /// a "gather" operation and so can be influenced by short timeouts,
    /// network partition events, etc.
    async fn get_hosts(&self, ctx: &Context) -> RpcResult<Hosts> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.GetHosts",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Hosts = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Hosts", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Queries for the contents of a host given the supplied 56-character unique ID
    async fn get_host_inventory<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<HostInventory> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.GetHostInventory",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: HostInventory = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': HostInventory", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Queries the lattice for the list of known/cached claims by taking the response
    /// from the first host that answers the query.
    async fn get_claims(&self, ctx: &Context) -> RpcResult<GetClaimsResponse> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.GetClaims",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: GetClaimsResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': GetClaimsResponse", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Instructs a given host to scale the indicated actor
    async fn scale_actor(
        &self,
        ctx: &Context,
        arg: &ScaleActorCommand,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.ScaleActor",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Instructs a given host to start the indicated actor
    async fn start_actor(
        &self,
        ctx: &Context,
        arg: &StartActorCommand,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.StartActor",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Publish a link definition into the lattice, allowing it to be cached and
    /// delivered to the appropriate capability provider instances
    async fn advertise_link(
        &self,
        ctx: &Context,
        arg: &wasmbus_rpc::core::LinkDefinition,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.AdvertiseLink",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests the removal of a link definition. The definition will be removed
    /// from the cache and the relevant capability providers will be given a chance
    /// to de-provision any used resources
    async fn remove_link(
        &self,
        ctx: &Context,
        arg: &RemoveLinkDefinitionRequest,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.RemoveLink",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Queries all current link definitions in the lattice. The first host
    /// that receives this response will reply with the contents of the distributed
    /// cache
    async fn get_links(&self, ctx: &Context) -> RpcResult<LinkDefinitionList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.GetLinks",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: LinkDefinitionList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': LinkDefinitionList", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests that a specific host perform a live update on the indicated
    /// actor
    async fn update_actor(
        &self,
        ctx: &Context,
        arg: &UpdateActorCommand,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.UpdateActor",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests that the given host start the indicated capability provider
    async fn start_provider(
        &self,
        ctx: &Context,
        arg: &StartProviderCommand,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.StartProvider",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests that the given capability provider be stopped on the indicated host
    async fn stop_provider(
        &self,
        ctx: &Context,
        arg: &StopProviderCommand,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.StopProvider",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests that an actor be stopped on the given host
    async fn stop_actor(
        &self,
        ctx: &Context,
        arg: &StopActorCommand,
    ) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.StopActor",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Requests that the given host be stopped
    async fn stop_host(&self, ctx: &Context, arg: &StopHostCommand) -> RpcResult<CtlOperationAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.StopHost",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CtlOperationAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CtlOperationAck", e)))?;
        Ok(value)
    }

    #[allow(unused)]
    /// Instructs all listening hosts to use the enclosed credential map for
    /// authentication to secure artifact (OCI/bindle) registries. Any host that
    /// receives this message will _delete_ its previous credential map and replace
    /// it with the enclosed. The credential map for a lattice can be purged by sending
    /// this message with an empty map
    async fn set_registry_credentials(
        &self,
        ctx: &Context,
        arg: &RegistryCredentialMap,
    ) -> RpcResult<()> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "LatticeController.SetRegistryCredentials",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
}
