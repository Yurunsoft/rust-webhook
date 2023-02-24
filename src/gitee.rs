use serde::{Deserialize, Serialize};

use crate::{config::get_config, route::Context, util::shell_exec};

#[derive(Serialize, Deserialize, Debug)]
struct GiteeRequest {
    password: String,
    hook_name: String,
    #[serde(rename = "ref")]
    _ref: String,
    project: GiteeRequestProject,
}
#[derive(Serialize, Deserialize, Debug)]
struct GiteeRequestProject {
    path_with_namespace: String,
}

pub async fn route_gitee(context: &mut Context) -> anyhow::Result<()> {
    let body_bytes = hyper::body::to_bytes(context.request.body_mut()).await?;

    let data: GiteeRequest =
        json5::from_str(&String::from_utf8(body_bytes.into_iter().collect())?)?;

    tokio::spawn(async move {
        let config = get_config().unwrap();
        for item in config.sites.gitee {
            if !item.name.is_empty() && item.name != data.project.path_with_namespace {
                continue;
            }
            if !item.password.is_empty() && item.password != data.password {
                continue;
            }
            if !item.hook_name.is_empty() && item.hook_name != data.hook_name {
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

    println!("gitee hook");
    Ok(())
}
