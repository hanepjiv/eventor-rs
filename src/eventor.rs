// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/04/25

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
    event_type::EventType,
};
use crate::inner::{ListenerMap, Mediator, TypeMap};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Eventor
#[derive(Debug)]
pub struct Eventor {
    /// event type map
    type_map: RwLock<TypeMap>,
    /// event queue min capacity
    queue_capacity: usize,
    /// event queue
    queue: Mutex<EventQueue>,
    /// event listener map
    listener_map: RwLock<ListenerMap>,
    /// mediator
    mediator: Mediator,
}
// ============================================================================
impl Default for Eventor {
    fn default() -> Self {
        let queue_capacity = 64usize;
        Self {
            type_map: RwLock::<TypeMap>::default(),
            queue_capacity,
            queue: Mutex::new(EventQueue::with_capacity(queue_capacity)),
            listener_map: RwLock::<ListenerMap>::default(),
            mediator: Mediator::default(),
        }
    }
}
// ============================================================================
impl Eventor {
    // ========================================================================
    /// new
    pub fn new() -> Self {
        Self::default()
    }
    // ========================================================================
    /// with_capacity
    pub fn with_capacity(queue_capacity: usize) -> Self {
        Self {
            queue_capacity,
            queue: Mutex::new(EventQueue::with_capacity(queue_capacity)),
            ..Self::default()
        }
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
    ///
    /// # dispatch
    ///
    /// Process one event.
    ///
    /// ## return: bool
    ///         true    = There is or was an event.
    ///         false   = No event.
    ///
    #[allow(box_pointers)]
    pub fn dispatch(&self) -> bool {
        // Locking of the ListenerMap writer must be done
        // before locking of the Mediator.
        self.mediator.apply(self.listener_map.write());

        let Some(eve) = self.queue.lock().pop() else {
            self.queue.lock().shrink_to(self.queue_capacity);
            return false;
        };

        let Some(m) = self.listener_map.try_read() else {
            self.queue.lock().push_front(eve);
            return true;
        };

        let Some(list) = m.get(&(eve.peek_type().peek_hash())) else {
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
