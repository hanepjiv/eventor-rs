// -*- mode:rust; coding:utf-8-unix; -*-

//! event_data.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2017/05/26

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::fmt::Debug;
use std::any::Any;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
aelicit_define!(aelicit_t_event_data, TEventData);
// ----------------------------------------------------------------------------
pub use self::aelicit_t_event_data::Aelicit as EventDataAelicit;
pub use self::aelicit_t_event_data::EnableAelicitFromSelf as EventDataEAFS;
pub use self::aelicit_t_event_data::EnableAelicitFromSelfField;
use self::EnableAelicitFromSelfField as EventDataEAFSField;
// ============================================================================
/// trait TEventData
pub trait TEventData: Debug + EventDataEAFS + AsRef<Any> + AsMut<Any> {}
// ----------------------------------------------------------------------------
impl<T> TEventData for T
where
    T: Debug + EventDataEAFS + AsRef<Any> + AsMut<Any>,
{
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventData
#[derive(Debug)]
pub struct EventData<T: Debug + Any> {
    /// EventDataEAFSField
    _eefsf: EventDataEAFSField,
    /// data
    data: T,
}
// ============================================================================
impl<T: Debug + Any> EventDataEAFS for EventData<T> {
    enable_aelicit_from_self_delegate!(TEventData, EventDataAelicit, _eefsf);
}
// ============================================================================
impl<T: Debug + Any> EventData<T> {
    // ========================================================================
    /// new
    pub fn new(data: T) -> Self {
        Self {
            _eefsf: EventDataEAFSField::default(),
            data: data,
        }
    }
}
// ============================================================================
impl<T: Debug + Any> AsRef<Any> for EventData<T> {
    fn as_ref(&self) -> &Any {
        &self.data
    }
}
// ============================================================================
impl<T: Debug + Any> AsMut<Any> for EventData<T> {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.data
    }
}
