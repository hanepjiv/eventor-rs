/* -*- mode:rust; coding:utf-8; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/07
// @date 2016/03/27

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

//! event_data.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::fmt::{ Debug, };
use ::std::any::{ Any, };
use ::std::borrow::{ Borrow, BorrowMut, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
elicit_define!(elicit_t_event_data, TEventData);
/* -------------------------------------------------------------------------- */
pub use self::elicit_t_event_data::ElicitError
    as EventDataElicitError;
pub use self::elicit_t_event_data::ElicitResult
    as EventDataElicitResult;
pub use self::elicit_t_event_data::Elicit
    as EventDataElicit;
pub use self::elicit_t_event_data::EnableElicitFromSelf
    as EventDataEnableElicitFromSelf;
pub use self::elicit_t_event_data::EnableElicitFromSelfField
    as EventDataEnableElicitFromSelfField;
/* ========================================================================== */
/// trait TEventData
pub trait TEventData: Debug + EventDataEnableElicitFromSelf + BorrowMut< Any > {
}
/* -------------------------------------------------------------------------- */
impl < T > TEventData for T
    where T: Debug + EventDataEnableElicitFromSelf + BorrowMut< Any > {
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventData
#[derive( Debug, )]
    pub struct EventData< T: Debug + Any > {
    /// EventDataEnableElicitFromSelfField
    _eefsf:     EventDataEnableElicitFromSelfField,
    /// data
    data:       T,
}
/* ========================================================================== */
impl < T: Debug + Any > EventDataEnableElicitFromSelf for EventData< T > {
    enable_elicit_from_self_impl_inner!(TEventData, EventDataElicit, _eefsf);
}
/* ========================================================================== */
impl < T: Debug + Any > EventData< T > {
    /* ====================================================================== */
    /// new
    pub fn new(data: T) -> Self { EventData {
        _eefsf: EventDataEnableElicitFromSelfField::new(),
        data:   data,
    } }
}
/* ========================================================================== */
impl < T: Debug + Any > Borrow< Any > for EventData< T > {
    #[inline(always)]
    fn borrow(&self) -> &Any { &self.data }
}
/* ========================================================================== */
impl < T: Debug + Any > BorrowMut< Any > for EventData< T > {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut Any { &mut self.data }
}
