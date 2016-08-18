/* -*- mode:rust; coding:utf-8-unix; -*- */

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2016/08/18

/* ////////////////////////////////////////////////////////////////////////// */
/* attribute  =============================================================== */
#![deny(missing_docs, dead_code, unused_imports, unused_variables)]
/* ========================================================================== */
#[macro_use] extern crate libc;
#[macro_use] extern crate log;
/* -------------------------------------------------------------------------- */
#[macro_use] extern crate hash_combine;
#[macro_use] extern crate elicit;
/* ========================================================================== */
mod event_type;
mod event_data;
mod event;
mod event_listener;
mod eventor;
/* ========================================================================== */
pub use self::event_type::{ EventType, EventTypeRef, };
pub use self::event_data::{ EventDataAelicitError,
                            EventDataAelicitResult,
                            EventDataAelicit,
                            EventDataEnableAelicitFromSelf,
                            EventDataEnableAelicitFromSelfField,
                            EventData, };
pub use self::event::{ Event, };
pub use self::event_listener::{ TEventListener,
                                EventListenerAelicitResult,
                                EventListenerAelicit,
                                EventListenerEnableAelicitFromSelf,
                                EventListenerEnableAelicitFromSelfField, };
pub use self::eventor::{ TEventor,
                         EventorAelicitError,
                         EventorAelicitResult,
                         EventorAelicit,
                         EventorEnableAelicitFromSelf,
                         EventorEnableAelicitFromSelfField,
                         Eventor, };
