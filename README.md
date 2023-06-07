# AI Chat Assistant

This is a simple chat assistant program written in Rust that utilizes the OpenAI GPT-3.5 model to provide responses to user prompts.

## Prerequisites

- Rust programming language
- OpenAI API credentials

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/your-username/chat-bot-openAI.git
   ```

2. Change into the project directory:

   ```
   cd chat-bot-openAI
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Set up your OpenAI API credentials by creating a `.env` file in the project root directory and adding the following lines:

   ```
   OPENAI_API_KEY=your-api-key
   ```

## Usage

1. Run the program:

   ```
   cargo run
   ```

2. The chat assistant will start and prompt you to enter a message.

3. Enter your message and press Enter.

4. The assistant will process your input and provide a response.

5. Continue the conversation by entering more prompts.

6. To quit the program, enter "quit" when prompted.

## Customization

You can modify the behavior of the chat assistant by implementing your own logic in the `process_input` function. This function processes the user's input and returns the processed output, which is then used as the user's message in the chat.

## Creating a New Chat

If you want to start a new chat session and clear the chat history, you can enter "create new chat" when prompted. This will reset the messages and allow you to start a fresh conversation.

## Dependencies

- dotenv = "0.15.0"
- async-openai = "0.7.0"
- futures = "0.3.17"

## License

This project is licensed under the [MIT License](LICENSE).
