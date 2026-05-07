use crate::entities::{QueryTerm, TermId};

pub type Comparator = fn(&str, &QueryTerm) -> Option<TermId>;
