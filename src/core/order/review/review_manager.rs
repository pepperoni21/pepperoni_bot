use serenity::{model::{prelude::{component::ButtonStyle, UserId}, user::User}, prelude::Mentionable, futures::StreamExt};
use wither::{Model, bson::{doc, to_bson}};

use crate::{ContextHTTP, core::order::{models::order::Order, state::order_state::{self, OrderState}}, bot::Bot, utils::channel_utils};

use super::models::{review_rating::ReviewRating, review::Review};

pub async fn load(context_http: &ContextHTTP) {
    generate_message(context_http).await;
}

async fn generate_message(context_http: &ContextHTTP) {
    let make_review_channel = channel_utils::fetch_guild_channel("MAKE_REVIEW_CHANNEL_ID", context_http).await;

    let history = make_review_channel
        .messages(context_http, |retriever| retriever.limit(1))
        .await
        .expect("Failed to get make review channel messages");

    if history.is_empty() {
        make_review_channel.send_message(context_http, |message| {
            message.embed(|embed|
                embed
                    .title("Make a Review")
                    .description("**To make a review, you need to have made an order. Then, you can click on the button below to review your order.**\n\nNo review will be deleted, no matter what you write. However, I let myself the right to reply.")
            ).components(|components|
                components.create_action_row(|action_row|
                    action_row.create_button(|button|
                        button
                            .style(ButtonStyle::Success)
                            .custom_id("review")
                            .label("Make a Review")
                    )
                )
            )
        }).await.expect("Failed to send message to make review channel");
    }
}

pub async fn add_review(bot: &Bot, context_http: &ContextHTTP, user: &User, order_id: i32, review_rating: ReviewRating, comment: String) -> Result<(), String>{
    let db = &bot.db_info.db;

    let mut order: Order = Order::find_one(db, doc! {"order_id": order_id}, None).await.expect("Failed to find order").expect("Order not found");

    let reviews_channel = channel_utils::fetch_guild_channel("REVIEWS_CHANNEL_ID", context_http).await;

    let message = reviews_channel.send_message(context_http, |message|
        message.embed(|embed|
            embed.title(format!("Order #{}", order_id))
                .field("Customer", user.mention(), false)
                .field("Rating", review_rating.get_emoji(), false)
                .field("Comment", &comment, true)
                .author(|author| author.name(&user.name).icon_url(&user.face()))
        )
    ).await;

    let message = message.expect("Failed to send message to review channel");

    let review = Review {
        rating: review_rating,
        comment,
        message_id: message.id.0,
    };
    
    order.review = Some(review);
    order.save(db, None).await.expect("Failed to save order");

    Ok(())
}

pub async fn can_review(bot: &Bot, user_id: UserId) -> Option<Vec<Order>> {
    let db = &bot.db_info.db;

    let orders = Order::find(db, doc! {
        "customer_id": to_bson(&user_id.0).unwrap(),
        "order_state_id": order_state::DELIVERED_STATE.id(),
        "review": to_bson(&None::<Review>).unwrap()
    }, None).await.expect("Failed to find orders")
    .map(|order| order.expect("Failed to get order"))
    .collect::<Vec<Order>>().await;

    if orders.is_empty() {
        None
    } else {
        Some(orders)
    }
}