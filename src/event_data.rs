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
aelicit_define!(aelicit_event_data_trait, EventDataTrait);
// ----------------------------------------------------------------------------
pub use self::aelicit_event_data_trait::Aelicit as EventDataWrapper;
use self::aelicit_event_data_trait::EnableAelicitFromSelf as EventDataEAFS;
// ============================================================================
/// trait EventDataTrait
pub(crate) trait EventDataTrait:
    Debug + AsRef<dyn Any> + AsMut<dyn Any> + EventDataEAFS
{
}
// ----------------------------------------------------------------------------
impl<T> EventDataTrait for T where
    T: Debug + AsRef<dyn Any> + AsMut<dyn Any> + EventDataEAFS
{
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventDataShell
#[derive(Debug)]
struct EventDataShell<T: Debug + Any + Send + Sync> {
    /// data
    data: T,
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> EventDataShell<T> {
    // ========================================================================
    /// new
    fn new(data: T) -> Self {
        Self { data }
    }
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> EventDataEAFS for EventDataShell<T> {
    enable_aelicit_from_self_delegate!(EventDataTrait, EventDataWrapper);
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> AsRef<dyn Any> for EventDataShell<T> {
    fn as_ref(&self) -> &dyn Any {
        &self.data
    }
}
// ============================================================================
impl<T: Debug + Any + Send + Sync> AsMut<dyn Any> for EventDataShell<T> {
    fn as_mut(&mut self) -> &mut dyn Any {
        &mut self.data
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// event_data_wrapper
pub fn event_data_wrapper<T>(data: T) -> EventDataWrapper
where
    T: Debug + Any + Send + Sync,
{
    EventDataWrapper::new(EventDataShell::new(data))
}
