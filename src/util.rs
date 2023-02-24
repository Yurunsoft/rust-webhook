use anyhow::bail;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};

type HmacSha256 = Hmac<Sha256>;

pub async fn exec_ex(cmd: &str, args: &[&str]) -> anyhow::Result<ExitStatus> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    let mut out = BufReader::new(match child.stdout.as_mut() {
        Some(stdout) => stdout,
        None => bail!("Failed to open stdout"),
    });
    let mut line = String::new();
    while out.read_line(&mut line)? > 0 {
        println!("{}", line);
    }
    Ok(child.wait()?)
}

pub async fn exec(cmd: &str) -> anyhow::Result<ExitStatus> {
    let (cmd, args) = parse_cmd(cmd)?;
    Ok(exec_ex(cmd, args.as_slice()).await?)
}

pub async fn shell_exec_ex(cmd: &str, args: &[&str]) -> anyhow::Result<ExitStatus> {
    let mut args_vec = vec![];
    let shell = if cfg!(target_os = "windows") {
        args_vec.push("/s");
        args_vec.push("/c");
        "cmd"
    } else {
        args_vec.push("-c");
        "sh"
    };
    let cmd = cmd.to_string() + " " + &args.join(" ");
    args_vec.push(cmd.as_str());
    Ok(exec_ex(shell, args_vec.as_ref()).await?)
}

pub async fn shell_exec(cmd: &str) -> anyhow::Result<ExitStatus> {
    let (cmd, args) = parse_cmd(cmd)?;
    Ok(shell_exec_ex(cmd, args.as_slice()).await?)
}

fn parse_cmd(cmd: &str) -> anyhow::Result<(&str, Vec<&str>)> {
    let splited: Vec<&str> = cmd.trim().split(" ").into_iter().collect();
    let splited = match splited.split_first() {
        Some(v) => v,
        None => {
            bail!("Parse cmd failed!")
        }
    };
    let mut args = splited.1.to_vec();
    args.retain(|item| !item.is_empty());
    Ok((splited.0, args))
}

pub fn hash_hmac_sha256(data: &[u8], key: &[u8]) -> anyhow::Result<String> {
    let mut mac = match HmacSha256::new_from_slice(key) {
        Ok(mac) => mac,
        Err(e) => bail!("HMAC can take key of any size: {:?}", e),
    };
    mac.update(data);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(hex::encode(code_bytes))
}

pub fn current_dir() -> anyhow::Result<String> {
    match env::current_dir()?.to_str() {
        Some(v) => Ok(v.to_string()),
        None => bail!("Get current dir failed!"),
    }
}
