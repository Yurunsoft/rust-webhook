use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{
    config::get_config,
    route::Context,
    util::{hash_hmac_sha256, shell_exec},
};

#[derive(Serialize, Deserialize, Debug)]
struct GithubRequest {
    repository: GithubRequestDataRepository,
    #[serde(rename = "ref")]
    _ref: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GithubRequestDataRepository {
    full_name: String,
}

pub async fn route_github(context: &mut Context) -> anyhow::Result<()> {
    let body_bytes: Vec<u8> = hyper::body::to_bytes(context.request.body_mut())
        .await?
        .into_iter()
        .collect();

    let data: GithubRequest = json5::from_str(&String::from_utf8(body_bytes.clone())?)?;

    let x_github_event = match context.request.headers().get("x-github-event") {
        Some(v) => v.to_str()?.to_string(),
        None => bail!("Get header x-github-event failed!"),
    };
    let x_hub_signature_256 = match context.request.headers().get("x-hub-signature-256") {
        Some(v) => v.to_str()?.to_string(),
        None => bail!("Get header x-hub-signature-256 failed!"),
    };

    tokio::spawn(async move {
        let config = get_config().unwrap();
        for item in config.sites.github {
            if !item.password.is_empty()
                && !check_signature_sha256(&body_bytes, &item.password, &x_hub_signature_256)
            {
                continue;
            }
            if !item.name.is_empty() && item.name != data.repository.full_name {
                continue;
            }
            if !item.hook_name.is_empty() && item.hook_name != x_github_event {
                continue;
            }
            if !item._ref.is_empty() && item._ref != data._ref {
                continue;
            }
            for cmd in item.cmds {
                println!("CMD: {}", cmd);
                shell_exec(cmd.as_str()).await.unwrap();
            }
        }
    });

    println!("github hook");
    Ok(())
}

fn check_signature_sha256(bytes: &Vec<u8>, secret: &String, signature: &String) -> bool {
    let hash = match hash_hmac_sha256(bytes.as_slice(), secret.as_bytes()) {
        Ok(hash) => hash,
        Err(_) => return false,
    };
    match signature.find("=") {
        Some(index) => hash == signature.split_at(index + 1).1,
        None => false,
    }
}
