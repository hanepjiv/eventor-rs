// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/04/22

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::sync::{Mutex, RwLock};
// ----------------------------------------------------------------------------
use log::info;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    error::Error,
    event::{Event, EventQueue},
    event_listener::{
        aelicit_user::Aelicit as EventListenerAelicit, EventListenerMap,
        RetOnEvent,
    },
    event_type::{EventType, EventTypeMap},
};
use crate::inner::Mediator;
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
    listener_map: Mutex<EventListenerMap>,
    /// mediator
    mediator: Mediator,
}

// ============================================================================
impl Eventor {
    // ========================================================================
    /// fn new
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
        self.type_map
            .write()
            .expect("Eventor::new_type")
            .new_type(name.as_ref())
    }
    // ------------------------------------------------------------------------
    /// fn peek_typs
    pub fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        self.type_map
            .read()
            .expect("Eventor::peek_type")
            .peek_type(name.as_ref())
    }
    // ========================================================================
    /// fn insert_listener
    pub fn insert_listener(
        &self,
        event_hash: u32,
        id: Uuid,
        listener: EventListenerAelicit,
    ) {
        self.mediator.insert(event_hash, id, listener)
    }
    // ------------------------------------------------------------------------
    /// remove_listener
    pub fn remove_listener(&self, event_hash: u32, id: &Uuid) {
        self.mediator.remove(event_hash, id)
    }
    // ========================================================================
    /// fn push_event
    pub fn push_event(&self, event: Event) {
        self.queue.lock().expect("Eventor::push_event").push(event)
    }
    // ------------------------------------------------------------------------
    /// fn dispatch
    #[allow(box_pointers)]
    pub fn dispatch(&self) -> bool {
        self.mediator
            .apply(&mut self.listener_map.lock().expect("Eventor::dispatch"));

        let event = self.queue.lock().expect("Eventor::dispatch").pop();

        if let Some(e) = event {
            if let Some(list) = self
                .listener_map
                .lock()
                .expect("Eventor::dispatch")
                .get_mut(&(e.peek_type().peek_hash()))
            {
                for (_, listener) in list.iter_mut() {
                    if let RetOnEvent::Complete = listener
                        .write()
                        .expect("Eventor::dispatch")
                        .on_event(&e, self)
                    {
                        break;
                    }
                }
            } else if cfg!(debug_assertions) {
                info!("Eventor::dispatch: no listener: {:?}", e);
            }
            true
        } else {
            self.queue
                .lock()
                .expect("Eventor::dispatch")
                .shrink_to_fit();
            false
        }
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
