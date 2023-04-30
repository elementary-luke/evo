use crate::point::*;

#[derive(Clone, Debug)]
pub enum ForceTypes
{
    Gravity,
    Muscle,
}

#[derive(Clone, Debug)]
pub struct Force
{
    pub from : ForceTypes,
    pub strength : Point,
}