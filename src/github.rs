use serde::{Deserialize, Serialize};

use crate::{config::get_config, route::Context, util::{shell_exec, hash_hmac_sha256}};

#[derive(Serialize, Deserialize, Debug)]
struct GithubRequest {
    repository: GithubRequestDataRepository,
    #[serde(rename="ref")]
    _ref: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GithubRequestDataRepository {
    full_name: String,
}

pub async fn route_github(context: &mut Context) -> Result<(), hyper::Error> {
    let body_bytes: Vec<u8> = hyper::body::to_bytes(context.request.body_mut()).await?.into_iter().collect();

    let data: GithubRequest = serde_json::from_str(&String::from_utf8(body_bytes.clone()).expect("")).unwrap();

    let x_github_event = context.request.headers().get("x-github-event").unwrap().to_str().unwrap().to_string();
    let x_hub_signature_256 = context.request.headers().get("x-hub-signature-256").unwrap().to_str().unwrap().to_string();

    tokio::spawn(async move {
        let config = get_config();
        for item in config.sites.github {
            if !item.password.is_empty() && !check_signature_sha256(&body_bytes, &item.password, &x_hub_signature_256) {
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
    let hash = hash_hmac_sha256(bytes.as_slice(), secret.as_bytes());
    let index = signature.find("=");
    if index.is_none() {
        return false;
    }
    return hash == signature.split_at(index.unwrap() + 1).1;
}
