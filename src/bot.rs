use std::{env, sync::Arc};

use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::{Ready, GuildId, interaction::Interaction}};

use crate::{core::{order::{order_manager::OrderManager, command::order_command_executor}, db, developers::{developer_manager::DeveloperManager, command::developer_command_executor}}, ContextHTTP};

pub struct Bot {
    pub db_info: db::DBInfo,
    pub guild_id: GuildId,
    pub order_manager: Arc<OrderManager>,
    pub developer_manager: DeveloperManager
}

impl Bot {

    pub async fn new() -> Self {
        let db_info = db::DBInfo::new().await;
        let guild_id = GuildId(env::var("GUILD_ID")
            .expect("Expected a GUILD_ID in the environment")
            .parse()
            .expect("GUILD_ID is not a valid ID"));
        let bot = Self {
            db_info,
            guild_id,
            order_manager: Arc::new(OrderManager::new().await),
            developer_manager: DeveloperManager
        };

        bot
    }

    async fn load(&self, context_http: ContextHTTP){
        println!("Connected to Discord!");

        self.order_manager.load(self, &context_http).await;
        self.developer_manager.load(self, &context_http).await;
    }

}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        self.load(ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction){
        let context_http: ContextHTTP = ctx.http;

        self.order_manager.review_manager.listener.on_interaction(self, &context_http, interaction.clone()).await;
        self.order_manager.listener.on_interaction(self, &context_http, interaction.clone()).await;
        order_command_executor::on_interaction(&self, &context_http, interaction.clone()).await;
        developer_command_executor::on_interaction(&self, &context_http, interaction.clone()).await;
    }
}
