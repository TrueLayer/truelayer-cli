use std::future::Future;
use std::sync::Arc;

use anyhow::Error;
use futures_util::stream::StreamExt;
use json::{parse, JsonValue};
use pubnub_hyper::core::data::channel;
use pubnub_hyper::core::data::channel::Name;
use pubnub_hyper::core::data::message::Type;
use pubnub_hyper::core::data::timetoken::Timetoken;
use pubnub_hyper::core::json;
use pubnub_hyper::runtime::tokio_global::TokioGlobal;
use pubnub_hyper::transport::hyper::Hyper;
use pubnub_hyper::{core::json::object, core::PubNub, Builder};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;

#[derive(Clone, Debug)]
pub struct PubnubTransport {
    pubnub: PubNub<Hyper, TokioGlobal>,
}

impl PubnubTransport {
    pub async fn send(&self, channel: &str, val: &str) -> anyhow::Result<()> {
        self.pubnub
            .publish(
                channel::Name::from_string_unchecked(String::from(channel)),
                parse(val)?,
            )
            .await
            .map(|_| ())
            .map_err(|e| Error::msg("some publish error"))
    }

    pub async fn subscribe(
        &mut self,
        channel: String,
        sender: Sender<JsonValue>,
        mut kill: Receiver<()>,
    ) -> JoinHandle<()> {
        let mut stream = self
            .pubnub
            .subscribe(Name::from_string_unchecked(channel.clone()))
            .await;
        println!("Subscribing......");
        tokio::spawn(async move {
            loop {
                println!("Loop started......");
                tokio::select! {
                    Some(m) = stream.next() => {
                        match sender.send(m.json) {
                           Ok(size) => println!("Sent size {}", size),
                            Err(e) => println!("Error occured while sending {}", e)
                        }
                    },
                    _ = kill.recv() => {
                        println!("sub to channel {} killed", channel);
                        break;
                    }
                }
            }
        })
    }
}

pub fn new_pubnub_transport() -> anyhow::Result<PubnubTransport> {
    let transport = Hyper::new()
        .publish_key("pub-c-53d82f63-c563-4216-a63f-6e7b1954677b")
        .subscribe_key("sub-c-766c55c7-583c-4eec-adb8-cf1e448a4925")
        .build()?;

    let mut pubnub = Builder::new()
        .transport(transport)
        .runtime(TokioGlobal)
        .build();

    Ok(PubnubTransport { pubnub })
}
