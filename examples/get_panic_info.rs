use std::{any::Any, ops::Deref, panic};

fn main() {
    {
        let p = panic::catch_unwind(|| panic!("A wild panic appeared!"));
        let err = p.unwrap_err();

        println!("{:#?}", err.downcast_ref::<panic::PanicInfo>()); // None
        println!("{:#?}", err.downcast_ref::<&str>()); // Some("A wild panic appeared!")
    }
    {
        // get_panic_message utility
        let p = panic::catch_unwind(|| panic!("A wild panic appeared!"));
        let err = p.unwrap_err();
        let message = get_panic_message(&err).unwrap();
        println!("{message}");
    }
}

pub fn get_panic_message(cause: &Box<dyn Any + Send>) -> Option<&str> {
    cause
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| cause.downcast_ref::<&'static str>().map(Deref::deref))
}