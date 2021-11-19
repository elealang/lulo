//! CLI util
//! Common functions for Lulo CLI interface

pub fn val_or_cli_error<T, E>(res: Result<T, E>, msg: &dyn Fn(E) -> String) -> T {
    match res {
        Ok(val) => return val,
        Err(e)  => panic!("{}", msg(e)),
    }
}
