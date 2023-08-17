use std::fmt::Display;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use stackable_operator::{cpu::CpuQuantity, memory::MemoryQuantity};

use crate::utils::k8s::{KubeClient, KubeClientError};

/// Demos and stacks can define how much cluster resources they need to run
/// via their definition. The struct [`ResourceRequests`] contains information
/// how many CPU cores and how much memory and disk space are required to run
/// the demo/stack.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ResourceRequests {
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub memory: Quantity,

    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub cpu: Quantity,

    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub pvc: Quantity,
}

impl Display for ResourceRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU: {}, Memory: {}, PVC space: {}",
            self.cpu.0, self.memory.0, self.pvc.0
        )
    }
}

/// This error indicates that the [`ResourceRequests`] of a stack or a demo
/// can not be parsed or validation of those requests failed.
#[derive(Debug, Snafu)]
pub enum ResourceRequestsError {
    #[snafu(display("kube error: {source}"), context(false))]
    KubeError { source: KubeClientError },

    #[snafu(display("failed to parse cpu resource requirements"))]
    ParseCpuResourceRequirements {
        source: stackable_operator::error::Error,
    },

    #[snafu(display("failed to parse memory resource requirements"))]
    ParseMemoryResourceRequirements {
        source: stackable_operator::error::Error,
    },

    #[snafu(display("invalid resource requirements"))]
    ValidationErrors {
        errors: Vec<ResourceRequestsValidationError>,
    },
}

#[derive(Debug, Snafu)]
pub enum ResourceRequestsValidationError {
    #[snafu(display(
        "The {object_name} requires {} CPU cores, but there are only {} CPU cores available in the cluster", required.as_cpu_count(), available.as_cpu_count()
    ))]
    InsufficientCpu {
        available: CpuQuantity,
        required: CpuQuantity,
        object_name: String,
    },

    #[snafu(display(
        "The {object_name} requires {required} of memory, but there are only {available} of memory available in the cluster"
    ))]
    InsufficientMemory {
        available: MemoryQuantity,
        required: MemoryQuantity,
        object_name: String,
    },
}

impl ResourceRequests {
    /// Validates the struct [`ResourceRequests`] by comparing the required
    /// resources to the available ones in the current cluster. `object_name`
    /// should be `stack` or `demo`.
    pub async fn validate_cluster_size(
        &self,
        object_name: &str,
    ) -> Result<(), ResourceRequestsError> {
        let kube_client = KubeClient::new().await?;
        let cluster_info = kube_client.get_cluster_info().await?;

        let stack_cpu =
            CpuQuantity::try_from(&self.cpu).context(ParseCpuResourceRequirementsSnafu)?;
        let stack_memory =
            MemoryQuantity::try_from(&self.memory).context(ParseMemoryResourceRequirementsSnafu)?;

        // The above errors are "hard" errors which cannot be recovered and
        // should be handled by the caller. The errors below get collected
        // before returning to provide the caller (and user) with more
        // information during troubleshooting.
        let mut errors = Vec::new();

        if stack_cpu > cluster_info.untainted_allocatable_cpu {
            errors.push(ResourceRequestsValidationError::InsufficientCpu {
                available: cluster_info.untainted_allocatable_cpu,
                object_name: object_name.to_string(),
                required: stack_cpu,
            });
        }

        if stack_memory > cluster_info.untainted_allocatable_memory {
            errors.push(ResourceRequestsValidationError::InsufficientMemory {
                available: cluster_info.untainted_allocatable_memory,
                object_name: object_name.to_string(),
                required: stack_memory,
            });
        }

        if !errors.is_empty() {
            Err(ResourceRequestsError::ValidationErrors { errors })
        } else {
            Ok(())
        }
    }
}
