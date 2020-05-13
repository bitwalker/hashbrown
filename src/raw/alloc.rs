pub use self::inner::*;

#[cfg(feature = "nightly")]
mod inner {
    pub use crate::alloc::alloc::{AllocRef, AllocInit, Global};
}

#[cfg(not(feature = "nightly"))]
mod inner {
    compile_error!("this branch requires a nightly compiler");
}
