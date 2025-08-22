use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, warn, error, debug};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use opentelemetry_proto::tonic::collector::{
    metrics::v1::{
        metrics_service_server::{MetricsService, MetricsServiceServer},
        ExportMetricsServiceRequest, ExportMetricsServiceResponse,
    },
    logs::v1::{
        logs_service_server::{LogsService, LogsServiceServer}, 
        ExportLogsServiceRequest, ExportLogsServiceResponse,
    },
};

use crate::storage::{Database, DatabaseError, MetricRecord, LogRecord};
use crate::otel::metrics::{EnhancedClaudeMetric, MetricClassifier};

#[derive(Clone)]
pub struct OtelReceiver {
    db: Arc<dyn Database>,
}

impl OtelReceiver {
    pub fn new(db: Arc<dyn Database>) -> Self {
        Self { db }
    }
}

// Claude Code specific metric types
#[derive(Debug, Clone)]
pub struct ClaudeCodeMetric {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub labels: HashMap<String, String>,
    pub session_id: Option<String>,
}

// Claude Code specific log event
#[derive(Debug, Clone)]
pub struct ClaudeCodeEvent {
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub attributes: HashMap<String, String>,
    pub session_id: Option<String>,
}

#[tonic::async_trait]
impl MetricsService for OtelReceiver {
    async fn export(
        &self,
        request: Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, Status> {
        let req = request.into_inner();
        
        info!("Received {} metric resource(s)", req.resource_metrics.len());
        
        let mut metrics_to_store = Vec::new();
        
        // Process each resource metric
        for resource_metrics in req.resource_metrics {
            // Extract resource attributes
            let mut resource_attrs = HashMap::new();
            if let Some(resource) = resource_metrics.resource {
                for attr in resource.attributes {
                    if let Some(value) = attr.value {
                        if let Some(value_data) = value.value {
                            resource_attrs.insert(attr.key, extract_attribute_value(value_data));
                        }
                    }
                }
            }
            
            // Process scope metrics
            for scope_metrics in resource_metrics.scope_metrics {
                for metric in scope_metrics.metrics {
                    let metric_name = metric.name.clone();
                    match parse_claude_code_metric(metric, &resource_attrs) {
                        Ok(parsed_metrics) => {
                            for claude_metric in parsed_metrics {
                                debug!("Processing Claude Code metric: {} = {}", 
                                    claude_metric.name, claude_metric.value);
                                
                                // Create enhanced metric with user context
                                let enhanced_metric = EnhancedClaudeMetric::from_basic_metric(
                                    claude_metric.name.clone(),
                                    claude_metric.value,
                                    claude_metric.timestamp,
                                    claude_metric.labels.clone(),
                                );
                                
                                debug!("Enhanced metric type: {:?}, User: {:?}", 
                                    enhanced_metric.metric_type, enhanced_metric.user_email);
                                
                                let metric_record = MetricRecord {
                                    id: Uuid::new_v4(),
                                    session_id: enhanced_metric.session_id
                                        .and_then(|s| Uuid::parse_str(&s).ok()),
                                    name: enhanced_metric.name,
                                    timestamp: enhanced_metric.timestamp,
                                    value: enhanced_metric.value,
                                    labels: enhanced_metric.labels,
                                    created_at: Utc::now(),
                                };
                                
                                metrics_to_store.push(metric_record);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse metric {}: {}", metric_name, e);
                        }
                    }
                }
            }
        }
        
        // Batch store metrics
        if !metrics_to_store.is_empty() {
            match store_metrics_batch(&*self.db, metrics_to_store).await {
                Ok(_) => info!("Successfully stored metrics batch"),
                Err(e) => error!("Failed to store metrics: {}", e),
            }
        }
        
        Ok(Response::new(ExportMetricsServiceResponse {
            partial_success: None,
        }))
    }
}

#[tonic::async_trait]
impl LogsService for OtelReceiver {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        let req = request.into_inner();
        
        info!("Received {} log resource(s)", req.resource_logs.len());
        
        let mut logs_to_store = Vec::new();
        
        // Process each resource log
        for resource_logs in req.resource_logs {
            // Extract resource attributes
            let mut resource_attrs = HashMap::new();
            if let Some(resource) = resource_logs.resource {
                for attr in resource.attributes {
                    if let Some(value) = attr.value {
                        if let Some(value_data) = value.value {
                            resource_attrs.insert(attr.key, extract_attribute_value(value_data));
                        }
                    }
                }
            }
            
            // Process scope logs
            for scope_logs in resource_logs.scope_logs {
                for log_record in scope_logs.log_records {
                    match parse_claude_code_event(log_record, &resource_attrs) {
                        Ok(claude_event) => {
                            debug!("Processing Claude Code event: {}", claude_event.event_type);
                            
                            let log_record = LogRecord {
                                id: Uuid::new_v4(),
                                session_id: claude_event.session_id
                                    .and_then(|s| Uuid::parse_str(&s).ok()),
                                timestamp: claude_event.timestamp,
                                level: "INFO".to_string(), // Claude Code events are typically info level
                                message: claude_event.event_type.clone(),
                                attributes: claude_event.attributes,
                                created_at: Utc::now(),
                            };
                            
                            logs_to_store.push(log_record);
                        }
                        Err(e) => {
                            warn!("Failed to parse log record: {}", e);
                        }
                    }
                }
            }
        }
        
        // Batch store logs
        if !logs_to_store.is_empty() {
            match store_logs_batch(&*self.db, logs_to_store).await {
                Ok(_) => info!("Successfully stored logs batch"),
                Err(e) => error!("Failed to store logs: {}", e),
            }
        }
        
        Ok(Response::new(ExportLogsServiceResponse {
            partial_success: None,
        }))
    }
}

// Parse Claude Code specific metrics
fn parse_claude_code_metric(
    metric: opentelemetry_proto::tonic::metrics::v1::Metric,
    resource_attrs: &HashMap<String, String>,
) -> Result<Vec<ClaudeCodeMetric>, String> {
    let mut parsed_metrics = Vec::new();
    
    // Extract session ID from resource attributes
    let session_id = resource_attrs.get("session.id").cloned();
    
    // Handle different metric data types
    if let Some(data) = metric.data {
        use opentelemetry_proto::tonic::metrics::v1::metric::Data;
        
        match data {
            Data::Gauge(gauge) => {
                for data_point in gauge.data_points {
                    let mut labels = extract_labels(&data_point.attributes);
                    
                    // Add resource attributes as labels
                    labels.extend(resource_attrs.clone());
                    
                    let timestamp = timestamp_from_nanos(data_point.time_unix_nano);
                    
                    let value = match data_point.value {
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsDouble(v)) => v,
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsInt(v)) => v as f64,
                        None => 0.0,
                    };
                    
                    parsed_metrics.push(ClaudeCodeMetric {
                        name: metric.name.clone(),
                        value,
                        timestamp,
                        labels,
                        session_id: session_id.clone(),
                    });
                }
            }
            Data::Sum(sum) => {
                for data_point in sum.data_points {
                    let mut labels = extract_labels(&data_point.attributes);
                    labels.extend(resource_attrs.clone());
                    
                    let timestamp = timestamp_from_nanos(data_point.time_unix_nano);
                    
                    let value = match data_point.value {
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsDouble(v)) => v,
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsInt(v)) => v as f64,
                        None => 0.0,
                    };
                    
                    parsed_metrics.push(ClaudeCodeMetric {
                        name: metric.name.clone(),
                        value,
                        timestamp,
                        labels,
                        session_id: session_id.clone(),
                    });
                }
            }
            Data::Histogram(histogram) => {
                for data_point in histogram.data_points {
                    let mut labels = extract_labels(&data_point.attributes);
                    labels.extend(resource_attrs.clone());
                    
                    let timestamp = timestamp_from_nanos(data_point.time_unix_nano);
                    
                    // For histograms, we'll store the count and sum as separate metrics
                    if data_point.count > 0 {
                        parsed_metrics.push(ClaudeCodeMetric {
                            name: format!("{}_count", metric.name),
                            value: data_point.count as f64,
                            timestamp,
                            labels: labels.clone(),
                            session_id: session_id.clone(),
                        });
                    }
                    
                    if let Some(sum) = data_point.sum {
                        parsed_metrics.push(ClaudeCodeMetric {
                            name: format!("{}_sum", metric.name),
                            value: sum,
                            timestamp,
                            labels,
                            session_id: session_id.clone(),
                        });
                    }
                }
            }
            _ => {
                return Err(format!("Unsupported metric data type for {}", metric.name));
            }
        }
    }
    
    Ok(parsed_metrics)
}

// Parse Claude Code specific log events
fn parse_claude_code_event(
    log_record: opentelemetry_proto::tonic::logs::v1::LogRecord,
    resource_attrs: &HashMap<String, String>,
) -> Result<ClaudeCodeEvent, String> {
    let mut attributes = extract_log_attributes(&log_record.attributes);
    
    // Add resource attributes
    attributes.extend(resource_attrs.clone());
    
    let session_id = resource_attrs.get("session.id").cloned();
    
    let timestamp = timestamp_from_nanos(log_record.time_unix_nano);
    
    // Extract event type from body or attributes
    let event_type = if let Some(body) = log_record.body {
        extract_log_body_string(body).unwrap_or_else(|| "unknown_event".to_string())
    } else {
        attributes.get("event.name")
            .or_else(|| attributes.get("event_type"))
            .cloned()
            .unwrap_or_else(|| "unknown_event".to_string())
    };
    
    Ok(ClaudeCodeEvent {
        event_type,
        timestamp,
        attributes,
        session_id,
    })
}

// Helper functions
fn extract_attribute_value(
    value: opentelemetry_proto::tonic::common::v1::any_value::Value
) -> String {
    use opentelemetry_proto::tonic::common::v1::any_value::Value;
    
    match value {
        Value::StringValue(s) => s,
        Value::IntValue(i) => i.to_string(),
        Value::DoubleValue(d) => d.to_string(),
        Value::BoolValue(b) => b.to_string(),
        Value::BytesValue(b) => String::from_utf8_lossy(&b).to_string(),
        Value::ArrayValue(array) => {
            // Convert array to JSON-like string
            let values: Vec<String> = array.values.into_iter()
                .map(|v| v.value.map_or_else(|| "null".to_string(), extract_attribute_value))
                .collect();
            format!("[{}]", values.join(", "))
        }
        Value::KvlistValue(kvlist) => {
            // Convert key-value list to JSON-like string
            let pairs: Vec<String> = kvlist.values.into_iter()
                .map(|kv| {
                    let value_str = kv.value
                        .and_then(|v| v.value)
                        .map_or_else(|| "null".to_string(), extract_attribute_value);
                    format!("\"{}\":\"{}\"", kv.key, value_str)
                })
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

fn extract_labels(
    attributes: &[opentelemetry_proto::tonic::common::v1::KeyValue]
) -> HashMap<String, String> {
    let mut labels = HashMap::new();
    
    for attr in attributes {
        if let Some(value) = &attr.value {
            if let Some(value_data) = &value.value {
                labels.insert(attr.key.clone(), extract_attribute_value(value_data.clone()));
            }
        }
    }
    
    labels
}

fn extract_log_attributes(
    attributes: &[opentelemetry_proto::tonic::common::v1::KeyValue]
) -> HashMap<String, String> {
    extract_labels(attributes)
}

fn extract_log_body_string(
    body: opentelemetry_proto::tonic::common::v1::AnyValue
) -> Option<String> {
    body.value.map(extract_attribute_value)
}

fn timestamp_from_nanos(nanos: u64) -> DateTime<Utc> {
    if nanos == 0 {
        return Utc::now();
    }
    let seconds = nanos / 1_000_000_000;
    let nanoseconds = (nanos % 1_000_000_000) as u32;
    
    DateTime::from_timestamp(seconds as i64, nanoseconds)
        .unwrap_or_else(Utc::now)
}

// Batch processing functions
async fn store_metrics_batch(
    db: &dyn Database,
    metrics: Vec<MetricRecord>
) -> Result<(), DatabaseError> {
    // Store metrics in batches for better performance
    const BATCH_SIZE: usize = 100;
    
    for chunk in metrics.chunks(BATCH_SIZE) {
        for metric in chunk {
            db.store_metric(metric).await?;
        }
    }
    
    Ok(())
}

async fn store_logs_batch(
    db: &dyn Database,
    logs: Vec<LogRecord>
) -> Result<(), DatabaseError> {
    // Store logs in batches for better performance  
    const BATCH_SIZE: usize = 100;
    
    for chunk in logs.chunks(BATCH_SIZE) {
        for log in chunk {
            db.store_log(log).await?;
        }
    }
    
    Ok(())
}

// Main server startup function
pub async fn start_otel_server(
    addr: SocketAddr,
    db: Arc<dyn Database>,
) -> Result<(), Box<dyn std::error::Error>> {
    let otel_receiver = OtelReceiver::new(db);

    info!("OpenTelemetry gRPC server listening on {}", addr);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!("../../opentelemetry_descriptor.bin"))
        .build()
        .unwrap_or_else(|e| {
            warn!("Could not build reflection service: {}", e);
            panic!("Failed to build reflection service");
        });

    Server::builder()
        .add_service(MetricsServiceServer::new(otel_receiver.clone()))
        .add_service(LogsServiceServer::new(otel_receiver))
        .add_service(tonic_web::enable(reflection_service))
        .serve(addr)
        .await
        .map_err(|e| {
            error!("OpenTelemetry server error: {}", e);
            e.into()
        })
}