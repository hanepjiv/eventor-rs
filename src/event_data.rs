/* -*- mode:rust; coding:utf-8-unix; -*- */

//! event_data.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2016/08/18

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::fmt::{ Debug, };
use ::std::any::{ Any, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
aelicit_define!(aelicit_t_event_data, TEventData);
/* -------------------------------------------------------------------------- */
pub use self::aelicit_t_event_data::AelicitError
    as EventDataAelicitError;
pub use self::aelicit_t_event_data::AelicitResult
    as EventDataAelicitResult;
pub use self::aelicit_t_event_data::Aelicit
    as EventDataAelicit;
pub use self::aelicit_t_event_data::EnableAelicitFromSelf
    as EventDataEnableAelicitFromSelf;
pub use self::aelicit_t_event_data::EnableAelicitFromSelfField
    as EventDataEnableAelicitFromSelfField;
/* ========================================================================== */
/// trait TEventData
pub trait TEventData: Debug +
    EventDataEnableAelicitFromSelf + AsRef< Any > + AsMut< Any > {
}
/* -------------------------------------------------------------------------- */
impl < T > TEventData for T
    where T: Debug +
    EventDataEnableAelicitFromSelf + AsRef< Any > + AsMut< Any > {
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventData
#[derive( Debug, )]
pub struct EventData< T: Debug + Any > {
    /// EventDataEnableAelicitFromSelfField
    _eefsf:     EventDataEnableAelicitFromSelfField,
    /// data
    data:       T,
}
/* ========================================================================== */
impl < T: Debug + Any > EventDataEnableAelicitFromSelf for EventData< T > {
    enable_aelicit_from_self_impl_inner!(TEventData, EventDataAelicit, _eefsf);
}
/* ========================================================================== */
impl < T: Debug + Any > EventData< T > {
    /* ====================================================================== */
    /// new
    pub fn new(data: T) -> Self { EventData {
        _eefsf: EventDataEnableAelicitFromSelfField::default(),
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
