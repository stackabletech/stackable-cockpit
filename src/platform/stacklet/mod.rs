use indexmap::IndexMap;
use kube::{
    core::{DynamicObject, GroupVersionKind},
    ResourceExt,
};
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use tracing::warn;

use crate::{
    constants::PRODUCT_NAMES,
    kube::{KubeClient, KubeError},
    utils::string::Casing,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    /// Name of the product.
    pub name: String,

    /// Some CRDs are cluster scoped.
    pub namespace: Option<String>,

    /// List of endpoints. The key describes the use of the endpoint like
    /// `web-ui`, `grpc` or `http`. The value is a URL at which the endpoint
    /// is accessible.
    pub endpoints: IndexMap<String, String>,

    // List of additional information about the product.
    pub additional_information: Vec<(String, String)>,
}

#[derive(Debug, Snafu)]
pub enum StackletError {
    #[snafu(display("kubernetes error"))]
    KubeError { source: KubeError },

    #[snafu(display("no namespace set for custom resource '{crd_name}'"))]
    CustomCrdNamespaceError { crd_name: String },
}

/// [`StackletListOptions`] describes available options when listing deployed
/// services.
pub struct StackletListOptions {
    /// Toggle wether to show credentials / secrets in the output. This defaults
    /// to `false` because of security reasons. Users need to explicitly tell
    /// the ctl or the web UI to show these credentials.
    pub show_credentials: bool,

    /// Toggle wether to show product versions in the output. This defaults to
    /// `true`.
    pub show_versions: bool,
}

impl Default for StackletListOptions {
    fn default() -> Self {
        Self {
            show_credentials: false,
            show_versions: true,
        }
    }
}

pub type StackletList = IndexMap<String, Vec<Product>>;

pub fn build_products_gvk_list(
    product_names: &[&'static str],
) -> IndexMap<&'static str, GroupVersionKind> {
    let mut map = IndexMap::new();

    for product_name in product_names {
        // Why? Just why? Can we please make this consistent?
        if *product_name == "spark-history-server" {
            map.insert(
                *product_name,
                GroupVersionKind {
                    group: "spark.stackable.tech".into(),
                    version: "v1alpha1".into(),
                    kind: "SparkHistoryServer".into(),
                },
            );
            continue;
        }

        map.insert(
            *product_name,
            GroupVersionKind {
                group: format!("{product_name}.stackable.tech"),
                version: "v1alpha1".into(),
                kind: format!("{}Cluster", product_name.capitalize()),
            },
        );
    }

    map
}

/// Lists all installed stacklets. If `namespace` is [`None`], stacklets from ALL
/// namespaces are returned. If `namespace` is [`Some`], only stacklets installed
/// in the specified namespace are returned. The `options` allow further
/// customization of the returned information.
pub async fn list_stacklets(
    namespace: Option<&str>,
    options: StackletListOptions,
) -> Result<StackletList, StackletError> {
    let kube_client = KubeClient::new().await.context(KubeSnafu {})?;
    let products = build_products_gvk_list(PRODUCT_NAMES);

    let mut stacklets = StackletList::new();

    for (product_name, product_gvk) in products {
        let objects = match kube_client
            .list_objects(&product_gvk, namespace)
            .await
            .context(KubeSnafu {})?
        {
            Some(obj) => obj,
            None => {
                warn!(
                    "Failed to list services because the gvk {product_gvk:?} can not be resolved"
                );
                continue;
            }
        };

        let mut products = Vec::new();

        for object in objects {
            let object_name = object.name_any();
            let object_namespace = match object.namespace() {
                Some(ns) => ns,
                // If the custom resource does not have a namespace set it can't expose a service
                None => continue,
            };

            let services = kube_client
                .list_services(&object_namespace, product_name, &object_name)
                .await
                .context(KubeSnafu {})?;

            let additional_information = get_additional_information(
                &kube_client,
                product_name,
                &object,
                options.show_credentials,
                options.show_versions,
            )
            .await?;

            let endpoints = IndexMap::new();
            for _service in services {}

            let product = Product {
                name: object_name,
                namespace: Some(object_namespace),
                endpoints,
                additional_information,
            };

            products.push(product);
        }

        stacklets.insert(product_name.to_string(), products);
    }

    Ok(stacklets)
}

async fn get_additional_information(
    client: &KubeClient,
    product_name: &str,
    object: &DynamicObject,
    show_credentials: bool,
    show_version: bool,
) -> Result<Vec<(String, String)>, StackletError> {
    let namespace = object.namespace().ok_or(
        CustomCrdNamespaceSnafu {
            crd_name: object.name_any(),
        }
        .build(),
    )?;

    let mut additional_information = Vec::new();

    match product_name {
        "airflow" | "superset" => {
            if let Some(secret_name) = object.data["spec"]["credentialsSecret"].as_str() {
                let credentials = client
                    .get_credentials_from_secret(
                        secret_name,
                        &namespace,
                        "adminUser.username",
                        show_credentials.then_some("adminUser.password"),
                    )
                    .await
                    .context(KubeSnafu {})?;

                if let Some((username, password)) = credentials {
                    additional_information.push(("USERNAME".into(), username));
                    additional_information.push(("PASSWORD".into(), password));
                }
            }
        }
        "nifi" => {
            if let Some(secret_name) = object.data["spec"]["config"]["authentication"]["method"]
                ["singleUser"]["adminCredentialsSecret"]
                .as_str()
            {
                let credentials = client
                    .get_credentials_from_secret(
                        secret_name,
                        &namespace,
                        "username",
                        show_credentials.then_some("password"),
                    )
                    .await
                    .context(KubeSnafu {})?;

                if let Some((username, password)) = credentials {
                    additional_information.push(("USERNAME".into(), username));
                    additional_information.push(("PASSWORD".into(), password));
                }
            }
        }
        _ => (),
    }

    if show_version {
        if let Some(version) = object.data["spec"]["version"].as_str() {
            additional_information.push(("VERSION".into(), version.into()))
        }
    }

    Ok(additional_information)
}
