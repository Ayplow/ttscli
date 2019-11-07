#[macro_use]
extern crate json;

use failure::ResultExt;
use std::io::{Read, Write};

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
enum Opt {
    SaveAndPlay,
    Message,
    Execute {
        #[structopt(default_value = "-1")]
        guid: String
    },
    SetScript {
        #[structopt(default_value = "-1")]
        guid: String,
    }
}
fn read_stdin() -> String {
    let mut s = String::new();
    std::io::stdin().lock().read_to_string(&mut s).unwrap();
    s
}
fn main() -> Result<(), exitfailure::ExitFailure> {
    write!(std::net::TcpStream::connect("127.0.0.1:39999").context("Could not connect to server")?, "{}", match Opt::from_args() {
        Opt::SaveAndPlay => object!{
            "messageID" => 1,
            "scriptStates" => json::parse(&read_stdin()).context("Invalid script states")?
        },
        Opt::Message => object!{
            "messageID" => 2,
            "customMessage" => json::parse(&read_stdin()).context("Invalid custom message")?
        },
        Opt::Execute { guid } => object!{
            "messageID" => 3,
            "guid" => guid,
            "script" => read_stdin()
        },
        Opt::SetScript { guid } => object!{
            "messageID" => 1,
            "scriptStates" => json::JsonValue::Array(vec![object!{
                "guid" => guid,
                "script" => read_stdin()
            }])
        }
    })?;
    Ok(())
}
