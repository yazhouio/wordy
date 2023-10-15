use anyhow::Result;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatCompletionResponse, ChatMessage, Role};

pub async fn chat(openai_api_key:&str, messages: &[ChatMessage]) -> Result<ChatCompletionResponse> {
    let client = Client::new(openai_api_key.to_owned());
    let parameters = ChatCompletionParameters {
    model: "gpt-3.5-turbo".to_string(),
        messages: messages.into(),
        temperature: None,
        top_p: None,
        n: None,
        stop: None,
        max_tokens: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };
    let result = client.chat().create(parameters).await?;
    Ok(result)
}

pub fn get_en_teacher_chat_message(msg: &str) -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: Role::System,
            content: "Suppose you are a kindergarten English starter teacher, I am going to ask you some simple words and ask you to say his English and give as much English explanation and example sentences as possible，and mark the English portion with ``".to_string(),
            name: None,
        },
        ChatMessage {
            role: Role::User,
            content: "路灯".to_string(),
            name: None,
        },
        ChatMessage {
            role: Role::Assistant,
            content: "路灯的英文是`street light`。它是指在街道上安装的照明设备，通常用来照亮道路，提供行人和车辆安全的照明。这是它的英文例句：1. `Look, the street lights are turning on as it gets dark outside.` 2. `It is important to have street lights in the city for safety reasons.`".to_string(),
            name: None,
        },
        ChatMessage {
            role: Role::User,
            content: msg.to_string(),
            name: None,
        },
    ]
}

pub async fn en_teacher_chat(openai_api_key:&str, msg: &str) -> Result<ChatCompletionResponse> {
    let messages = get_en_teacher_chat_message(msg);
    chat(openai_api_key, &messages).await
}
