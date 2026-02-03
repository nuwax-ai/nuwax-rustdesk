mod keyboard;
/// cbindgen:ignore
pub mod platform;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use platform::{
    clip_cursor, get_cursor, get_cursor_data, get_cursor_pos, get_focused_display,
    set_cursor_pos, start_os_service,
};
#[cfg(not(any(target_os = "ios")))]
/// cbindgen:ignore
mod server;
#[cfg(not(any(target_os = "ios")))]
pub use self::server::*;
mod client;
mod lan;
#[cfg(not(any(target_os = "ios")))]
mod rendezvous_mediator;
#[cfg(not(any(target_os = "ios")))]
pub use self::rendezvous_mediator::*;
/// cbindgen:ignore
pub mod common;
#[cfg(not(any(target_os = "ios")))]
pub mod ipc;
#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    feature = "cli",
    feature = "flutter"
)))]
pub mod ui;
mod version;
pub use version::*;
#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
mod bridge_generated;
#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
pub mod flutter;
#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
pub mod flutter_ffi;
use common::*;
mod auth_2fa;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(not(target_os = "ios"))]
mod clipboard;
#[cfg(not(any(target_os = "android", target_os = "ios", feature = "cli")))]
pub mod core_main;
mod custom_server;
mod lang;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod port_forward;

#[cfg(all(feature = "flutter", feature = "plugin_framework"))]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod plugin;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod tray;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod whiteboard;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod updater;

mod ui_cm_interface;
mod ui_interface;
mod ui_session_interface;

mod hbbs_http;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub mod clipboard_file;

pub mod privacy_mode;

#[cfg(windows)]
pub mod virtual_display_manager;

mod kcp_stream;

// agent-client feature: 暴露给 nuwax-agent 等嵌入方使用的 API
#[cfg(feature = "agent-client")]
pub use hbb_common;

#[cfg(all(feature = "agent-client", not(any(target_os = "android", target_os = "ios"))))]
pub mod client_api {
    pub use crate::client::{
        Client, Data, FileManager, Interface, Key, LoginConfigHandler,
        handle_hash, handle_login_error, handle_login_from_ui, handle_test_delay,
    };
    // 文件传输相关类型导出
    pub use hbb_common::fs::{
        TransferJob, JobType, DataSource,
        get_next_job_id, get_recursive_files, read_dir,
    };
    pub use hbb_common::message_proto::{
        FileEntry, FileType,
        FileTransferBlock, FileTransferSendRequest, FileTransferReceiveRequest,
        FileTransferCancel, FileTransferSendConfirmRequest, FileTransferDone,
        FileTransferDigest, FileTransferError, FileResponse, FileAction,
        FileDirectory, FileDirCreate, FileRemoveDir, FileRemoveFile,
        BusinessMessage,
    };
    pub use hbb_common::message_proto::message::Union as MessageUnion;
}
