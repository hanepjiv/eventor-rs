// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2018/05/13

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.26.0 (a77568041 2018-05-07)
#![deny(
    anonymous_parameters, missing_copy_implementations,
    missing_debug_implementations, missing_docs, unstable_features,
    unused_extern_crates, unused_import_braces, unused_qualifications,
    unused_results, variant_size_differences, const_err, deprecated,
    illegal_floating_point_literal_pattern, improper_ctypes,
    incoherent_fundamental_impls, late_bound_lifetime_arguments,
    non_camel_case_types, non_shorthand_field_patterns, non_snake_case,
    non_upper_case_globals, no_mangle_generic_items, overflowing_literals,
    path_statements, patterns_in_fns_without_body, plugin_as_library,
    private_in_public, private_no_mangle_fns, private_no_mangle_statics,
    renamed_and_removed_lints, safe_packed_borrows, stable_features,
    type_alias_bounds, tyvar_behind_raw_pointer, unconditional_recursion,
    unions_with_drop_fields, unknown_lints, unreachable_code,
    unreachable_patterns, unstable_name_collision, unused_allocation,
    unused_assignments, unused_attributes, unused_comparisons,
    unused_doc_comment, unused_features, unused_imports, unused_macros,
    unused_must_use, unused_mut, unused_parens, unused_unsafe,
    unused_variables, while_true, exceeding_bitshifts,
    invalid_type_param_default, legacy_constructor_visibility,
    legacy_directory_ownership, legacy_imports, missing_fragment_specifier,
    mutable_transmutes, no_mangle_const_items,
    parenthesized_params_in_types_and_modules, pub_use_of_private_extern_crate,
    safe_extern_statics, unknown_crate_types
)]
#![warn(bare_trait_object, dead_code, unreachable_pub)]
#![allow(
    box_pointers, elided_lifetime_in_path, unsafe_code, trivial_casts,
    single_use_lifetime, trivial_numeric_casts
)]
// extern  ====================================================================
extern crate libc;
#[macro_use]
extern crate log;
// ----------------------------------------------------------------------------
#[macro_use]
extern crate elicit;
extern crate hash_combine;
// use  =======================================================================
pub use self::{
    error::{Error, Result}, event::Event,
    event_data::{
        EventData, EventDataAelicit, EventDataEAFS, EventDataEAFSField,
    },
    event_listener::{
        EventListenerAelicit, EventListenerEAFS, EventListenerEAFSField,
        RetOnEvent, TEventListener,
    },
    event_type::{EventType, EventTypeRef},
    eventor::{
        Eventor, EventorAelicit, EventorEAFS, EventorEAFSField, TEventor,
    },
};
// mod  =======================================================================
mod error;
mod event;
mod event_data;
mod event_listener;
mod event_type;
mod eventor;
