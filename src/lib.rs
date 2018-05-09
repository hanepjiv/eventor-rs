// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2018/05/09

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(box_pointers, missing_copy_implementations,
        missing_debug_implementations, missing_docs, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features,
        unused_extern_crates, unused_import_braces, unused_qualifications,
        unused_results, variant_size_differences, const_err, dead_code,
        deprecated, illegal_floating_point_literal_pattern, improper_ctypes,
        late_bound_lifetime_arguments, non_camel_case_types,
        non_shorthand_field_patterns, non_snake_case, non_upper_case_globals,
        no_mangle_generic_items, overflowing_literals, path_statements,
        patterns_in_fns_without_body, plugin_as_library, private_in_public,
        private_no_mangle_fns, private_no_mangle_statics,
        renamed_and_removed_lints, stable_features, unconditional_recursion,
        unions_with_drop_fields, unknown_lints, unreachable_code,
        unreachable_patterns, unused_allocation, unused_assignments,
        unused_attributes, unused_comparisons, unused_doc_comment,
        unused_features, unused_imports, unused_macros, unused_must_use,
        unused_mut, unused_parens, unused_unsafe, unused_variables,
        while_true)]
#![warn(anonymous_parameters, dead_code)]
#![allow(box_pointers, unsafe_code, trivial_casts, trivial_numeric_casts)]
// extern  ====================================================================
extern crate libc;
#[macro_use]
extern crate log;
// ----------------------------------------------------------------------------
#[macro_use]
extern crate elicit;
extern crate hash_combine;
// use  =======================================================================
pub use self::error::{Error, Result};
pub use self::event::Event;
pub use self::event_data::{EventData, EventDataAelicit, EventDataEAFS,
                           EventDataEAFSField};
pub use self::event_listener::{EventListenerAelicit, EventListenerEAFS,
                               EventListenerEAFSField, RetOnEvent,
                               TEventListener};
pub use self::event_type::{EventType, EventTypeRef};
pub use self::eventor::{Eventor, EventorAelicit, EventorEAFS,
                        EventorEAFSField, TEventor};
// mod  =======================================================================
mod error;
mod event;
mod event_data;
mod event_listener;
mod event_type;
mod eventor;
