// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2025/04/07

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use log::info;
// ----------------------------------------------------------------------------
use super::{
    error::Result,
    event::{Event, EventQueue},
    event_listener::{
        RetOnEvent, aelicit_user::Aelicit as EventListenerAelicit,
    },
    event_type::EventType,
    inner::{ListenerMap, Mediator, TypeMap},
};

#[cfg(feature = "parking_lot")]
use super::inner::sync::{Condvar, Mutex, MutexGuard, RwLock};

#[cfg(not(any(feature = "parking_lot"),))]
use super::inner::sync::{
    Condvar, Mutex, MutexGuard, RwLock, TryLockReadError,
};
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
    #[inline]
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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    // ========================================================================
    // ------------------------------------------------------------------------
    #[cfg(feature = "parking_lot")]
    /// `new_type`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::new_type")`
    ///
    /// # Errors
    ///
    /// `eventor::Error`
    #[inline]
    pub fn new_type<T>(&self, name: T) -> Result<EventType>
    where
        T: AsRef<str>,
    {
        return self.type_map.write().new_type(name.as_ref());
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// `new_type`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::new_type")`
    ///
    /// # Errors
    ///
    /// `eventor::Error`
    #[expect(
        clippy::unwrap_in_result,
        clippy::expect_used,
        reason = "checked"
    )]
    #[inline]
    pub fn new_type<T>(&self, name: T) -> Result<EventType>
    where
        T: AsRef<str>,
    {
        return self
            .type_map
            .write()
            .expect("Eventor::new_type")
            .new_type(name.as_ref());
    }
    // ------------------------------------------------------------------------
    #[cfg(feature = "parking_lot")]
    /// `peek_typs`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::peek_type")`
    #[inline]
    pub fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        return self.type_map.read().peek_type(name.as_ref());
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// `peek_typs`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::peek_type")`
    #[expect(
        clippy::unwrap_in_result,
        clippy::expect_used,
        reason = "checked"
    )]
    #[inline]
    pub fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        return self
            .type_map
            .read()
            .expect("Eventor::peek_type")
            .peek_type(name.as_ref());
    }
    // ========================================================================
    /// `insert_listener`
    #[inline]
    pub fn insert_listener(&self, hash: u32, listener: EventListenerAelicit) {
        self.mediator.insert(hash, listener);
    }
    // ------------------------------------------------------------------------
    /// `remove_listener`
    #[inline]
    pub fn remove_listener(&self, hash: u32, id: usize) {
        self.mediator.remove(hash, id);
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// `push_event`
    /// # Panics
    ///
    /// `expect("Eventor::push_event")`
    #[inline]
    pub fn push_event(&self, event: Event) {
        self.queue.lock().push(event);
        let _ = self.condvar_queue.notify_one();
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// `push_event`
    /// # Panics
    ///
    /// `expect("Eventor::push_event")`
    #[expect(clippy::expect_used, reason = "checked")]
    #[inline]
    pub fn push_event(&self, event: Event) {
        self.queue.lock().expect("Eventor::push_event").push(event);
        self.condvar_queue.notify_one();
    }
    // ------------------------------------------------------------------------
    #[cfg(feature = "parking_lot")]
    /// `push_event_front`
    #[inline]
    fn push_event_front(&self, event: Event) {
        self.queue.lock().push_front(event);
        let _ = self.condvar_queue.notify_one();
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// `push_event_front`
    #[expect(clippy::expect_used, reason = "checked")]
    #[inline]
    fn push_event_front(&self, event: Event) {
        self.queue
            .lock()
            .expect("Eventor::push_event_front")
            .push_front(event);
        self.condvar_queue.notify_one();
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    fn lock_guard(&self) -> MutexGuard<'_, EventQueue> {
        self.queue.lock()
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    #[expect(clippy::expect_used, reason = "checked")]
    fn lock_guard(&self) -> MutexGuard<'_, EventQueue> {
        self.queue.lock().expect("Eventor::lock_guard")
    }
    // ------------------------------------------------------------------------
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
    #[inline]
    pub fn dispatch(&self) -> bool {
        let event = {
            let mut guard = self.lock_guard();
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
            .wait_for(&mut guard, core::time::Duration::from_millis(200));
        (guard, res.timed_out())
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    #[expect(clippy::expect_used, reason = "checked")]
    fn wait_for<'a>(
        &self,
        guard: MutexGuard<'a, EventQueue>,
    ) -> (MutexGuard<'a, EventQueue>, bool) {
        let (grd, res) = self
            .condvar_queue
            .wait_timeout(guard, core::time::Duration::from_millis(200))
            .expect("Eventor::dispatch_while");
        (grd, res.timed_out())
    }
    // ------------------------------------------------------------------------
    /// `dispatch_while`
    ///
    /// # Panics
    ///
    /// `expect("Eventor::dispatch_while")`
    #[expect(
        clippy::significant_drop_tightening,
        clippy::mixed_read_write_in_expression,
        reason = "checked"
    )]
    #[inline]
    pub fn dispatch_while<F>(&self, mut condition: F)
    where
        F: FnMut() -> bool,
    {
        'outer: while condition() {
            let event = {
                let mut guard = self.lock_guard();

                'inner: loop {
                    let Some(event) = guard.pop() else {
                        guard.shrink();
                        let (grd, timed_out) = self.wait_for(guard);
                        if timed_out {
                            continue 'outer;
                        }
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
    #[cfg(feature = "parking_lot")]
    fn dispatch_impl(&self, event: Event) {
        // Locking of the ListenerMap writer must be done
        // before locking of the Mediator.
        self.mediator.apply(self.listener_map.write());

        let listener_list_cloned = {
            let Some(listener_map) = self.listener_map.try_read() else {
                self.push_event_front(event);
                return;
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

            listener_list.clone()
        };

        for listener in listener_list_cloned.values() {
            let ret = listener.read().on_event(&event, self);
            if ret == RetOnEvent::Complete {
                break;
            }
        }
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    #[expect(
        clippy::significant_drop_tightening,
        clippy::expect_used,
        clippy::panic,
        reason = "checked"
    )]
    fn dispatch_impl(&self, event: Event) {
        // Locking of the ListenerMap writer must be done
        // before locking of the Mediator.
        self.mediator
            .apply(self.listener_map.write().expect("Eventor::dispatch_impl"));

        let listener_list_cloned = {
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

            listener_list.clone()
        };

        for listener in listener_list_cloned.values() {
            let ret = listener
                .read()
                .expect("Eventor::dispatch")
                .on_event(&event, self);

            if ret == RetOnEvent::Complete {
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
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Eventor>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Eventor>();
    }
}
