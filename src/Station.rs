use crate::Division::*;
//use crate::Industry::*;

#[derive(Debug)]
pub struct Station<'system> {
    name: String,
    comment: String,
    division: &'system Division<'system>::Division,
//    industries: Vec<&'system Industry<'system>::Industry>
}

