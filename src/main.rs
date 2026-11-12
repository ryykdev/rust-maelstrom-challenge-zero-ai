use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::io;

fn main() {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let stdin_result = stdin.read_line(&mut buffer);
        if stdin_result.is_err() {
            continue;
        }
        match from_str::<Message>(&buffer) {
            Ok(msg) => match msg.body.typ {
                Typ::Init => {
                    let message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::InitOk,
                            msg_id: msg.body.msg_id,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: None,
                        },
                    };
                    let out = to_string(&message).unwrap();
                    println!("{out}");
                }
                Typ::Echo => {
                    let message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            typ: Typ::EchoOk,
                            msg_id: msg.body.msg_id,
                            in_reply_to: Some(msg.body.msg_id),
                            node_id: None,
                            node_ids: None,
                            echo: msg.body.echo,
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
struct Message {
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
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Typ {
    Echo,
    EchoOk,
    Init,
    InitOk,
}
