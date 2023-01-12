use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use std::time::Duration;

use tonic::{codegen::InterceptedService, transport::Channel};

use crate::merchantflows_grpc::{merchant_flows_service_client::MerchantFlowsServiceClient, *};

pub struct MerchantFlowsGrpc {
    timeout: Duration,
    channel: Channel,
}

impl MerchantFlowsGrpc {
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
    ) -> MerchantFlowsServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        return MerchantFlowsServiceClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );
    }

    pub async fn is_merchant(
        &self,
        client_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> bool {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsIsMerchantRequest { client_id };

        let request_future = client.is_merchant(request);

        let response = tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();

        response.into_inner().is_merchant
    }

    pub async fn get_merchant_settings(
        &self,
        client_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Option<MerchantFlowsMerchantSettingsModel> {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsGetMerchantSettingsRequest { client_id };

        let request_future = client.get_merchant_settings(request);

        let response = tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();

        response.into_inner().settings
    }

    pub async fn get_merchant_logs(
        &self,
        client_id: String,
        from: i64,
        to: i64,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Vec<MerchantFlowsCallbackLogModel> {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsGetCallbacksLogsRequest {
            from,
            to,
            client_id,
        };

        let request_future = client.get_merchant_callback_logs(request);

        let response = tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();

        let result =
            my_grpc_extensions::read_grpc_stream::as_vec(response.into_inner(), self.timeout)
                .await
                .unwrap();

        return match result {
            Some(data) => data,
            None => vec![],
        };
    }

    pub async fn add_merchant_callback_url(
        &self,
        client_id: String,
        callback_url: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsAddCallbackUrlRequest {
            callback_url,
            client_id,
        };

        let request_future = client.add_callback_url(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn delete_merchant_callback_url(
        &self,
        client_id: String,
        callback_url: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsRemoveCallbackUrlRequest {
            callback_url,
            client_id,
        };

        let request_future = client.remove_callback_url(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn add_callback_override(
        &self,
        client_id: String,
        currency: String,
        network: String,
        callback_type: MerchantFlowsCallbackOverrideType,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsAddMerchantOverrideRequest {
            client_id: client_id,
            merchant_override: Some(MerchantFlowsMerchantOverrideModel {
                currency,
                network,
                override_type: callback_type as i32,
            }),
        };

        let request_future = client.add_merchant_callback_override(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn bulk_callback_settings_update(
        &self,
        client_id: String,
        callback_type: MerchantFlowsCallbackOverrideType,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsSetAllCallbackOverrideOverridesRequest {
            client_id,
            override_type: callback_type as i32,
        };

        let request_future = client.set_all_callback_override(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn reset_default_merchant_callback_settings(
        &self,
        client_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = MerchantFlowsGetMerchantOverridesRequest { client_id };

        let request_future = client.reset_default_callback_settings(request);

        tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn get_merchant_callback_overrides(
        &self,
        client_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Vec<MerchantFlowsMerchantOverrideModel> {
        let mut client = self.create_grpc_service(my_telemetry_context).await;

        let request_future =
            client.get_merchant_callback_overrides(MerchantFlowsGetMerchantOverridesRequest {
                client_id,
            });

        let response = tokio::time::timeout(self.timeout, request_future)
            .await
            .unwrap()
            .unwrap();

        let result =
            my_grpc_extensions::read_grpc_stream::as_vec(response.into_inner(), self.timeout)
                .await
                .unwrap();

        return match result {
            Some(data) => data,
            None => vec![],
        };
    }
}
