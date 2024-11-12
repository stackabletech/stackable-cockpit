use stackable_cockpit::platform::stacklet::Stacklet;

#[derive(Debug)]
pub enum Message {
    StackletUpdate { stacklets: Vec<Stacklet> },
    Quit,
}
