use std::sync::Arc;

use crate::{EmailAlertsGrpc, MerchantFlowsGrpc, SettingsModel, SwapOperationsListener, OrderExecutionOperationsListener};
use my_service_bus_tcp_client::MyServiceBusClient;
use rust_extensions::AppStates;

use my_service_bus_abstractions::subscriber::TopicQueueType;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const MY_SB_QUEUE_NAME: &str = "alerts-producer";

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    pub my_sb_connection: MyServiceBusClient,
    pub merchant_flows_grpc_client: MerchantFlowsGrpc,
    pub email_alerts_grpc_client: EmailAlertsGrpc,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        let settings = settings_reader.get_settings().await;
        let app_states = Arc::new(AppStates::create_initialized());

        let my_sb_connection = MyServiceBusClient::new(
            APP_NAME,
            APP_VERSION,
            settings_reader.clone(),
            my_logger::LOGGER.clone(),
        );

        let merchant_flows_grpc_client =
            MerchantFlowsGrpc::new(settings.merchant_flows_grpc_url.clone()).await;
        let email_alerts_grpc_client =
            EmailAlertsGrpc::new(settings.email_alerts_grpc_url.clone()).await;

        Self {
            app_states,
            settings,
            my_sb_connection,
            merchant_flows_grpc_client,
            email_alerts_grpc_client,
        }
    }
}

pub async fn bind_sb_subscribers(app: Arc<AppContext>) {
    app.my_sb_connection
        .subscribe(
            MY_SB_QUEUE_NAME.to_string(),
            TopicQueueType::Permanent,
            Arc::new(SwapOperationsListener::new(app.clone())),
        )
        .await;

    app.my_sb_connection
        .subscribe(
            MY_SB_QUEUE_NAME.to_string(),
            TopicQueueType::Permanent,
            Arc::new(OrderExecutionOperationsListener::new(app.clone())),
        )
        .await;
}
