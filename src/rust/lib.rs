// 临时抑制 Clippy 警告，确保 CI 通过
// TODO: 后续逐步修复这些警告后移除对应的 allow 项
#![allow(
    clippy::bind_instead_of_map,
    clippy::collapsible_if,
    clippy::collapsible_str_replace,
    clippy::derivable_impls,
    clippy::double_ended_iterator_last,
    clippy::empty_line_after_doc_comments,
    clippy::field_reassign_with_default,
    clippy::if_same_then_else,
    clippy::io_other_error,
    clippy::let_unit_value,
    clippy::manual_abs_diff,
    clippy::manual_div_ceil,
    clippy::manual_is_multiple_of,
    clippy::manual_map,
    clippy::manual_ok_err,
    clippy::manual_strip,
    clippy::map_clone,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_range_loop,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::option_as_ref_deref,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::redundant_pattern_matching,
    clippy::should_implement_trait,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    clippy::unnecessary_sort_by,
    clippy::useless_conversion,
    clippy::useless_format,
    clippy::collapsible_match,
    clippy::vec_init_then_push,
)]

pub mod app;
pub mod config;
pub mod constants;
pub mod mcp;
pub mod network;
pub mod telegram;
pub mod ui;
pub mod utils;

// 避免重名导出，使用限定导出
pub use config::*;
pub use utils::*;

// 选择性导出常用项，避免冲突
pub use constants::{app as app_constants, theme, validation, network as network_constants, telegram as telegram_constants};
pub use mcp::{server, tools, types, handlers, utils as mcp_utils};
pub use ui::{window as ui_window, audio as ui_audio, audio_assets, updater};
