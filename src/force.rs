use crate::point::*;

#[derive(Clone)]
pub enum ForceTypes
{
    Gravity,
    Muscle,
}

#[derive(Clone)]
pub struct Force
{
    pub from : ForceTypes,
    pub strength : Point,
}