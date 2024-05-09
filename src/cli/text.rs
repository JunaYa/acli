use std::{fmt, fs, path::PathBuf, str::FromStr};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExector;

use super::{verify_input, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key pair")]
    Generate(KeyGenerateOpts),
    #[command(about = "Encrypt a text")]
    Encrypt(EncryptOpts),
    #[command(about = "Decrypt a text")]
    Decrypt(DecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input)]
    pub key: String,
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input)]
    pub key: String,
    #[arg(short, long)]
    pub signature: String,
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(short, long)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, String> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let sig = crate::process_text_sign(&mut reader, &key, self.format)?;
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.signature)?;
        let verified = crate::process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("✅ Signature verified");
        } else {
            println!("❌ Signature not verified");
        }
        Ok(())
    }
}

impl CmdExector for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_key_generate(self.format)?;
        for (k, v) in key {
            let _ = fs::write(self.output_path.join(k), v);
        }
        Ok(())
    }
}

impl CmdExector for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let encrypted = crate::process_text_encrypt(&mut reader, &key)?;
        let encoded = URL_SAFE_NO_PAD.encode(encrypted);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let input: Vec<u8> = crate::get_content(&self.input)?;
        let encrypted = URL_SAFE_NO_PAD.decode(input)?;
        let key: Vec<u8> = crate::get_content(&self.key)?;
        let decrypted = crate::process_text_decrypt(&mut encrypted.as_slice(), &key)?;
        println!("{}", String::from_utf8_lossy(&decrypted));
        Ok(())
    }
}
