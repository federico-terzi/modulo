mod form;

pub fn show_window() {
    unsafe {crate::form::show_window()}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
