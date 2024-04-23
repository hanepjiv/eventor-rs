// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2024/04/24

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
        listener: EventListenerAelicit,
    ) {
        self.mediator.insert(event_hash, listener)
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
        self.mediator.apply(&self.listener_map);

        let event = self
            .queue
            .lock()
            .expect("Eventor::dispatch: event queue")
            .pop();

        let Some(eve) = event else {
            self.queue
                .lock()
                .expect("Eventor::dispatch: event queue shrink")
                .shrink_to_fit();
            return false;
        };

        let list = {
            let Some(list) = self
                .listener_map
                .read()
                .expect("Eventor::dispatch: listener_map")
                .get(&(eve.peek_type().peek_hash()))
            else {
                if cfg!(debug_assertions) {
                    info!("Eventor::dispatch: no listener: {eve:?}");
                }
                return true;
            };
            list
        };

        match list.try_read() {
            Ok(x) => {
                for (_, listener) in x.iter() {
                    if let RetOnEvent::Complete = listener
                        .read()
                        .expect("Eventor::dispatch: listener")
                        .on_event(&eve, self)
                    {
                        break;
                    }
                }
            }
            Err(e) => match e {
                std::sync::TryLockError::WouldBlock => {
                    self.queue
                        .lock()
                        .expect("Eventor::dispatch: event queue return")
                        .push_front(eve);
                    return true;
                }
                _ => {
                    // poisoned
                    panic!("Eventor::dispatch: listener_map list");
                }
            },
        }
        /*
            for (_, listener) in list
            .read()
            .expect("Eventor::dispatch: listener_map list")
            .iter()
            {
            if let RetOnEvent::Complete = listener
            .read()
            .expect("Eventor::dispatch: listener")
            .on_event(&e, self)
            {
            break;
        }
        }
         */

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
