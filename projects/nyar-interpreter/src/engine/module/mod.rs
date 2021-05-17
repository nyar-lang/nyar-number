#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod flags;

mod context;
mod instance;
mod manager;
mod optimized;

pub use self::{
    context::{DefaultDecimalHandler, IntegerHandlerManager, NyarContext, NyarIndexSystem, NYAR_CONTEXT_PRESET},
    flags::NyarReadWrite,
    instance::ModuleInstance,
    manager::ModuleManager,
};

use crate::{
    value::{function::FunctionPrototype, variable::Variable},
    NyarError, Result,
};
use shredder::{
    marker::{GcDrop, GcSafe},
    plumbing::check_gc_drop,
    GRwLock, Gc, Scan, Scanner,
};
use std::{collections::HashMap, sync::RwLock};

pub type SharedModule = Gc<RwLock<ModuleInstance>>;
