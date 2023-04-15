pub mod cat34;
pub mod cat_error;
pub mod field_spec;
pub mod tools;


#[cfg(test)]
mod tests {
    use super::*;
    use cat34::Cat34Message;

    #[test]
    fn cat34() {
        let message = Cat34Message::new(cat34::MessageType::NorthMarker);

        println!("{:?}", message);
    }
}
