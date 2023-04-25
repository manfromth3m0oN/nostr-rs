use futures_util::SinkExt;
use http::Uri;
use tokio_websockets::{ClientBuilder, Connector, Error, Message};

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let addr = Uri::from_static("wss://ws.postman-echo.com/raw");
    let addr = Uri::from_static("wss://relay.nostrich.de");
    let init_msg = Message::text(String::from(
        r#"["REQ", "my-sub", {"kinds":[1], "authors":["35d26e4690cbe1"]}]"#,
    ));

    let mut client = ClientBuilder::from_uri(addr)
        .connector(&Connector::new()?)
        .connect()
        .await?;
    client.send(init_msg).await?;

    while let Some(message) = client.next().await {
        println!("{message:?}");
    }

    Ok(())
}
