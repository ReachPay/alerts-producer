use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use std::time::Duration;

use tonic::{codegen::InterceptedService, transport::Channel};

use crate::alerts_service_grpc::{
    alerts_service_client::AlertsServiceClient, SendMerchantExchangeAlertRequest,
    SendMerchantOrderExecuteAlertRequest,
};

pub struct EmailAlertsGrpc {
    timeout: Duration,
    channel: Channel,
}

impl EmailAlertsGrpc {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            timeout: Duration::from_secs(1),
            channel,
        }
    }

    async fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> AlertsServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        return AlertsServiceClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );
    }

    pub async fn send_merchant_swap_email(
        &self,
        merchant_id: String,
        amount: f64,
        currency_from: String,
        currency_to: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantExchangeAlertRequest {
            merchant_id,
            amount,
            currency_from,
            currency_to,
        };

        let request_future = client.send_merchant_exchange_alert(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn send_order_execution_email(
        &self,
        merchant_id: String,
        amount: f64,
        currency: String,
        order_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantOrderExecuteAlertRequest {
            merchant_id,
            amount,
            currency,
            order_id,
        };

        let request_future = client.send_merchant_order_execute_alert(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }
}
