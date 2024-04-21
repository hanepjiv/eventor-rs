// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2024/04/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::thread::yield_now;
// ----------------------------------------------------------------------------
use eventor::{
    event_listener_aelicit_author,
    event_listener_aelicit_author::Aelicit as EventListenerAelicit, Event,
    EventDataBox, EventListener, Eventor, RetOnEvent,
};
use uuid::Uuid;
// mod  =======================================================================
mod error;
use error::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Listener
#[derive(Debug, elicit::Aelicit)]
#[aelicit_mod_author(event_listener_aelicit_author)]
pub(crate) struct Listener(Uuid);
// ============================================================================
impl EventListener for Listener {
    // ========================================================================
    fn peek_id(&self) -> &Uuid {
        &self.0
    }
    // ========================================================================
    fn on_event(&mut self, event: &Event, _eventor: &Eventor) -> RetOnEvent {
        match event.peek_type().peek_hash() {
            4201860248 => {
                event
                    .with(|x: &i32| -> error::Result<()> {
                        println!(
                            "Listener::on_event({}): 00 {x}",
                            self.peek_id()
                        );
                        Ok(())
                    })
                    .expect("on 00");
                RetOnEvent::Complete
            }
            4201860249 => {
                event
                    .with(|x: &i32| -> error::Result<()> {
                        println!(
                            "Listener::on_event({}): 01 {x}",
                            self.peek_id()
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
    println!("event_type_00: {:?}", event_type_00);

    let event_type_01 = eventor.new_type("event_type_01")?;
    println!("event_type_00: {:?}", event_type_01);

    {
        let listener_id = Uuid::now_v7();
        println!("listener: {listener_id}");
        eventor.insert_listener(
            4201860248,
            listener_id,
            EventListenerAelicit::new(Listener(listener_id))?,
        );
    }

    {
        let listener_id = Uuid::now_v7();
        println!("listener: {listener_id}");
        eventor.insert_listener(
            4201860249,
            listener_id,
            EventListenerAelicit::new(Listener(listener_id))?,
        )
    }

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
