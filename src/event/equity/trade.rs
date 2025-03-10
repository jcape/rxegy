//! Equity Trade Events

use crate::{event::Common, impl_wrapper_on_newtype, object::Kind as ObjectKind};
use std::{ffi::c_void, ptr::NonNull};

/// An equity trade event
#[derive(Debug)]
pub struct Event(NonNull<c_void>);

impl_wrapper_on_newtype!(Event, ObjectKind::EventEquityTrade);

impl Common for Event {}
