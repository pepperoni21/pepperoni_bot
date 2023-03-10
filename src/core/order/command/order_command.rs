use enum_iterator::all;
use serenity::{model::{prelude::command::{CommandOptionType, CommandType, Command}, Permissions}, builder::CreateApplicationCommandOption};

use crate::{ContextHTTP, core::order::models::order_type::OrderType};

pub async fn load_command(context_http: &ContextHTTP){
    Command::create_global_application_command(context_http, |command|
        command
            .name("order")
            .description("Manager orders")
            .default_member_permissions(Permissions::MOVE_MEMBERS)
            .kind(CommandType::ChatInput)
            .create_option(|option| {
                fill_create_command(option);
                option
            })
            .create_option(|option| {
                fill_cancel_command(option);
                option
            })
    )
    .await
    .expect("Failed to load commands");
}


fn fill_create_command(option: &mut CreateApplicationCommandOption){
    option
                .name("create")
                .description("Create an order")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|user_option|
                    user_option
                    .name("user")
                    .description("User who ordered")
                    .kind(CommandOptionType::User)
                    .required(true)
                ).create_sub_option(|type_option| {
                    type_option
                    .name("type")
                    .description("Type of order")
                    .kind(CommandOptionType::String)
                    .required(true);

                    all::<OrderType>().into_iter().for_each(|order_type| {
                        type_option.add_string_choice(order_type.get_display_name(), order_type.get_value());
                    });

                    type_option
                }).create_sub_option(|price_option|
                    price_option
                    .name("price")
                    .description("Price of order")
                    .kind(CommandOptionType::Integer)
                    .required(true)
                ).create_sub_option(|description_option|
                    description_option
                    .name("description")
                    .description("Description of order")
                    .kind(CommandOptionType::String)
                    .required(true)
                );
}

fn fill_cancel_command(option: &mut CreateApplicationCommandOption){
    option
    .name("cancel")
    .description("Cancel an order")
    .kind(CommandOptionType::SubCommand)
    .create_sub_option(|id_option|
        id_option
        .name("id")
        .description("Id of order")
        .kind(CommandOptionType::Integer)
        .required(true)
    );
}
