use std::sync::{Arc};
use std::time::SystemTime;
use chrono::Duration;
use futures_util::lock::Mutex;
use json::JsonValue;
use model::Message;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::platform::model::MessageType;
use crate::platform::transport::{new_pubnub_transport, PubnubTransport};
use timer;
use tokio::task::JoinHandle;
use crate::platform::{client, model};
use colored::Colorize;
use reqwest::{Response};
use anyhow::Error;
use crate::client::v3::client::Client as v3_client;
use serde::{Deserialize};

#[derive(Deserialize)]
struct RegistrationResponse {
    channel: String
}

pub struct Client {
    transport: Arc<Mutex<PubnubTransport>>,
    last_received_ping: Arc<Mutex<Option<SystemTime>>>,
    channel: String,
}

impl Client {
    pub async fn init(
        webhook_router_uri: String,
        client: v3_client,
    ) -> anyhow::Result<Self> {
       let pubnub = new_pubnub_transport()?;

        let token = client.get_token().await?;

        let channel = match reqwest::Client::new()
            .post(webhook_router_uri)
            .bearer_auth(token)
            .send()
            .await {
            Ok(r) => {
                if r.status().is_success() {
                    let reg_res: RegistrationResponse = r.json()?;
                    reg_res.channel
                } else {
                    return Err(Error::msg("Returned non-success status code".red()))
                }
            }
            Err(e) => return Err(Error::from(e))
        };

        Ok(Client{
            transport: Arc::new(Mutex::new(pubnub)),
            last_received_ping: Arc::new(Mutex::new(None)),
            channel: channel
        })
    }

    pub async fn start(&mut self) -> JoinHandle<()> {
        let (kill_sender, mut kill_listener) = broadcast::channel(1);
        let t = Arc::clone(&self.transport);

        let (sender, mut receiver): (Sender<JsonValue>, Receiver<JsonValue>) =
            broadcast::channel(100000);
        let mut t_guard = t.lock().await;
        let join = t_guard
            .subscribe(self.channel.clone(), sender, kill_listener)
            .await;

        self.start_receiving_from(receiver);
        self.start_timeout();

        join
    }

    fn start_receiving_from(&self, mut receiver: Receiver<JsonValue>) {
        let lrp = Arc::clone(&self.last_received_ping);
        tokio::spawn(async move {
            loop {
                let val = match receiver.recv().await {
                    Ok(val) => val,
                    Err(e) => {
                        break;
                    }
                };
                let m: Message = serde_json::from_str(val.to_string().as_str()).unwrap();
                match m.typ {
                    MessageType::WebhookRequest => {
                    }
                    MessageType::Ping => {
                        println!("Ping type message arrived");
                        let now = SystemTime::now();
                        let mut lrp_guard = lrp.lock().await;
                        *lrp_guard = Some(now);
                    }
                }
            }
        });
    }

    fn start_timeout(&self) {
        let lrp = Arc::clone(&self.last_received_ping);
        tokio::spawn(async move {
            loop {
                println!("Timeout started");
                let (mut tx, mut rx) = broadcast::channel(30);

                let timer = timer::Timer::new();
                let tx_c = tx.clone();
                let g = timer.schedule_with_delay(Duration::seconds(3), move || {
                    println!("Checked");
                    tx_c.send(());
                });

                rx.recv().await.unwrap();
                println!("Received timeout");

                let mut lrp_guard = lrp.lock().await;
                lrp_guard.map(|lrp| {
                    let passed = SystemTime::now().duration_since(lrp).unwrap().as_secs();
                    if passed > 30 {
                        println!("Server has been inactive for {} seconds", passed.to_string().red());
                    } else {
                        println!("Not yet");
                    }
                });

            }
        });
    }

    fn start_ping_cycle(&self) {
        let t = Arc::clone(&self.transport);
        let c = self.channel.clone();
        tokio::spawn(async move {
            loop {
                println!("Timeout started");
                let (mut tx, mut rx) = broadcast::channel(30);

                let timer = timer::Timer::new();
                let tx_c = tx.clone();
                let g = timer.schedule_with_delay(Duration::seconds(3), move || {
                    println!("Checked");
                    tx_c.send(());
                });

                rx.recv().await.unwrap();
                println!("Received timeout for ping");

                let transport = t.lock().await;
                match serde_json::to_string(&Message{
                    typ: MessageType::Ping,
                    payload: None
                }) {
                    Ok(m) => {
                        match transport.send(&c, &m).await {
                            Ok(_) => {}
                            Err(e) => println!("error sending ping: {}", e)
                        };
                    }
                    Err(e) => println!("error while serializing ping message: {}", e)
                };
            }
        });
    }
}