```rust
        pub struct ActionOrderManager<'a> {
            content: Vec< &'a ActionValue<'a> >
        }
        impl<'a> ActionOrderManager<'a> {
            pub fn new()->Self {
                Self { content:Vec::new() }
            }
            pub fn append(&mut self, action_value: &'a ActionValue<'a>) {
                self.content.push(action_value);
            }
            pub fn get_next_character(&self) -> &Character{
                self.content[0].owner //TODO: Implement get next character function
            }
        }

```