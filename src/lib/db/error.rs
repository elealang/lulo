//! Lib Database errors
//! database errors


pub enum Error {
    Sync(ErrorSync),
}


pub struct ErrorSync {
    pub err: String,    
}
