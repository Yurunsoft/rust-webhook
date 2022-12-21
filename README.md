# rust-webhook

![GitHub Workflow Status (branch)](https://img.shields.io/github/actions/workflow/status/yurunsoft/rust-webhook/test.yml?branch=master)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://github.com/yurunsoft/rust-webhook/blob/master/LICENSE)

## 介绍

rust-webook 是宇润用 Rust 开发的简易 Webhook 服务，支持 Gitee 和 Github。

可执行文件下载：<https://github.com/Yurunsoft/rust-webhook/releases>

PHP Swoole 版本：<https://gitee.com/yurunsoft/swoole-webhook>

> 两个项目的配置文件通用

## 使用说明

### 配置文件

**文件名：** `config.json`

```js
{
    // 服务配置
    "server": {
        "host": "0.0.0.0", // 主机名
        "port": 12580 // 端口号
    },
    // 站点列表
    "sites": {
        // Gitee 配置
        "gitee": [
            {
                "name": "yurunsoft/imi", // 仓库名称，留空不验证
                "password": "密码", // 密码或Secret，留空不验证，但强烈建议配置
                "ref": "refs/heads/master", // 分支，留空不验证
                "hook_name": "push_hooks", // 动作名称，留空不验证
                // 执行的命令数组
                "cmds": [
                    "git -C /projects/imi pull",
                    "systemctl restart imi"
                ]
            }
        ],
        // Github 配置
        "github": [
            {
                "name": "yurunsoft/imi", // 仓库名称
                "password": "密码", // 密码或Secret，留空不验证，但强烈建议配置
                "ref": "refs/heads/master", // 分支，留空不验证
                "hook_name": "push_hooks", // 动作名称，留空不验证
                // 执行的命令数组
                "cmds": [
                    "git -C /projects/imi pull",
                    "systemctl restart imi"
                ]
            }
        ]
    }
}
```

> 解析 json 使用了 json5，所以可以带上注释。

### 启动服务

`./rust-webhook`

> 工作目录中必须要有 `config.json` 配置文件，否则会报错！

## Q&A

## 为什么要开发这个项目

**回答：** 为了学习 Rust

## 项目是否生产可用

**回答：** 项目逻辑十分简单，并且经过验证，已经生产可用。如有问题欢迎提 issue 和 PR，我会尽快处理。
