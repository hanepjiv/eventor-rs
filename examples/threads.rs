// -*- mode:rust; coding:utf-8-unix; -*-

//! threads.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2025/01/20

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
    event_listener_aelicit_author::{
        Aelicit as EventListenerAelicit, AelicitBase, AelicitFromSelf,
        AelicitFromSelfField as EventListenerAelicitFromSelfField,
    },
    Event, EventDataBox, EventListener, Eventor, RetOnEvent, SyncResult,
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
static E01: &str = "event_type_01";
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Listener
#[derive(Debug, Default, elicit::Aelicit)]
#[aelicit_mod_author(event_listener_aelicit_author)]
pub struct Listener {
    #[aelicit_from_self_field]
    _fsf: EventListenerAelicitFromSelfField,
}
// ============================================================================
impl Listener {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    /// fn `new_aelicit`
    pub fn new_aelicit() -> EventListenerAelicit {
        EventListenerAelicit::new(Listener { ..Self::default() })
            .expect("Listener::new")
    }
}
// ============================================================================
impl EventListener for Listener {
    // ========================================================================
    fn on_event(&self, event: &Event, eventor: &Eventor) -> RetOnEvent {
        match event.peek_type().peek_hash() {
            4_201_860_248 => {
                event
                    .with(|x: &u64| -> SyncResult<'_, ()> {
                        println!("0x{:x} event_00 data({x})", self.usizeptr());
                        Ok(())
                    })
                    .expect("on 00");

                eventor.push_event(Event::new(
                    eventor.peek_type(E01).unwrap_or_else(|| {
                        panic!(r#"Listener::on_event: peek_type("{E01}")"#)
                    }),
                    EventDataBox::new(99u64),
                ));
                eventor.remove_listener(4_201_860_248, self.usizeptr());

                RetOnEvent::Next
            }
            4_201_860_249 => {
                event
                    .with(|x: &u64| -> SyncResult<'_, ()> {
                        println!("0x{:x} event_01 data({x})", self.usizeptr());
                        Ok(())
                    })
                    .expect("on 01");
                eventor.insert_listener(
                    4_201_860_248,
                    self.aelicit_from_self()
                        .expect("Listener::on_event: aelicit_from_self"),
                );
                RetOnEvent::Next
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
    println!("{event_type_00:?}");

    let event_type_01 = eventor.new_type("event_type_01")?;
    println!("{event_type_01:?}");

    for _ in 0..num_cpu {
        let listener = Listener::new_aelicit();
        eventor.insert_listener(4_201_860_248, listener.clone());
        eventor.insert_listener(4_201_860_249, listener);
    }

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        threads.push(spawn(move || {
            e.dispatch_while(|| a.load(Ordering::Acquire));
            format!("dispatcher thread({i})")
        }));
    }

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        let e00 = event_type_00.clone();
        threads.push(spawn(move || {
            let mut times = 0usize;
            while a.load(Ordering::Acquire) {
                println!("push event_00 thread({i}) times={times}");
                e.push_event(Event::new(e00.clone(), EventDataBox::new(i)));
                times += 1;
                sleep(Duration::from_millis(100));
            }
            format!("pusher {} thread({i})", e00.peek_name())
        }));
    }

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        let e01 = event_type_01.clone();
        threads.push(spawn(move || {
            let mut times = 0usize;
            while a.load(Ordering::Acquire) {
                println!("push event_01 thread({i}) times={times}");
                e.push_event(Event::new(e01.clone(), EventDataBox::new(i)));
                times += 1;
                sleep(Duration::from_millis(200));
            }
            format!("pusher {} thread({i})", e01.peek_name())
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
