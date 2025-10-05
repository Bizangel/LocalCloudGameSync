use crate::config::RuntimeSyncConfig;

/// Implementors must implement their own Drop.
pub trait RemoteLock<'c> {
    fn acquire(config: &'c RuntimeSyncConfig) -> Result<Self, String>
    where
        Self: Sized;

    fn is_acquired(&self) -> bool;
}
