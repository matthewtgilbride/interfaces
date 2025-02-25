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

/// The Factorial service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
/// wasmbus.contractId: wasmcloud:example:factorial
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait Factorial {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:factorial"
    }
    /// Calculates the factorial (n!) of the input parameter
    async fn calculate(&self, ctx: &Context, arg: &u32) -> RpcResult<u64>;
}

/// FactorialReceiver receives messages defined in the Factorial service trait
/// The Factorial service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
#[doc(hidden)]
#[async_trait]
pub trait FactorialReceiver: MessageDispatch + Factorial {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Calculate" => {
                let value: u32 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U32': {}", e)))?;
                let resp = Factorial::calculate(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Factorial.Calculate",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Factorial::{}",
                message.method
            ))),
        }
    }
}

/// FactorialSender sends messages to a Factorial service
/// The Factorial service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
/// client for sending Factorial messages
#[derive(Debug)]
pub struct FactorialSender<T: Transport> {
    transport: T,
}

impl<T: Transport> FactorialSender<T> {
    /// Constructs a FactorialSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> FactorialSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl FactorialSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl FactorialSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Factorial provider
    /// implementing the 'wasmcloud:example:factorial' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:factorial",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Factorial provider
    /// implementing the 'wasmcloud:example:factorial' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:factorial",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Factorial for FactorialSender<T> {
    #[allow(unused)]
    /// Calculates the factorial (n!) of the input parameter
    async fn calculate(&self, ctx: &Context, arg: &u32) -> RpcResult<u64> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Factorial.Calculate",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: u64 = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': U64", e)))?;
        Ok(value)
    }
}
