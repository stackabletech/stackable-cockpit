use std::marker::PhantomData;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

use crate::{
    utils::path::PathOrUrl,
    xfer::{processor::Yaml, FileTransferClient, FileTransferError},
};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to transfer the list file"))]
    FileTransfer { source: FileTransferError },
}

pub trait SpecIter<S> {
    fn inner(&self) -> &IndexMap<String, S>;
}

/// A [`List`] describes a list of specs. The list can contain any specs, for
/// example demos, stacks or releases. The generic parameter `L` represents
/// the initial type of the spec list, directly deserialized from YAML. This
/// type has to implement [`SpecIter`], which returns a map of specs of type
/// `S`.
#[derive(Debug, Serialize)]
pub struct List<L, S>
where
    L: for<'a> Deserialize<'a> + Serialize + SpecIter<S>,
    S: for<'a> Deserialize<'a> + Serialize + Clone,
{
    inner: IndexMap<String, S>,
    list_type: PhantomData<L>,
}

impl<L, S> List<L, S>
where
    L: for<'a> Deserialize<'a> + Serialize + SpecIter<S>,
    S: for<'a> Deserialize<'a> + Serialize + Clone,
{
    /// Builds a list of specs of type `S` based on a list of files. These files
    /// can be located locally (on disk) or remotely. Remote files will get
    /// downloaded.
    pub async fn build(files: &[PathOrUrl], transfer_client: &FileTransferClient) -> Result<Self> {
        let mut map = IndexMap::new();

        for file in files {
            let specs = transfer_client
                .get(file, &Yaml::<L>::new())
                .await
                .context(FileTransferSnafu)?;

            for (spec_name, spec) in specs.inner() {
                map.insert(spec_name.clone(), spec.clone());
            }
        }

        Ok(Self {
            list_type: PhantomData,
            inner: map,
        })
    }

    /// Returns a reference to the inner [`IndexMap`]
    pub fn inner(&self) -> &IndexMap<String, S> {
        &self.inner
    }

    /// Returns an optional reference to a single spec of type `S` by `name`
    pub fn get<T>(&self, name: T) -> Option<&S>
    where
        T: AsRef<str>,
    {
        self.inner.get(name.as_ref())
    }
}
