use crate::model::*;
use crate::store::*;

pub trait Aggregate {}

impl Aggregate for dyn Note {}
