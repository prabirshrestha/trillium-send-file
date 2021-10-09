#![forbid(unsafe_code)]
#![deny(
    clippy::dbg_macro,
    missing_copy_implementations,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    unused_qualifications
)]

/*!
Send a static file from the file system.
*/

use std::path::PathBuf;
use trillium::{conn_try, Conn};
use trillium::{Body, KnownHeaderName};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "smol")] {
        use async_fs::{self as fs, File};
    } else if #[cfg(feature = "async-std")] {
        use async_std_crate::fs::{self, File};
    } else {
        compile_error!("trillium-send-file:
You must enable one of the two runtime feature flags
to use this crate:
* async-std
* smol
This is a temporary constraint, and hopefully soon this
will not require the use of cargo feature flags."
);
    }
}

cfg_if! { if #[cfg(any(feature= "smol", feature = "async-std"))] {
    /**
     Extension trait adding `send_file` capabilities to [`Conn`].
     */
    #[trillium::async_trait]
    pub trait SendFileConnExt : Sync {
        /// sets the streaming body of file.
        async fn send_file(mut self, path: PathBuf) -> Conn;
    }

    #[trillium::async_trait]
    impl SendFileConnExt for Conn {
        async fn send_file(mut self, path: PathBuf) -> Conn {
            let file = conn_try!(File::open(path.clone()).await, self);
            let metadata = conn_try!(fs::metadata(path.clone()).await, self);
            let len = metadata.len();
            self.with_header(
                KnownHeaderName::ContentType,
                mime_guess::from_path(path.clone())
                    .first_or_octet_stream()
                    .to_string(),
            )
            .with_body(Body::new_streaming(file, Some(len)))
            .with_status(200)
        }
    }
}}
