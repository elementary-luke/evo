use crate::point::*;
pub enum ForceTypes
{
    Gravity,
    Muscle,
}
    
pub struct Force
{
    pub from : ForceTypes,
    pub strength : Point,
}