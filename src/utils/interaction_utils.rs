use serenity::model::prelude::{interaction::{message_component::MessageComponentInteraction, InteractionResponseType, modal::ModalSubmitInteraction, application_command::ApplicationCommandInteraction, Interaction}, GuildId};

use crate::ContextHTTP;

pub async fn reply_message_component<S: ToString>(context_http: &ContextHTTP, interaction: &MessageComponentInteraction, content: S) {
    interaction
        .create_interaction_response(context_http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content).ephemeral(true))
        }).await.expect("Failed to send interaction response");
}

pub async fn _reply_modal_submit<S: ToString>(context_http: &ContextHTTP, interaction: &ModalSubmitInteraction, content: S) {
    interaction
        .create_interaction_response(context_http, |response| {
            response
                .kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|message| message.content(content).ephemeral(true))
        }).await.expect("Failed to send interaction response");
}

pub async fn reply_application_command<S: ToString>(context_http: &ContextHTTP, interaction: &ApplicationCommandInteraction, content: S) {
    interaction
        .create_interaction_response(context_http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content).ephemeral(true))
        }).await.expect("Failed to send interaction response");
}

pub fn get_interaction_guild(interaction: &Interaction) -> Option<GuildId> {
    match interaction {
        Interaction::MessageComponent(interaction) => interaction.guild_id,
        Interaction::ApplicationCommand(interaction) => interaction.guild_id,
        Interaction::ModalSubmit(interaction) => interaction.guild_id,
        _ => None,
    }
}