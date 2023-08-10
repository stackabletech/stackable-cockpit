//! Please be aware that this file is mostly copy/paste from <https://github.com/stackabletech/stackablectl/blob/eda45945cfcf5c6581cf1b88c782d98fada8065f/src/kube.rs#L48-L187>
//! It does only support services of type NodePort and LoadBalancer.
//! This code will be removed entirely once listener-op is implemented in all operators,
//! the endpoints will than be read by some sort of listener-operator API.
//!
//! So there is no point in optimizing this code or to blame the ones who have wrote it ;P

use std::collections::HashMap;

use indexmap::IndexMap;
use k8s_openapi::api::core::v1::{Endpoints, Node, Service, ServiceSpec};
use kube::{api::ListParams, Api, Client, ResourceExt};
use snafu::{OptionExt, ResultExt, Snafu};
use tracing::{debug, warn};

#[derive(Debug, Snafu)]
pub enum ServiceError {
    #[snafu(display("kube error: {source}"))]
    KubeError { source: kube::error::Error },

    #[snafu(display("missing namespace for service '{service}'"))]
    MissingServiceNamespace { service: String },

    #[snafu(display("missing spec for service '{service}'"))]
    MissingServiceSpec { service: String },

    #[snafu(display("failed to get status of node {node_name}"))]
    GetNodeStatus { node_name: String },

    #[snafu(display("failed to get address of node {node_name}"))]
    GetNodeAddress { node_name: String },

    #[snafu(display("Could not find an ExternalIP or InternalIP for node {node_name}"))]
    NoIpForNode { node_name: String },

    #[snafu(display("failed to find node {node_name} in node_name_ip_mapping"))]
    NodeMissingInIpMapping { node_name: String },
}

pub async fn get_service_endpoint_urls(
    service: &Service,
    referenced_object_name: &str,
) -> Result<IndexMap<String, String>, ServiceError> {
    let client = get_client().await?;
    let service_name = service.name_unchecked();
    let namespace = service.namespace().context(MissingServiceNamespaceSnafu {
        service: service_name.clone(),
    })?;
    let service_spec = service.spec.as_ref().context(MissingServiceSpecSnafu {
        service: service_name.clone(),
    })?;

    let endpoints = match service_spec.type_.as_deref() {
        Some("NodePort") => {
            get_service_endpoint_urls_for_nodeport(
                client,
                &service_name,
                service_spec,
                &namespace,
                referenced_object_name,
            )
            .await?
        }
        Some("LoadBalancer") => {
            get_service_endpoint_urls_for_loadbalancer(
                &service_name,
                service,
                service_spec,
                referenced_object_name,
            )
            .await?
        }
        _ => IndexMap::new(),
    };

    Ok(endpoints)
}

pub async fn get_service_endpoint_urls_for_loadbalancer(
    service_name: &str,
    service: &Service,
    service_spec: &ServiceSpec,
    referenced_object_name: &str,
) -> Result<IndexMap<String, String>, ServiceError> {
    let mut endpoints = IndexMap::new();

    let lb_ip = service
        .status
        .as_ref()
        .unwrap()
        .load_balancer
        .as_ref()
        .unwrap()
        .ingress
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap()
        .ip
        .as_ref()
        .unwrap();

    for service_port in service_spec.ports.iter().flatten() {
        let lb_port = service_port.port;

        let endpoint_name = service_name
            .trim_start_matches(referenced_object_name)
            .trim_start_matches('-');

        let port_name = service_port
            .name
            .clone()
            .unwrap_or_else(|| lb_port.to_string());
        let endpoint_name = if endpoint_name.is_empty() {
            port_name.clone()
        } else {
            format!("{endpoint_name}-{port_name}")
        };

        // TODO: Consolidate web-ui port names in operators based on decision in arch meeting from 2022/08/10
        // For Superset: https://github.com/stackabletech/superset-operator/issues/248
        // For Airflow: https://github.com/stackabletech/airflow-operator/issues/146
        // As we still support older operator versions we need to also include the "old" way of naming
        let endpoint = if port_name == "http"
            || port_name.starts_with("http-")
            || port_name == "ui"
            || port_name == "airflow"
            || port_name == "superset"
        {
            format!("http://{lb_ip}:{lb_port}")
        } else if port_name == "https" || port_name.starts_with("https-") {
            format!("https://{lb_ip}:{lb_port}")
        } else {
            format!("{lb_ip}:{lb_port}")
        };

        endpoints.insert(endpoint_name, endpoint);
    }

    Ok(endpoints)
}

pub async fn get_service_endpoint_urls_for_nodeport(
    client: Client,
    service_name: &str,
    service_spec: &ServiceSpec,
    namespace: &str,
    referenced_object_name: &str,
) -> Result<IndexMap<String, String>, ServiceError> {
    let endpoints_api: Api<Endpoints> = Api::namespaced(client.clone(), namespace);
    let endpoints = endpoints_api.get(service_name).await.context(KubeSnafu)?;

    let node_name = match &endpoints.subsets {
        Some(subsets) if subsets.len() == 1 => match &subsets[0].addresses {
            Some(addresses) if !addresses.is_empty() => match &addresses[0].node_name {
                Some(node_name) => node_name,
                None => {
                    warn!("Could not determine the node the endpoint {service_name} is running on because the address of the subset didn't had a node name");
                    return Ok(IndexMap::new());
                }
            },
            Some(_) => {
                warn!("Could not determine the node the endpoint {service_name} is running on because the subset had no addresses");
                return Ok(IndexMap::new());
            }
            None => {
                warn!("Could not determine the node the endpoint {service_name} is running on because subset had no addresses. Is the service {service_name} up and running?");
                return Ok(IndexMap::new());
            }
        },
        Some(subsets) => {
            warn!("Could not determine the node the endpoint {service_name} is running on because endpoints consists of {num_subsets} subsets", num_subsets=subsets.len());
            return Ok(IndexMap::new());
        }
        None => {
            warn!("Could not determine the node the endpoint {service_name} is running on because the endpoint has no subset. Is the service {service_name} up and running?");
            return Ok(IndexMap::new());
        }
    };

    let node_ip = get_node_ip(&client, node_name).await?;

    let mut endpoints = IndexMap::new();
    for service_port in service_spec.ports.iter().flatten() {
        match service_port.node_port {
            Some(node_port) => {
                let endpoint_name = service_name
                    .trim_start_matches(referenced_object_name)
                    .trim_start_matches('-');

                let port_name = service_port
                    .name
                    .clone()
                    .unwrap_or_else(|| service_port.port.to_string());
                let endpoint_name = if endpoint_name.is_empty() {
                    port_name.clone()
                } else {
                    format!("{endpoint_name}-{port_name}")
                };

                // TODO: Consolidate web-ui port names in operators based on decision in arch meeting from 2022/08/10
                // For Superset: https://github.com/stackabletech/superset-operator/issues/248
                // For Airflow: https://github.com/stackabletech/airflow-operator/issues/146
                // As we still support older operator versions we need to also include the "old" way of naming
                let endpoint = if port_name == "http"
                    || port_name.starts_with("http-")
                    || port_name == "ui"
                    || port_name == "airflow"
                    || port_name == "superset"
                {
                    format!("http://{node_ip}:{node_port}")
                } else if port_name == "https" || port_name.starts_with("https-") {
                    format!("https://{node_ip}:{node_port}")
                } else {
                    format!("{node_ip}:{node_port}")
                };

                endpoints.insert(endpoint_name, endpoint);
            }
            None => debug!("Could not get endpoint_url as service {service_name} has no nodePort"),
        }
    }

    Ok(endpoints)
}

async fn get_node_ip(client: &Client, node_name: &str) -> Result<String, ServiceError> {
    let node_name_ip_mapping = get_node_name_ip_mapping(client).await?;

    match node_name_ip_mapping.get(node_name) {
        Some(node_ip) => Ok(node_ip.to_string()),
        None => NodeMissingInIpMappingSnafu { node_name }.fail(),
    }
}

// TODO(sbernauer): Add caching
async fn get_node_name_ip_mapping(
    client: &Client,
) -> Result<HashMap<String, String>, ServiceError> {
    let node_api: Api<Node> = Api::all(client.clone());
    let nodes = node_api
        .list(&ListParams::default())
        .await
        .context(KubeSnafu)?;

    let mut result = HashMap::new();
    for node in nodes {
        let node_name = node.name_unchecked();
        let preferred_node_ip = node
            .status
            .context(GetNodeStatusSnafu {
                node_name: node_name.to_string(),
            })?
            .addresses
            .context(GetNodeAddressSnafu {
                node_name: node_name.to_string(),
            })?
            .iter()
            .filter(|address| address.type_ == "InternalIP" || address.type_ == "ExternalIP")
            .min_by_key(|address| &address.type_) // ExternalIP (which we want) is lower than InternalIP
            .map(|address| address.address.clone())
            .context(NoIpForNodeSnafu {
                node_name: node_name.to_string(),
            })?;
        result.insert(node_name, preferred_node_ip);
    }

    Ok(result)
}

async fn get_client() -> Result<Client, ServiceError> {
    Client::try_default().await.context(KubeSnafu)
}
