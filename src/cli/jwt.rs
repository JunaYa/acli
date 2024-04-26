use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{process_jwt_decode, process_jwt_encode, CmdExector};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Sign a text with a private/shared key")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a signature")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Serialize, Deserialize, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long)]
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret: String = process_jwt_encode(self.sub, self.aud, self.exp)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_jwt_decode(&self.token)?;
        println!("{:?}", ret);
        Ok(())
    }
}
