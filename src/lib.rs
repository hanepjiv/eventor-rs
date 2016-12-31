// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2016/12/31

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(fat_ptr_transmutes, missing_copy_implementations,
        missing_debug_implementations, missing_docs, unstable_features,
        unused_results, variant_size_differences)]
#![warn(unused_qualifications, unused_extern_crates, warnings)]
#![allow(box_pointers, trivial_casts, trivial_numeric_casts, unsafe_code,
         unused_import_braces)]
// extern  ====================================================================
#[macro_use] extern crate libc;
// ----------------------------------------------------------------------------
#[macro_use] extern crate hash_combine;
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
