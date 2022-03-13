use std::{boxed::Box, error::Error};

pub type FailableUnit = Result<(), Box<dyn Error>>;
