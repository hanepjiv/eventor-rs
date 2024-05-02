// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/05/02

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use log::info;
use parking_lot::{Condvar, Mutex, RwLock};
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
    /// event queue
    queue: Mutex<EventQueue>,
    /// condvar queue
    condvar_queue: Condvar,
    /// event listener map
    listener_map: RwLock<ListenerMap>,
    /// mediator
    mediator: Mediator,
}
// ============================================================================
impl Default for Eventor {
    fn default() -> Self {
        Self {
            type_map: Default::default(),
            queue: Mutex::new(EventQueue::with_capacity(64usize)),
            condvar_queue: Default::default(),
            listener_map: Default::default(),
            mediator: Default::default(),
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
        let mut guard = self.queue.lock();
        guard.push(event);
        let _ = self.condvar_queue.notify_one();
    }
    // ------------------------------------------------------------------------
    /// push_event_front
    #[inline]
    fn push_event_front(&self, event: Event) {
        let mut guard = self.queue.lock();
        guard.push_front(event);
        let _ = self.condvar_queue.notify_one();
    }
    // ------------------------------------------------------------------------
    /*
        /// pop_event_wait
        #[inline]
        fn pop_event_wait(&self) -> Event {

    }
        */
    // ========================================================================
    ///
    /// # dispatch
    ///
    /// Process one event.
    ///
    /// ## return: bool
    /// true    = There is or was an event.
    /// false   = No event.
    ///
    pub fn dispatch(&self) -> bool {
        let event = {
            let mut guard = self.queue.lock();
            let Some(event) = guard.pop() else {
                guard.shrink();
                return false;
            };
            event
            // unlock event quere here.
        };
        self.dispatch_impl(event);
        true
    }
    // ------------------------------------------------------------------------
    /// dispatch_while
    pub fn dispatch_while<F>(&self, mut condition: F)
    where
        F: FnMut() -> bool,
    {
        'outer: while condition() {
            let event = {
                let mut guard = self.queue.lock();
                'inner: loop {
                    let Some(event) = guard.pop() else {
                        guard.shrink();
                        if self
                            .condvar_queue
                            .wait_for(
                                &mut guard,
                                std::time::Duration::from_millis(20),
                            )
                            .timed_out()
                        {
                            continue 'outer;
                        };
                        continue 'inner;
                    };
                    break event;
                }
                // unlock event quere here.
            };
            self.dispatch_impl(event);
        }
    }
    // ------------------------------------------------------------------------
    #[allow(box_pointers)]
    fn dispatch_impl(&self, event: Event) {
        // Locking of the ListenerMap writer must be done
        // before locking of the Mediator.
        self.mediator.apply(self.listener_map.write());

        let Some(m) = self.listener_map.try_read() else {
            self.push_event_front(event);
            return;
        };

        let Some(list) =
            (if let Some(x) = m.get(&(event.peek_type().peek_hash())) {
                (!x.is_empty()).then_some(x)
            } else {
                None
            })
        else {
            if cfg!(debug_assertions) {
                info!("Eventor::dispatch: no listener: {event:?}");
            }
            return;
        };

        for (_, listener) in list.iter() {
            #[cfg(feature = "elicit-parking_lot")]
            let ret = listener.read().on_event(&event, self);

            #[cfg(not(any(feature = "elicit-parking_lot"),))]
            let ret = listener
                .read()
                .expect("Eventor::dispatch")
                .on_event(&event, self);

            if let RetOnEvent::Complete = ret {
                break;
            }
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
