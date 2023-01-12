use std::sync::Arc;

use my_sb_contracts::SwapOperation;
use my_service_bus_abstractions::subscriber::{
    MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
};
use my_telemetry::MyTelemetryContext;

use crate::AppContext;

pub struct SwapOperationsListener {
    pub app: Arc<AppContext>,
}

impl SwapOperationsListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<SwapOperation> for SwapOperationsListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<SwapOperation>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let swap_operation = message.take_message();
            let telemetry = MyTelemetryContext::new();
            let is_merchant = self
                .app
                .merchant_flows_grpc_client
                .is_merchant(swap_operation.client_id.clone(), &telemetry)
                .await;

            if is_merchant {
                self.app
                    .email_alerts_grpc_client
                    .send_merchant_swap_email(
                        swap_operation.client_id,
                        swap_operation.sell_amount,
                        swap_operation.sell_asset,
                        swap_operation.buy_asset,
                        &telemetry,
                    )
                    .await;
            }
        }
        Ok(())
    }
}
