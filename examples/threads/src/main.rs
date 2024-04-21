// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2024/04/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{available_parallelism, sleep, spawn, yield_now, JoinHandle},
    time::Duration,
};
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
    #[allow(trivial_casts)]
    fn peek_id(&self) -> &Uuid {
        &self.0
    }
    // ========================================================================
    fn on_event(&mut self, event: &Event, eventor: &Eventor) -> RetOnEvent {
        match event.peek_type().peek_hash() {
            4201860248 => {
                event
                    .with(|x: &u64| -> error::Result<()> {
                        println!(
                            "Listener::on_event({}): 00 {x}",
                            self.peek_id()
                        );
                        Ok(())
                    })
                    .expect("on 00");
                const E01: &str = "event_type_01";
                eventor.push_event(Event::new(
                    eventor.peek_type(E01).expect(
                        format!(r#"Listener::on_event: peek_type("{E01}")"#)
                            .as_str(),
                    ),
                    EventDataBox::new(99u64),
                ));
            }
            4201860249 => event
                .with(|x: &u64| -> error::Result<()> {
                    println!("Listener::on_event({}): 01 {x}", self.peek_id());
                    Ok(())
                })
                .expect("on 01"),
            x => println!("Listener::on_event: unknown {x}"),
        }
        RetOnEvent::Next
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn main() -> Result<()> {
    let num_cpu: u64 = <usize as TryInto<u64>>::try_into(
        available_parallelism()
            .expect("available_parallelism")
            .into(),
    )
    .expect("TryInto<u64>");

    println!("num_cpu: {num_cpu}");

    let mut threads = Vec::<JoinHandle<_>>::new();
    let alive = Arc::new(AtomicBool::new(true));
    let eventor = Arc::new(Eventor::new());

    let event_type_00 = eventor.new_type("event_type_00")?;
    println!("{:?}", event_type_00);

    let event_type_01 = eventor.new_type("event_type_01")?;
    println!("{:?}", event_type_01);

    let listener = EventListenerAelicit::new(Listener(Uuid::now_v7()))?;

    eventor.insert_listener(4201860248, &listener);
    eventor.insert_listener(4201860249, &listener);

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        threads.push(spawn(move || {
            while a.load(Ordering::Acquire) {
                while e.dispatch() {
                    yield_now();
                }
                yield_now();
            }
            format!("dispatcher {i}")
        }));
    }

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        let e00 = event_type_00.clone();
        threads.push(spawn(move || {
            let mut times = 0usize;
            while a.load(Ordering::Acquire) {
                println!("push_event_00 {i} times={times}");
                e.push_event(Event::new(e00.clone(), EventDataBox::new(i)));
                times += 1;
                sleep(Duration::from_millis(100));
            }
            format!("{}: {}", e00.peek_name(), i)
        }));
    }

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        let e01 = event_type_01.clone();
        threads.push(spawn(move || {
            let mut times = 0usize;
            while a.load(Ordering::Acquire) {
                println!("push_event_01 {i} times={times}");
                e.push_event(Event::new(e01.clone(), EventDataBox::new(i)));
                times += 1;
                sleep(Duration::from_millis(200));
            }
            format!("{}: {}", e01.peek_name(), i)
        }));
    }

    sleep(Duration::from_millis(1000));

    alive.store(false, Ordering::Release); // stop all threads.

    while threads.iter().any(|x| !x.is_finished()) {
        // waiting threads
        yield_now();
    }

    let results: Vec<_> = threads.into_iter().map(JoinHandle::join).collect();
    println!("{results:?}");

    while eventor.dispatch() {
        // event empty check
        yield_now();
    }

    Ok(())
}
