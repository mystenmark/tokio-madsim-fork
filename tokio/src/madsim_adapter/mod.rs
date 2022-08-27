/// JoinError wrapper
pub mod join_error {

    use crate::runtime::task::Id;
    use crate::task::JoinError;
    use std::any::Any;

    /// wrapper for JoinError::cancelled
    pub fn cancelled(id: Id) -> JoinError {
        JoinError::cancelled(id)
    }

    /// wrapper for JoinError::panic
    pub fn panic(id: Id, err: Box<dyn Any + Send + 'static>) -> JoinError {
        JoinError::panic(id, err)
    }
}

/// runtime::task wrapper
pub mod runtime_task {
    pub use crate::runtime::task::*;

    /// wrapper for task::Id::next()
    pub fn next_task_id() -> Id {
        Id::next()
    }
}

/// expose ability to reset internal rng
pub mod util {
    use crate::util::reset_rng as reset;

    /// reset the rng in src/util/rand.rs
    pub fn reset_rng(seed: u64) {
        reset(seed);
    }
}
