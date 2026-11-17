use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{collections::HashMap, io};
use uuid::Uuid;

fn main() {
    let mut received_messages = Vec::<i64>::new();
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let stdin_result = stdin.read_line(&mut buffer);
        if stdin_result.is_err() {
            panic!("stdin read_line failed"); // this is panic sort of exercise
        }
        match from_str::<PayLoad>(&buffer) {
            Ok(msg) => match msg.body.typ {
                Typ::Init => {
                    let message = PayLoad {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::InitOk,
                            msg_id: msg.body.msg_id,
                            id: None,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: None,
                            message: None,
                            messages: None,
                            topology: None,
                        },
                    };
                    let out = to_string(&message).unwrap();
                    println!("{out}");
                }
                Typ::Echo => {
                    let message = PayLoad {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::EchoOk,
                            msg_id: msg.body.msg_id,
                            id: None,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: msg.body.echo,
                            message: None,
                            messages: None,
                            topology: None,
                        },
                    };
                    let out = to_string(&message).unwrap();
                    println!("{out}");
                }
                Typ::Generate => {
                    let id = Uuid::new_v4();
                    let message = PayLoad {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::GenerateOk,
                            id: Some(id),
                            msg_id: msg.body.msg_id,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: None,
                            message: None,
                            messages: None,
                            topology: None,
                        },
                    };
                    let out = to_string(&message).unwrap();
                    println!("{out}");
                }
                Typ::Broadcast => {
                    if let Some(message) = msg.body.message {
                        received_messages.push(message);
                        let message = PayLoad {
                            src: msg.dest,
                            dest: msg.src,
                            body: Body {
                                typ: Typ::BroadcastOk,
                                id: None,
                                msg_id: msg.body.msg_id,
                                in_reply_to: Some(msg.body.msg_id),
                                node_id: None,
                                node_ids: None,
                                echo: None,
                                message: None,
                                messages: None,
                                topology: None,
                            },
                        };
                        let out = to_string(&message).unwrap();
                        println!("{out}");
                    }
                }
                Typ::Read => {
                    // return integers
                    let message = PayLoad {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::ReadOk,
                            id: None,
                            msg_id: msg.body.msg_id,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: None,
                            message: None,
                            messages: Some(received_messages.clone()),
                            topology: None,
                        },
                    };
                    let out = to_string(&message).unwrap();
                    println!("{out}");
                }
                Typ::Topology => {
                    // handled in next step
                    let message = PayLoad {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::TopologyOk,
                            id: None,
                            msg_id: msg.body.msg_id,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: None,
                            message: None,
                            messages: None,
                            topology: None,
                        },
                    };
                    let out = to_string(&message).unwrap();
                    println!("{out}");
                }
                _ => continue,
            },
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PayLoad {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    typ: Typ,
    msg_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topology: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Typ {
    Echo,
    EchoOk,
    Init,
    InitOk,
    Generate,
    GenerateOk,
    Broadcast,
    BroadcastOk,
    Read,
    ReadOk,
    Topology,
    TopologyOk,
}
