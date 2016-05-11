/* -*- mode:rust; coding:utf-8-unix; -*- */

//! event_data.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2016/05/11

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::fmt::{ Debug, };
use ::std::any::{ Any, };
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
pub trait TEventData: Debug +
    EventDataEnableElicitFromSelf + AsRef< Any > + AsMut< Any > {
}
/* -------------------------------------------------------------------------- */
impl < T > TEventData for T
    where T: Debug +
    EventDataEnableElicitFromSelf + AsRef< Any > + AsMut< Any > {
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
        _eefsf: EventDataEnableElicitFromSelfField::default(),
        data:   data,
    } }
}
/* ========================================================================== */
impl < T: Debug + Any > AsRef< Any > for EventData< T > {
    fn as_ref(&self) -> &Any { &self.data }
}
/* ========================================================================== */
impl < T: Debug + Any > AsMut< Any > for EventData< T > {
    fn as_mut(&mut self) -> &mut Any { &mut self.data }
}
