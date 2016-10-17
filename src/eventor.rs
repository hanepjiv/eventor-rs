// -*- mode:rust; coding:utf-8-unix; -*-

//! eventor.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/03
//  @date 2016/10/10

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use ::std::sync::{ RwLock, };
// ============================================================================
use self::super::event::{ Event, EventQueue, };
use super::event_type::{ EventTypeRef, EventTypeMap, };
use super::event_listener::{ EventListenerAelicit,
                             EventListenerMap,
                             EventListenerWaiting, EventListenerRemoving, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
aelicit_define!(aelicit_t_eventor, TEventor);
// ----------------------------------------------------------------------------
pub use self::aelicit_t_eventor::AelicitError   as EventorAelicitError;
pub use self::aelicit_t_eventor::AelicitResult  as EventorAelicitResult;
pub use self::aelicit_t_eventor::Aelicit        as EventorAelicit;
pub use self::aelicit_t_eventor::EnableAelicitFromSelf
    as EventorEnableAelicitFromSelf;
pub use self::aelicit_t_eventor::EnableAelicitFromSelfField
    as EventorEnableAelicitFromSelfField;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// TEventor
pub trait TEventor: ::std::fmt::Debug + EventorEnableAelicitFromSelf {
    // ========================================================================
    /// new_type
    fn new_type(&self, &str) -> Option< EventTypeRef >;
    // ------------------------------------------------------------------------
    /// peek_type
    fn peek_type(&self, &str) -> Option< EventTypeRef >;
    // ////////////////////////////////////////////////////////////////////////
    // ------------------------------------------------------------------------
    /// insert_listener
    fn insert_listener(&self,
                       event_hash: u32, listener: &EventListenerAelicit) -> ();
    // ------------------------------------------------------------------------
    /// remove_listener
    fn remove_listener(&self,
                       event_hash: u32, id: ::libc::uintptr_t) -> ();
    // ========================================================================
    /// push_event
    fn push_event(&self, Event) -> ();
    // ------------------------------------------------------------------------
    /// dispatch
    fn dispatch(&self) -> bool;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Eventor
#[derive( Debug, )]
pub struct Eventor {
    // base  ==================================================================
    _eefsf:             EventorEnableAelicitFromSelfField,
    // field  =================================================================
    /// event type map
    type_map:           RwLock< EventTypeMap >,
    /// event queue
    queue:              RwLock< EventQueue >,
    /// event listener map
    listener_map:       RwLock< EventListenerMap >,
    /// event listener waiting
    listener_waiting:   EventListenerWaiting,
    /// event listener removing
    listener_removing:  EventListenerRemoving,
}
// ============================================================================
impl Eventor {
    // ========================================================================
    /// new
    pub fn new() -> EventorAelicit {
        EventorAelicit::new(Eventor {
            _eefsf:             EventorEnableAelicitFromSelfField::default(),
            type_map:           RwLock::new(EventTypeMap::default()),
            queue:              RwLock::new(EventQueue::default()),
            listener_map:       RwLock::new(EventListenerMap::default()),
            listener_waiting:   EventListenerWaiting::default(),
            listener_removing:  EventListenerRemoving::default(),
        })
    }
}
// ============================================================================
impl EventorEnableAelicitFromSelf for Eventor {
    enable_aelicit_from_self_impl_inner!(TEventor, EventorAelicit, _eefsf);
}
// ============================================================================
impl TEventor for Eventor {
    // ========================================================================
    // ------------------------------------------------------------------------
    fn new_type(&self, name: &str) -> Option< EventTypeRef > {
        self.type_map.write().expect("Eventor::new_type").new_type(name)
    }
    // ------------------------------------------------------------------------
    fn peek_type(&self, name: &str) -> Option< EventTypeRef > {
        self.type_map.read().expect("Eventor::peek_type").peek_type(name)
    }
    // ========================================================================
    // ------------------------------------------------------------------------
    fn insert_listener(&self,
                       event_hash:      u32,
                       listener:        &EventListenerAelicit) -> () {
        self.listener_waiting.insert(event_hash, listener.clone())
    }
    // ------------------------------------------------------------------------
    fn remove_listener(&self,
                       event_hash: u32, id: ::libc::uintptr_t) -> () {
        self.listener_removing.insert(event_hash, id)
    }
    // ========================================================================
    // ------------------------------------------------------------------------
    fn push_event(&self, event: Event) -> () {
        self.queue.write().expect("Eventor::push_event").push(event)
    }
    // ------------------------------------------------------------------------
    fn dispatch(&self) -> bool {
        self.listener_waiting.
            apply(&mut self.listener_map.write().expect("Eventor::dispatch"));

        self.listener_removing.
            apply(&mut self.listener_map.write().expect("Eventor::dispatch"));

        let event = self.queue.write().expect("Eventor::dispatch").pop();
        match event {
            None        => {
                self.queue.write().expect("Eventor::dispatch").shrink_to_fit();
                self.listener_waiting.shrink_to_fit();
                self.listener_removing.shrink_to_fit();
                false
            },
            Some(e)     => {
                match self.listener_map.write().expect("Eventor::dispatch").
                    get_mut(&(e.peek_type().peek_hash())) {
                        None        => {
                            debug!("Eventor::dispatch: no listener");
                        },
                        Some(list)  => {
                            for (_, ref mut listener) in list.iter_mut() {
                                listener.write().expect("Eventor::dispatch").
                                    on_event(&e,
                                             &self.aelicit_from_self()
                                             .expect("Eventor::dispatch"));
                            }
                        },
                    }
                true
            },
        }
    }

}
