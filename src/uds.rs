//! UDS listener — accepts signal requests from criome.
//!
//! Skeleton-as-design. Reads rkyv-archived signal frames from
//! a Unix-domain socket; dispatches to the appropriate actor;
//! writes rkyv-archived reply frames back.

use crate::Result;

pub struct UdsListener;

impl UdsListener {
    pub async fn bind(_socket_path: &std::path::Path) -> Result<Self> {
        todo!("bind UDS socket; SO_PEERCRED check; return listener")
    }

    pub async fn run(self) -> Result<()> {
        todo!("accept loop; spawn per-connection task; route to actors")
    }
}
