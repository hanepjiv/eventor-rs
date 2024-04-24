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
        self.type_map.write().new_type(name.as_ref())
    }
    // ------------------------------------------------------------------------
    /// fn peek_typs
    pub fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        self.type_map.read().peek_type(name.as_ref())
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
        self.queue.lock().push(event)
    }
    // ------------------------------------------------------------------------
    /// fn dispatch
    #[allow(box_pointers)]
    pub fn dispatch(&self) -> bool {
        self.mediator.apply(&self.listener_map);

        let event = self.queue.lock().pop();

        let Some(eve) = event else {
            self.queue.lock().shrink_to_fit();
            return false;
        };

        let Some(list) =
            self.listener_map.read().get(&(eve.peek_type().peek_hash()))
        else {
            if cfg!(debug_assertions) {
                info!("Eventor::dispatch: no listener: {eve:?}");
            }
            return true;
        };

        'outer: loop {
            if let Some(x) = list.try_read() {
                for (_, listener) in x.iter() {
                    if let RetOnEvent::Complete = listener
                        .read()
                        .expect("Eventor::dispatch: listener")
                        .on_event(&eve, self)
                    {
                        break;
                    }
                }
                break 'outer;
            }
            std::thread::yield_now();
            std::thread::sleep(std::time::Duration::from_millis(200));
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
