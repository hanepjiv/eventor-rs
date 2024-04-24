// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/04/24

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use log::info;
use parking_lot::{Mutex, RwLock};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    error::Error,
    event::{Event, EventQueue},
    event_listener::{
        aelicit_user::Aelicit as EventListenerAelicit, RetOnEvent,
    },
    event_type::{EventType, EventTypeMap},
};
use crate::inner::{ListenerMap, Mediator};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Eventor
#[derive(Debug, Default)]
pub struct Eventor {
    /// event type map
    type_map: RwLock<EventTypeMap>,
    /// event queue
    queue: Mutex<EventQueue>,
    /// event listener map
    listener_map: RwLock<ListenerMap>,
    /// mediator
    mediator: Mediator,
}

// ============================================================================
impl Eventor {
    // ========================================================================
    /// new
    pub fn new() -> Self {
        Self::default()
    }
    // ========================================================================
    // ------------------------------------------------------------------------
    /// new_type
    pub fn new_type<T>(&self, name: T) -> Result<EventType, Error>
    where
        T: AsRef<str>,
    {
        self.type_map.write().new_type(name.as_ref())
    }
    // ------------------------------------------------------------------------
    /// peek_typs
    pub fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        self.type_map.read().peek_type(name.as_ref())
    }
    // ========================================================================
    /// insert_listener
    pub fn insert_listener(&self, hash: u32, listener: EventListenerAelicit) {
        self.mediator.insert(hash, listener)
    }
    // ------------------------------------------------------------------------
    /// remove_listener
    pub fn remove_listener(&self, hash: u32, id: &Uuid) {
        self.mediator.remove(hash, id)
    }
    // ========================================================================
    /// push_event
    pub fn push_event(&self, event: Event) {
        self.queue.lock().push(event)
    }
    // ------------------------------------------------------------------------
    /// dispatch
    ///
    /// return: bool
    ///         true    = dispatch event
    ///         false   = no event
    ///
    #[allow(box_pointers)]
    pub fn dispatch(&self) -> bool {
        self.mediator.apply(self.listener_map.write());

        let Some(eve) = self.queue.lock().pop() else {
            self.queue.lock().shrink_to_fit();
            return false;
        };

        let Some(map) = self.listener_map.try_read() else {
            self.queue.lock().push_front(eve);
            return true;
        };

        let Some(list) = map.get(&(eve.peek_type().peek_hash())) else {
            if cfg!(debug_assertions) {
                info!("Eventor::dispatch: no listener: {eve:?}");
            }
            return true;
        };

        if list.is_empty() {
            if cfg!(debug_assertions) {
                info!("Eventor::dispatch: no listener: {eve:?}");
            }
            return true;
        }

        for (_, listener) in list.iter() {
            if let RetOnEvent::Complete = listener
                .read()
                .expect("Eventor::dispatch: listener")
                .on_event(&eve, self)
            {
                break;
            }
        }

        true
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::Eventor;
    // ========================================================================
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Eventor>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Eventor>();
    }
}
