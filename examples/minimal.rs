// -*- mode:rust; coding:utf-8-unix; -*-

//! minimal.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2025/01/20

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::thread::yield_now;
// ----------------------------------------------------------------------------
use eventor::{
    Event, EventDataBox, EventListener, Eventor, RetOnEvent, SyncResult,
    event_listener_aelicit_author,
    event_listener_aelicit_author::{
        Aelicit as EventListenerAelicit, AelicitBase,
    },
};
// ----------------------------------------------------------------------------
use hash_combine as _;
use log as _;
#[cfg(feature = "parking_lot")]
use parking_lot as _;
// mod  =======================================================================
mod inner;
use inner::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Listener
#[derive(Debug, elicit::Aelicit)]
#[aelicit_mod_author(event_listener_aelicit_author)]
struct Listener;
// ============================================================================
impl EventListener for Listener {
    // ========================================================================
    /// `on_event`
    #[allow(clippy::missing_panics_doc)]
    fn on_event(&self, event: &Event, _eventor: &Eventor) -> RetOnEvent {
        match event.peek_type().peek_hash() {
            4_201_860_248 => {
                event
                    .with(|x: &i32| -> SyncResult<'_, ()> {
                        println!(
                            "Listener::on_event({}): 00 {x}",
                            self.usizeptr()
                        );
                        Ok(())
                    })
                    .expect("on 00");
                RetOnEvent::Complete
            }
            4_201_860_249 => {
                event
                    .with(|x: &i32| -> SyncResult<'_, ()> {
                        println!(
                            "Listener::on_event({}): 01 {x}",
                            self.usizeptr()
                        );
                        Ok(())
                    })
                    .expect("on 01");
                RetOnEvent::Complete
            }
            x => {
                println!("Listener::on_event: unknown hash {x}");
                RetOnEvent::Next
            }
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn main() -> Result<()> {
    let eventor = Eventor::new();

    let event_type_00 = eventor.new_type("event_type_00")?;
    println!("event_type_00: {event_type_00:?}");

    let event_type_01 = eventor.new_type("event_type_01")?;
    println!("event_type_00: {event_type_01:?}");

    eventor
        .insert_listener(4_201_860_248, EventListenerAelicit::new(Listener)?);

    eventor
        .insert_listener(4_201_860_249, EventListenerAelicit::new(Listener)?);

    for i in 0..2 {
        eventor.push_event(Event::new(
            event_type_00.clone(),
            EventDataBox::new(i),
        ));
        println!("push_event: event_00 {i}");

        eventor.push_event(Event::new(
            event_type_01.clone(),
            EventDataBox::new(i),
        ));
        println!("push_event: event_01 {i}");
    }

    println!(" *** start dispatch.");

    while eventor.dispatch() {
        yield_now();
    }

    Ok(())
}
