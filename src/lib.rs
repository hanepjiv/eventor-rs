// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2017/02/23

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(
    fat_ptr_transmutes,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    const_err,
    deprecated,
    deprecated_attr,
    extra_requirement_in_impl,
    improper_ctypes,
    legacy_imports,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    renamed_and_removed_lints,
    safe_extern_statics,
    stable_features,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unreachable_code,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    while_true,
    exceeding_bitshifts,
    hr_lifetime_in_assoc_type,
    illegal_floating_point_constant_pattern,
    illegal_struct_or_enum_constant_pattern,
    inaccessible_extern_crate,
    invalid_type_param_default,
    lifetime_underscore,
    mutable_transmutes,
    no_mangle_const_items,
    overlapping_inherent_impls,
    super_or_self_in_global_path,
    transmute_from_fn_item_types,
    unknown_crate_types,
)]
#![warn(
    dead_code,
)]
#![allow(
    box_pointers,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
)]
// extern  ====================================================================
extern crate libc;
#[macro_use] extern crate log;
// ----------------------------------------------------------------------------
extern crate hash_combine;
#[macro_use] extern crate elicit;
// use  =======================================================================
pub use self::error::{ Result, Error, };
pub use self::event_type::{ EventType, EventTypeRef, };
pub use self::event_data::{ EventDataAelicit,
                            EventDataEnableAelicitFromSelf,
                            EventDataEnableAelicitFromSelfField,
                            EventData, };
pub use self::event::Event;
pub use self::event_listener::{ TEventListener,
                                EventListenerAelicit,
                                EventListenerEnableAelicitFromSelf,
                                EventListenerEnableAelicitFromSelfField, };
pub use self::eventor::{ TEventor,
                         EventorAelicit,
                         EventorEnableAelicitFromSelf,
                         EventorEnableAelicitFromSelfField,
                         Eventor, };
// mod  =======================================================================
mod error;
mod event_type;
mod event_data;
mod event;
mod event_listener;
mod eventor;
