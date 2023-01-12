use std::sync::Arc;

use my_sb_contracts::OrderExecutionOperation;
use my_service_bus_abstractions::subscriber::{
    MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
};
use my_telemetry::MyTelemetryContext;

use crate::AppContext;

pub struct OrderExecutionOperationsListener {
    pub app: Arc<AppContext>,
}

impl OrderExecutionOperationsListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<OrderExecutionOperation> for OrderExecutionOperationsListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<OrderExecutionOperation>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let order_execution_operation = message.take_message();
            let telemetry = MyTelemetryContext::new();
            let step = order_execution_operation.steps.first();

            if let Some(step) = step{
                let is_merchant = self
                    .app
                    .merchant_flows_grpc_client
                    .is_merchant(step.to_client_id.clone(), &telemetry)
                    .await;

                if is_merchant {
                    self.app
                        .email_alerts_grpc_client
                        .send_order_execution_email(
                            step.to_client_id.clone(),
                            step.delta,
                            order_execution_operation.currency,
                            order_execution_operation.order_id,
                            &telemetry,
                        )
                        .await;
                }
            }
        }
        Ok(())
    }
}
