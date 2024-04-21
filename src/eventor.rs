// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/04/21

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::sync::RwLock;
// ----------------------------------------------------------------------------
use log::info;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    error::Error,
    event::{Event, EventQueue},
    event_listener::{
        aelicit_user::Aelicit as EventListenerAelicit, EventListenerMap,
        EventListenerRemoving, EventListenerWaiting, RetOnEvent,
    },
    event_type::{EventType, EventTypeMap},
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Eventor
#[derive(Debug)]
pub struct Eventor {
    /// event type map
    type_map: RwLock<EventTypeMap>,
    /// event queue
    queue: RwLock<EventQueue>,
    /// event listener map
    listener_map: RwLock<EventListenerMap>,
    /// event listener waiting
    listener_waiting: EventListenerWaiting,
    /// event listener removing
    listener_removing: EventListenerRemoving,
}
// ============================================================================
impl Default for Eventor {
    fn default() -> Self {
        Eventor {
            type_map: RwLock::new(EventTypeMap::default()),
            queue: RwLock::new(EventQueue::default()),
            listener_map: RwLock::new(EventListenerMap::default()),
            listener_waiting: EventListenerWaiting::default(),
            listener_removing: EventListenerRemoving::default(),
        }
    }
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
        listener: &EventListenerAelicit,
    ) {
        self.listener_waiting.insert(event_hash, listener.clone())
    }
    // ------------------------------------------------------------------------
    /// remove_listener
    pub fn remove_listener(&self, event_hash: u32, id: &Uuid) {
        self.listener_removing.insert(event_hash, id)
    }
    // ========================================================================
    /// fn push_event
    pub fn push_event(&self, event: Event) {
        self.queue.write().expect("Eventor::push_event").push(event)
    }
    // ------------------------------------------------------------------------
    /// fn dispatch
    #[allow(box_pointers)]
    pub fn dispatch(&self) -> bool {
        self.listener_waiting
            .apply(&mut self.listener_map.write().expect("Eventor::dispatch"));

        self.listener_removing
            .apply(&mut self.listener_map.write().expect("Eventor::dispatch"));

        let event = self.queue.write().expect("Eventor::dispatch").pop();

        if let Some(e) = event {
            if let Some(list) = self
                .listener_map
                .write()
                .expect("Eventor::dispatch")
                .get_mut(&(e.peek_type().peek_hash()))
            {
                for (_, ref mut listener) in list.iter_mut() {
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
                .write()
                .expect("Eventor::dispatch")
                .shrink_to_fit();
            self.listener_waiting.shrink_to_fit();
            self.listener_removing.shrink_to_fit();
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
