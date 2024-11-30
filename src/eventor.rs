// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/12/01

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[allow(clippy::wildcard_imports)]
use crate::inner::sync::*;
use log::info;
// ----------------------------------------------------------------------------
use super::{
    error::Result,
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
            type_map: RwLock::default(),
            queue: Mutex::new(EventQueue::default()),
            condvar_queue: Condvar::default(),
            listener_map: RwLock::default(),
            mediator: Mediator::default(),
        }
    }
}
// ============================================================================
impl Eventor {
    // ========================================================================
    /// new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    // ========================================================================
    // ------------------------------------------------------------------------
    /// `new_type`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::new_type")`
    ///
    /// # Errors
    ///
    /// `eventor::Error`
    pub fn new_type<T>(&self, name: T) -> Result<EventType>
    where
        T: AsRef<str>,
    {
        #[cfg(feature = "parking_lot")]
        return self.type_map.write().new_type(name.as_ref());

        #[cfg(not(any(feature = "parking_lot"),))]
        return self
            .type_map
            .write()
            .expect("Eventor::new_type")
            .new_type(name.as_ref());
    }
    // ------------------------------------------------------------------------
    /// `peek_typs`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::peek_type")`
    ///
    pub fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        #[cfg(feature = "parking_lot")]
        return self.type_map.read().peek_type(name.as_ref());

        #[cfg(not(any(feature = "parking_lot"),))]
        return self
            .type_map
            .read()
            .expect("Eventor::peek_type")
            .peek_type(name.as_ref());
    }
    // ========================================================================
    /// `insert_listener`
    pub fn insert_listener(&self, hash: u32, listener: EventListenerAelicit) {
        self.mediator.insert(hash, listener);
    }
    // ------------------------------------------------------------------------
    /// `remove_listener`
    pub fn remove_listener(&self, hash: u32, id: usize) {
        self.mediator.remove(hash, id);
    }
    // ========================================================================
    /// `push_event`
    /// # Panics
    ///
    /// `expect("Eventor::push_event")`
    ///
    pub fn push_event(&self, event: Event) {
        #[cfg(feature = "parking_lot")]
        self.queue.lock().push(event);
        #[cfg(not(any(feature = "parking_lot"),))]
        self.queue.lock().expect("Eventor::push_event").push(event);

        #[cfg(feature = "parking_lot")]
        let _ = self.condvar_queue.notify_one();
        #[cfg(not(any(feature = "parking_lot"),))]
        self.condvar_queue.notify_one();
    }
    // ------------------------------------------------------------------------
    /// `push_event_front`
    #[inline]
    fn push_event_front(&self, event: Event) {
        #[cfg(feature = "parking_lot")]
        self.queue.lock().push_front(event);
        #[cfg(not(any(feature = "parking_lot"),))]
        self.queue
            .lock()
            .expect("Eventor::push_event_front")
            .push_front(event);

        #[cfg(feature = "parking_lot")]
        let _ = self.condvar_queue.notify_one();
        #[cfg(not(any(feature = "parking_lot"),))]
        self.condvar_queue.notify_one();
    }
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
    /// # Panics
    ///
    /// `expect("Eventor::dispatch")`
    ///
    pub fn dispatch(&self) -> bool {
        let event = {
            #[cfg(feature = "parking_lot")]
            let mut guard = self.queue.lock();
            #[cfg(not(any(feature = "parking_lot"),))]
            let mut guard = self.queue.lock().expect("Eventor::dispatch");

            let Some(event) = guard.pop() else {
                guard.shrink();
                drop(guard);
                return false;
            };
            event
            // unlock event quere here.
        };
        self.dispatch_impl(event);
        true
    }
    // ------------------------------------------------------------------------
    #[cfg(feature = "parking_lot")]
    fn wait_for<'a>(
        &self,
        mut guard: MutexGuard<'a, EventQueue>,
    ) -> (MutexGuard<'a, EventQueue>, bool) {
        let res = self
            .condvar_queue
            .wait_for(&mut guard, std::time::Duration::from_millis(200));
        (guard, res.timed_out())
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    fn wait_for<'a>(
        &self,
        guard: MutexGuard<'a, EventQueue>,
    ) -> (MutexGuard<'a, EventQueue>, bool) {
        let (grd, res) = self
            .condvar_queue
            .wait_timeout(guard, std::time::Duration::from_millis(200))
            .expect("Eventor::dispatch_while");
        (grd, res.timed_out())
    }
    // ------------------------------------------------------------------------
    /// `dispatch_while`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::dispatch_while")`
    ///
    pub fn dispatch_while<F>(&self, mut condition: F)
    where
        F: FnMut() -> bool,
    {
        'outer: while condition() {
            let event = {
                #[cfg(feature = "parking_lot")]
                let mut guard = self.queue.lock();
                #[cfg(not(any(feature = "parking_lot"),))]
                let mut guard =
                    self.queue.lock().expect("Eventor::dispatch_while");

                'inner: loop {
                    let Some(event) = guard.pop() else {
                        guard.shrink();
                        let (grd, timed_out) = self.wait_for(guard);
                        if timed_out {
                            continue 'outer;
                        };
                        guard = grd;
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
    fn dispatch_impl(&self, event: Event) {
        // Locking of the ListenerMap writer must be done
        // before locking of the Mediator.
        #[cfg(feature = "parking_lot")]
        self.mediator.apply(self.listener_map.write());
        #[cfg(not(any(feature = "parking_lot"),))]
        self.mediator
            .apply(self.listener_map.write().expect("Eventor::dispatch_impl"));

        #[cfg(feature = "parking_lot")]
        let Some(listener_map) = self.listener_map.try_read() else {
            self.push_event_front(event);
            return;
        };
        #[cfg(not(any(feature = "parking_lot"),))]
        let listener_map = match self.listener_map.try_read() {
            Ok(x) => x,
            Err(TryLockReadError::WouldBlock) => {
                self.push_event_front(event);
                return;
            }
            Err(TryLockReadError::Poisoned(_)) => {
                panic!(
                    "Eventor::dispatch_impl: listener_map.read() poisoned."
                );
            }
        };

        let Some(listener_list) = listener_map
            .get(&(event.peek_type().peek_hash()))
            .filter(|x| !x.is_empty())
        else {
            if cfg!(debug_assertions) {
                info!("Eventor::dispatch: no listener: {event:?}");
            }
            return;
        };

        for listener in listener_list.values() {
            #[cfg(feature = "parking_lot")]
            let ret = listener.read().on_event(&event, self);
            #[cfg(not(any(feature = "parking_lot"),))]
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
