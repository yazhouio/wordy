use std::{sync::Arc, default};

use anyhow::{anyhow, Result};
use tracing::info;
use uuid::Uuid;

type Sender<T> = tokio::sync::mpsc::UnboundedSender<T>;
type Receiver<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

use crate::{
    utils::{azure_tts::fetch_speed, event, event::WsResponse, openai::en_teacher_chat},
    ws,
};

pub async fn handle_message(
    r: &mut Receiver<event::ChannelMessage>,
    state: Arc<ws::state::WsState>,
) {
    loop {
        while let Some(msg) = r.recv().await {
            // println!("handle_message: {:?}", msg);
            handle_message_item(msg, state.clone()).await.map_err(|e| {
                tracing::error!("handle_message_item error: {:?}", e);
                anyhow!(e)
            }).unwrap();
        }
    }
}

async fn handle_message_item(
    msg: event::ChannelMessage,
    state: Arc<ws::state::WsState>,
) -> Result<()> {
    let msg_id = Arc::new(Uuid::new_v4().to_string());
    info!("handle_message_item: {:?}", msg.body.event);

    let uuids = if msg.body.to == 0 {
        Some(vec![msg.uuid.clone()])
    } else {
        state.get_user_uuid_map(msg.body.to)
    };

    if uuids.is_none() {
        info!("user {} not found", msg.body.to.to_string());
        return Ok(());
    }

    for uuid in uuids.unwrap() {
        let sender = state.get_user_peer_map(uuid.clone());
        if sender.is_none() {
            info!("user {} not found", msg.body.to.to_string());
            continue;
        }
        let sender = sender.unwrap();
        let msg_id = msg_id.clone();
        let msg = Arc::new(msg.body.clone());
        tokio::spawn(async move {
            let msg = msg.clone();
            match msg.to {
                0 => {
                    handle_system_message(msg, msg_id.clone(), &sender)
                        .await
                        .map_err(|e| {
                            tracing::error!("handle_system_message error: {:?}", e);
                            anyhow!("handle_system_message error: {:?}", e)
                        })?;
                }
                _ => {
                    sender.send(msg).map_err(|e| {
                        tracing::error!("handle_system_message error: {:?}", e);
                        anyhow!("handle_system_message error: {:?}", e)
                    })?;
                }
            };
            anyhow::Ok(())
        });
    }
    anyhow::Ok(())
}

async fn handle_system_message(
    msg: Arc<event::WsRequest>,
    msg_id: Arc<String>,
    sender: &Sender<Arc<event::WsRequest>>,
) -> Result<()> {
    let resp = WsResponse {
        event: event::Event::Loading(true),
        event_type: event::EventType::Loading,
        msg_id: msg_id.to_string(),
        from: 0,
        to: msg.from,
        reply_msg_id: Some(msg.msg_id.clone()),
    };
    sender.send(Arc::new(resp))?;
    let sender = sender.clone();
    let msg_id = msg_id.to_string();
    tokio::spawn(async move {
        let resp = handle_system_message_item(msg, msg_id.clone().to_string())
            .await
            .map_err(|e| {
                tracing::error!("handle_system_message_item error: {:?}", e);
                anyhow!(e)
            })?;
        println!("{:#?}", resp);
        sender.clone().send(Arc::new(resp)).map_err(|e| {
            tracing::error!("handle_system_message_item error: {:?}", e);
            anyhow!(e)
        })?;
        anyhow::Ok(())
    });
    Ok(())
}

async fn handle_system_message_item(
    msg: Arc<event::WsRequest>,
    msg_id: String,
) -> Result<event::WsResponse> {
    let msg = msg.clone();
    let mut resp = WsResponse {
        event: msg.event.clone(),
        event_type: event::EventType::Loading,
        msg_id,
        from: 0,
        to: msg.from,
        reply_msg_id: None,
    };
    match msg.event.clone() {
        event::Event::Chat(message) => {
            let openai_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
            let text = en_teacher_chat(&openai_key, &message).await?;
            let res = text.choices.first().unwrap().message.content.clone();
            // let res = "天空的英文是`sky`。它是指地球上大气层上方的空间，
            // 通常是呈现蓝色或灰色的。\    这是它的英文例句：1. `The sky is so
            // clear today, not a single cloud in \    sight.` 2. `When the sun
            // sets, the sky turns into a beautiful mixture of \
            //    pink, purple, and orange colors.`"
            // .to_owned();
            resp.event = event::Event::Chat(res.unwrap_or_default());
            resp.event_type = event::EventType::Chat;
        }
        event::Event::Speech(message) => {
            let azure_tts_key = std::env::var("AZURE_TTS_KEY").expect("AZURE_TTS_KEY not found");
            let region = std::env::var("AZURE_TTS_REGION").expect("AZURE_TTS_REGION not found");
            let path = fetch_speed(&azure_tts_key, &region, &message).await?;
            resp.event = event::Event::Speech(path);
            resp.event_type = event::EventType::Speech;
            resp.reply_msg_id = Some(msg.msg_id.clone());
        }
        _ => {
            return Err(anyhow::anyhow!("unknown event type"));
        }
    };
    Ok(resp)
}
