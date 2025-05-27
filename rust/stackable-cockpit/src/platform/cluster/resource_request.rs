use std::fmt::Display;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use stackable_operator::{cpu::CpuQuantity, memory::MemoryQuantity};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::utils::k8s::{Client, Error};

type Result<T, E = ResourceRequestsError> = std::result::Result<T, E>;

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
            "CPU: {cpu}, Memory: {memory}, PVC space: {pvc}",
            cpu = self.cpu.0,
            memory = self.memory.0,
            pvc = self.pvc.0
        )
    }
}

/// This error indicates that the [`ResourceRequests`] of a stack or a demo
/// can not be parsed or validation of those requests failed.
#[derive(Debug, Snafu)]
pub enum ResourceRequestsError {
    #[snafu(display("failed to create kube client"))]
    KubeClientCreate { source: Error },

    #[snafu(display("failed to retrieve cluster info"))]
    ClusterInfo { source: Error },

    #[snafu(display("failed to parse cpu resource requirements"))]
    ParseCpuResourceRequirements {
        source: stackable_operator::cpu::Error,
    },

    #[snafu(display("failed to parse memory resource requirements"))]
    ParseMemoryResourceRequirements {
        source: stackable_operator::memory::Error,
    },

    #[snafu(display("invalid resource requirements"))]
    ValidationErrors {
        errors: Vec<ResourceRequestsValidationError>,
    },
}

#[derive(Debug, Snafu)]
pub enum ResourceRequestsValidationError {
    #[snafu(display(
        "The {object_name} requires {required_cpu} CPU core(s), but there are only {available_cpu} CPU core(s) available in the cluster",
        required_cpu = required.as_cpu_count(), available_cpu = available.as_cpu_count()
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
    pub async fn validate_cluster_size(&self, client: &Client, object_name: &str) -> Result<()> {
        let cluster_info = client.get_cluster_info().await.context(ClusterInfoSnafu)?;

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
            return Err(ResourceRequestsError::ValidationErrors { errors });
        }

        Ok(())
    }
}
