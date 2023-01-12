use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,
    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,
    #[serde(rename = "MySbConnection")]
    pub my_service_bus_connection: String,
    #[serde(rename = "MerchantFlowsGrpcUrl")]
    pub merchant_flows_grpc_url: String,
    #[serde(rename = "EmailAlertsGrpcUrl")]
    pub email_alerts_grpc_url: String,
}

#[async_trait::async_trait]
impl my_seq_logger::SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.seq_conn_string.clone()
    }
}

#[async_trait::async_trait]
impl my_telemetry_writer::MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_telemetry.clone()
    }
}

#[async_trait::async_trait]
impl my_service_bus_tcp_client::MyServiceBusSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_service_bus_connection.clone()
    }
}
