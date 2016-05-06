/* -*- mode:rust; coding:utf-8-unix; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/07
// @date 2016/05/05

// The MIT License (MIT)
//
// Copyright (c) <2016> hanepjiv <hanepjiv@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! event_type.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::hash::{ Hasher, };
use ::std::sync::{ Arc, };
use ::std::collections::{ BTreeMap, };
/* -------------------------------------------------------------------------- */
use ::hash_combine::{ hash_combine, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventType
#[derive( Debug, )]
pub struct EventType {
    /// name
    name:       String,
    /// hash
    hash:       u32,
}
/* ========================================================================== */
/// type EventTypeRef
pub type EventTypeRef   = Arc<EventType>;
/* ========================================================================== */
impl EventType {
    /* ====================================================================== */
    /// new
    pub fn new< 'a >(name: &'a str, hash: u32) -> EventTypeRef {
        Arc::new(EventType {
            name:       name.to_string(),
            hash:       hash,
        })
    }
    /* ====================================================================== */
    /// peek_name
    pub fn peek_name< 'a >(&'a self) -> &'a str { self.name.as_ref() }
    /* ====================================================================== */
    /// peek_hash
    pub fn peek_hash(&self) -> u32 { self.hash }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// EventTypeMap
#[derive( Debug, )]
pub struct EventTypeMap(BTreeMap<u32, EventTypeRef>);
/* ========================================================================== */
impl EventTypeMap {
    /* ====================================================================== */
    /// new
    pub fn new() -> Self { EventTypeMap(BTreeMap::new()) }
    /* ====================================================================== */
    /* ---------------------------------------------------------------------- */
    /// check_type
    pub fn check_type(&self, name: &str) -> (u32, Option< EventTypeRef >) {
        let hash = hash_combine(0u32, name.as_ref());
        let &EventTypeMap(ref inner) = self;
        match inner.get(&hash) {
            Some(x)     => (hash, Some(x.clone())),
            None        => (hash, None),
        }
    }
    /* ---------------------------------------------------------------------- */
    /// new_type
    pub fn new_type(&mut self, name: &str) -> Option< EventTypeRef > {
        let (hash, ret) = self.check_type(name);
        match ret {
            x@Some(_)   => x,  // already exists
            None        => {
                let event_type = EventType::new(name, hash);
                let &mut EventTypeMap(ref mut inner) = self;
                match inner.insert(hash, event_type.clone()) {
                    Some(_)     => {
                        error!("Eventor::new_type: \
                                Hash value are in conflict. \
                                Take a different name. \"{}\"", name);
                        None
                    },
                    None        => {
                        info!("Eventor::new_type: {:#x} \"{}\"", hash, name);
                        Some(event_type)
                    },
                }
            }
        }
    }
    /* ---------------------------------------------------------------------- */
    /// peek_type
    pub fn peek_type(&self, name: &str) -> Option< EventTypeRef > {
        let (_, ret) = self.check_type(name);
        ret
    }
}
