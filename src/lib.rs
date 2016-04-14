/* -*- mode:rust; coding:utf-8; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/07
// @date 2016/04/05

// The MIT License (MIT)
//
// Copyright (c) <2016> hanepjiv <hanepjiv@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! lib.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* attribute  =============================================================== */
#![warn(missing_docs)]
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
pub use self::event_data::{ EventDataElicitError,
                            EventDataElicitResult,
                            EventDataElicit,
                            EventDataEnableElicitFromSelf,
                            EventDataEnableElicitFromSelfField,
                            EventData, };
pub use self::event::{ Event, };
pub use self::event_listener::{ TEventListener,
                                EventListenerElicitResult,
                                EventListenerElicit,
                                EventListenerEnableElicitFromSelf,
                                EventListenerEnableElicitFromSelfField, };
pub use self::eventor::{ TEventor,
                         EventorElicitError,
                         EventorElicitResult,
                         EventorElicit,
                         EventorEnableElicitFromSelf,
                         EventorEnableElicitFromSelfField,
                         Eventor, };
