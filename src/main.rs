use dotenv::dotenv;
use std::error::Error;
use std::io::{self, stdout, Write};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::StreamExt;

pub fn initialize() {
    dotenv().ok();
}

fn process_input(input: &str) -> String {
    // Perform processing logic on the input string
    // Replace this with your own processing logic
    input.to_uppercase()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initialize();
    let client = Client::new();

    let mut messages = vec![ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content("You are a helpful assistant.")
        .build()?];

    loop {
        // Prompt for input
        print!("Enter prompt (or 'quit' to exit): ");
        io::stdout().flush().unwrap();

        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim leading/trailing whitespace and remove newline
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            // Exit the program if the user enters "quit"
            break;
        }

        if input.eq_ignore_ascii_case("create new chat") {
            // Exit the program if the user enters "quit"
            messages = Vec::new();
            continue;
        }

        // Process the input and print the output
        let output = process_input(input);

        messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(output.to_string())
                .build()?,
        );

        let currentMessage = messages.clone();

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages(currentMessage)
            .build()?;
        println!("Processed output: {}", output);

        let mut stream = client.chat().create_stream(request).await?;

        let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            write!(lock, "{}", content).unwrap();
                        }
                    });
                }
                Err(err) => {
                    writeln!(lock, "error: {err}").unwrap();
                }
            }
            stdout().flush()?;
        }
        println!("\n");
    }

    // For reasons not documented in OpenAI docs / OpenAPI spec,
    // the response of streaming call is different and doesn't include all the same fields.

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

    Ok(())
}
