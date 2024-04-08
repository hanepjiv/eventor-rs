// -*- mode:rust; coding:utf-8-unix; -*-

//! event_data.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/04/08

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{any::Any, fmt::Debug};
// ----------------------------------------------------------------------------
use elicit::{aelicit_define, enable_aelicit_from_self_delegate};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
aelicit_define!(aelicit_t_event_data, TEventData);
// ----------------------------------------------------------------------------
pub use self::aelicit_t_event_data::{
    Aelicit as EventDataAelicit, EnableAelicitFromSelf as EventDataEAFS,
    EnableAelicitFromSelfField as EventDataEAFSField,
};
// ============================================================================
/// trait TEventData
pub trait TEventData:
    Debug + EventDataEAFS + AsRef<dyn Any> + AsMut<dyn Any>
{
}
// ----------------------------------------------------------------------------
impl<T> TEventData for T where
    T: Debug + EventDataEAFS + AsRef<dyn Any> + AsMut<dyn Any>
{
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventData
#[derive(Debug)]
pub struct EventData<T: Debug + Any + Send + Sync> {
    /// EventDataEAFSField
    _eefsf: EventDataEAFSField,
    /// data
    data: T,
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> EventDataEAFS for EventData<T> {
    enable_aelicit_from_self_delegate!(TEventData, EventDataAelicit, _eefsf);
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> EventData<T> {
    // ========================================================================
    /// new
    pub fn new(data: T) -> Self {
        Self {
            _eefsf: EventDataEAFSField::default(),
            data,
        }
    }
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> AsRef<dyn Any> for EventData<T> {
    fn as_ref(&self) -> &dyn Any {
        &self.data
    }
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> AsMut<dyn Any> for EventData<T> {
    fn as_mut(&mut self) -> &mut dyn Any {
        &mut self.data
    }
}
