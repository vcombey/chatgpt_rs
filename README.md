# Regarding API changes from December 11th 2022
OpenAI made a change to API, and now requires a cloudflare clearance token. Due to this, authentication is becoming complicated. You can view recent updates regarding authentication methods in the [#3 Pull Request](https://github.com/Maxuss/chatgpt_rs/pull/3). The master branch version (and cargo published crate) does not work because of this.

# ChatGPT-rs

This is a reverse-engineered wrapper for the OpenAI's ChatGPT model.

## Usage

```rust
use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> chatgpt::Result<()> {
    // Starting client
    let token: String = std::env::var("OPENAI_SK").unwrap(); // obtain the session token. More on session tokens later.
    let mut client = ChatGPT::new(token)?;
    
    // sending a simple message
    // normal responses take ~10-30 seconds to complete
    let messages = vec![Message {
        role: "user".to_owned(),
        content: "Write me a simple sorting algorithm in Rust".to_owned(),
    }];
    let response: String = client.send_message(messages).await?;

    // in case dynamic updates are important
    // this method allows to receive the message as a stream
    let messages = vec![Message {
        role: "user".to_owned(),
        content: "Write me a simple sorting algorithm in Rust".to_owned(),
    }];
    let mut stream = client.send_message_streaming(messages).await?;
    
    while let Some(part) = stream.next().await {
        // a single response part
        println!("Got response part: {part:?}");
    }

    Ok(())
}
```

Since conversations only hold little data (conversation ID and latest message ID), you can have multiple conversations at the same time!

## Session Tokens
Session tokens allow access to the OpenAI API. You can find them in the Cookie storage of your browser.

### Chromium-based browsers

Do this on the [ChatGPT website](https://chat.openai.com/chat)
1. Ctrl+Shift+I to open dev tools
2. Navigate to the Application tab
3. On the left, choose Storage->Cookies->https://chat.openai.com/chat
4. Get the value of the cookie with name `__Secure-next-auth.session-token`

![Explained in image](./media/token_chromium.png)

### Firefox-based browsers

Do this on the [ChatGPT website](https://chat.openai.com/chat)
1. Ctrl+Shift+I to open dev tools
2. Navigate to the Storage tab
3. On the left choose Cookies->https://chat.openai.com/chat
4. Get the value of the cookie with name `__Secure-next-auth.session-token`

![Explained in image](./media/token_firefox.png)

## Library roadmap

- [x] Refreshing tokens
- [x] Sending message and receiving response
- [x] Receiving response as a stream
- [x] Scoped conversations
- [x] Multiple conversations at the same time
