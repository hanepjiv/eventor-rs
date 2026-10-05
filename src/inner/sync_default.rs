// -*- coding:utf-8-unix; -*-

//! `sync_default.rs`.

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/05/06
//  @date 2026/10/04

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub(crate) use std::sync::{
    Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type `TryLockError`.
#[expect(dead_code, reason = "checked")]
pub(crate) type TryLockError<'a, T> =
    std::sync::TryLockError<MutexGuard<'a, T>>;
// ============================================================================
/// type `TryLockReadError`.
pub(crate) type TryLockReadError<'a, T> =
    std::sync::TryLockError<RwLockReadGuard<'a, T>>;
/// type `TryLockWriteError`.
pub(crate) type TryLockWriteError<'a, T> =
    std::sync::TryLockError<RwLockWriteGuard<'a, T>>;
