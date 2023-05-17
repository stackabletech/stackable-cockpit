use kube::{
    api::{Patch, PatchParams},
    core::{DynamicObject, GroupVersionKind, TypeMeta},
    discovery::Scope,
    Api, Client, Discovery, ResourceExt,
};
use serde::Deserialize;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum KubeError {
    #[snafu(display("kube error: {source}"))]
    KubeError { source: kube::error::Error },

    #[snafu(display("yaml error: {source}"))]
    YamlError { source: serde_yaml::Error },

    #[snafu(display("failed to deploy manifest because type of object {object:?} is not set"))]
    ObjectTypeError { object: DynamicObject },

    #[snafu(display("failed to deploy manifest because GVK {gvk:?} cannot be resolved"))]
    DiscoveryError { gvk: GroupVersionKind },
}

pub async fn deploy_manifests(manifests: &str, namespace: &str) -> Result<(), KubeError> {
    let client = Client::try_default().await.context(KubeSnafu {})?;
    let discovery = Discovery::new(client.clone())
        .run()
        .await
        .context(KubeSnafu {})?;

    for manifest in serde_yaml::Deserializer::from_str(manifests) {
        let mut object = DynamicObject::deserialize(manifest).context(YamlSnafu {})?;
        let object_type = object.types.as_ref().ok_or(
            ObjectTypeSnafu {
                object: object.clone(),
            }
            .build(),
        )?;

        let gvk = gvk_of_typemeta(object_type);
        let (resource, capabilities) = discovery
            .resolve_gvk(&gvk)
            .ok_or(DiscoverySnafu { gvk }.build())?;

        let api: Api<DynamicObject> = match capabilities.scope {
            Scope::Cluster => {
                object.metadata.namespace = None;
                Api::all_with(client.clone(), &resource)
            }
            Scope::Namespaced => Api::namespaced_with(client.clone(), namespace, &resource),
        };

        api.patch(
            &object.name_any(),
            &PatchParams::apply("stackablectl"),
            &Patch::Apply(object),
        )
        .await
        .context(KubeSnafu {})?;
    }

    Ok(())
}

fn gvk_of_typemeta(type_meta: &TypeMeta) -> GroupVersionKind {
    match type_meta.api_version.split_once('/') {
        Some((group, version)) => GroupVersionKind::gvk(group, version, &type_meta.kind),
        None => GroupVersionKind::gvk("", &type_meta.api_version, &type_meta.kind),
    }
}
