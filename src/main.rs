use teloxide::prelude::*;
use teloxide::types::{Chat, User};

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting janitor...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        let chat = &message.update.chat;
        if chat.is_private() {
            return ResponseResult::<()>::Ok(());
        }

        if message.update.left_chat_member().is_some() {
            message.delete_message().send().await?;
        }

        if let Some(users) = message.update.new_chat_members() {
            for user in users {
                kick_user(&message, chat, user).await?;
                log::info!(
                    "The user has been kicked from the chat. ChatID = {}, UserID = {}, Username = {:?}",
                    chat.id,
                    user.id,
                    user.username
                );
            }
            message.delete_message().send().await?;
        }

        ResponseResult::<()>::Ok(())
    })
    .await;
}

async fn kick_user(
    message: &UpdateWithCx<Message>,
    chat: &Chat,
    user: &User,
) -> Result<(), RequestError> {
    message
        .bot
        .kick_chat_member(chat.id, user.id)
        .send()
        .await?;
    if chat.is_supergroup() {
        message
            .bot
            .unban_chat_member(chat.id, user.id)
            .send()
            .await?;
    }

    ResponseResult::<()>::Ok(())
}
