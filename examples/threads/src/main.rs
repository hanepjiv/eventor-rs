// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2024/04/24

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
        Aelicit as EventListenerAelicit, AelicitFromSelf,
        AelicitFromSelfField as EventListenerAelicitFromSelfField,
    },
    Event, EventDataBox, EventListener, Eventor, RetOnEvent,
};
use uuid::Uuid;
// mod  =======================================================================
mod error;
use error::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Listener
#[derive(Debug, Default, elicit::Aelicit)]
#[aelicit_mod_author(event_listener_aelicit_author)]
pub(crate) struct Listener {
    #[aelicit_from_self_field]
    _fsf: EventListenerAelicitFromSelfField,
    uuid: Uuid,
}
// ============================================================================
impl Listener {
    pub fn new_aelicit(uuid: Uuid) -> EventListenerAelicit {
        EventListenerAelicit::new(Listener {
            uuid,
            ..Self::default()
        })
        .expect("Listener::new")
    }
}
// ============================================================================
impl EventListener for Listener {
    // ========================================================================
    // ========================================================================
    fn peek_id(&self) -> &Uuid {
        &self.uuid
    }
    // ========================================================================
    fn on_event(&self, event: &Event, eventor: &Eventor) -> RetOnEvent {
        match event.peek_type().peek_hash() {
            4201860248 => {
                event
                    .with(|x: &u64| -> error::Result<()> {
                        println!("{} event_00 data({x})", self.peek_id());
                        Ok(())
                    })
                    .expect("on 00");

                const E01: &str = "event_type_01";
                eventor.push_event(Event::new(
                    eventor.peek_type(E01).unwrap_or_else(|| {
                        panic!(r#"Listener::on_event: peek_type("{E01}")"#)
                    }),
                    EventDataBox::new(99u64),
                ));
                eventor.remove_listener(4201860248, self.peek_id());

                RetOnEvent::Next
            }
            4201860249 => {
                event
                    .with(|x: &u64| -> error::Result<()> {
                        println!("{} event_01 data({x})", self.peek_id());

                        Ok(())
                    })
                    .expect("on 01");
                eventor.insert_listener(
                    4201860248,
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
    println!("{:?}", event_type_00);

    let event_type_01 = eventor.new_type("event_type_01")?;
    println!("{:?}", event_type_01);

    for _ in 0..1 {
        let listener = Listener::new_aelicit(Uuid::now_v7());
        eventor.insert_listener(4201860248, listener.clone());
        eventor.insert_listener(4201860249, listener);
    }

    for i in 0..num_cpu {
        let a = alive.clone();
        let e = eventor.clone();
        threads.push(spawn(move || {
            while a.load(Ordering::Acquire) {
                while e.dispatch() {
                    // yield_now();
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                yield_now();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
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
                sleep(Duration::from_millis(10));
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
                sleep(Duration::from_millis(20));
            }
            format!("pusher {} thread({i})", e01.peek_name())
        }));
    }

    sleep(Duration::from_millis(50));

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
