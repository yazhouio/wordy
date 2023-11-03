use std::{path::PathBuf, str};

use anyhow::Result;
use aspeak::{
    get_rest_endpoint_by_region, AudioFormat, AuthOptionsBuilder, RichSsmlOptionsBuilder,
    SynthesizerConfig, TextOptionsBuilder,
};
use blake2::{Blake2s256, Digest};
use tokio::fs;

#[allow(dead_code)]

pub struct TextConfigOptions {
    rate: String,
    voice: String,
    pitch: String,
    style: String,
}

impl TextConfigOptions {
    pub fn new_cn() -> Self {
        Self {
            rate: String::from("-20%"),
            voice: String::from("zh-CN-XiaoxiaoNeural"),
            pitch: String::from("medium"),
            style: String::from("cheerful"),
        }
    }

    pub fn new_en() -> Self {
        Self {
            rate: String::from("-20%"),
            voice: String::from("en-US-AriaNeural"),
            pitch: String::from("medium"),
            style: String::from("cheerful"),
        }
    }
}

async fn text_to_speech(azure_api_key: &str, region: &str, msg: &str) -> Result<Vec<u8>> {
    let auth = AuthOptionsBuilder::new(get_rest_endpoint_by_region(region))
        .key(azure_api_key)
        .build();
    let config = SynthesizerConfig::new(auth, AudioFormat::Audio16Khz32KBitRateMonoMp3);
    let options = TextConfigOptions::new_en();
    let options = TextOptionsBuilder::new() // Adjusting text options like rate, pitch and voice
        .rate(options.rate)
        .voice(options.voice)
        .pitch(options.pitch)
        .chain_rich_ssml_options_builder(
            RichSsmlOptionsBuilder::new().style(options.style), // Set speech style to newscast
        )
        .build();
    let rest_syn = config.rest_synthesizer()?;
    let audio_data = rest_syn.synthesize_text(msg, &options).await?;
    Ok(audio_data)
}

pub fn hash(msg: &str) -> Result<String> {
    let mut hasher = Blake2s256::new();
    hasher.update(msg.as_bytes());
    let res = hasher.finalize();
    Ok(hex::encode(res))
}

pub async fn fetch_speed(azure_tts_key: &str, region: &str, msg: &str) -> Result<String> {
    let name = hash(msg)?;
    let result = format!("{name}.mp3");
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let path = assets_dir.join(&result);
    dbg!("assets_dir: {:?}", &path);
    {
        if !path.exists() {
            let res: Vec<u8> = text_to_speech(azure_tts_key, region, msg)
                .await
                .map_err(|e| anyhow::anyhow!("text_to_speech error: {:?}", e))?;
            fs::write(path, res).await?;
        };
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash() {
        let msg = &"hello world";
        let name = hash(msg).unwrap();
        let code = "9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b";
        assert_eq!(name, code);
    }
}
