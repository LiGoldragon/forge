//! FileMaterialiser — projects store entries into a workdir.
//!
//! When prism emits source files for the runtime-creation
//! pipeline to compile, this actor materialises the source tree
//! from sema records (resolved through criome) into actual files
//! at a temp path.

use crate::Result;

pub struct FileMaterialiser;

impl FileMaterialiser {
    pub async fn materialise(_workdir: &std::path::Path) -> Result<()> {
        todo!("walk records; write file tree; preserve permissions")
    }
}
