use std::path::{Path, PathBuf};
use anyhow::Result;
use aspeak::{AudioFormat, AuthOptionsBuilder, get_rest_endpoint_by_region, RichSsmlOptionsBuilder, SynthesizerConfig, TextOptionsBuilder};
use md5::{Digest, Md5};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn text_to_speech(azure_api_key: &str, region: &str, msg: &str) -> Result<Vec<u8>> {
    let auth = AuthOptionsBuilder::new(
        get_rest_endpoint_by_region(region),
    ).key(azure_api_key).build();
    let config = SynthesizerConfig::new(auth, AudioFormat::Audio16Khz32KBitRateMonoMp3);
    let options = TextOptionsBuilder::new() // Adjusting text options like rate, pitch and voice
        .rate("-20%")
        .voice("zh-CN-XiaoxiaoNeural")
        .pitch("medium")
        .chain_rich_ssml_options_builder(
            RichSsmlOptionsBuilder::new().style("cheerful"), // Set speech style to newscast
        )
        .build();
    let rest_syn = config.rest_synthesizer()?;
    let audio_data = rest_syn.synthesize_text(msg, &options).await?;
    Ok(audio_data)
}

pub async fn fetch_speed(azure_tts_key: &str, region: &str, msg: &str) -> Result<String> {
    let mut hasher = Md5::new();
    hasher.update(msg);
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let result = format!("{:X}.mp3", hasher.finalize());

    let path = assets_dir.join(&result);
    {
        if !path.exists() {
            let result: Vec<u8> = text_to_speech(azure_tts_key, region, msg).await.map_err(|e| anyhow::anyhow!("text_to_speech error: {:?}", e))?;
            fs::write(path, result).await?;
        };
    }
    Ok(result)
}
