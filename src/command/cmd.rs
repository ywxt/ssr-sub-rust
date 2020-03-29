pub trait Command {
    type Return;
    fn run(&self) -> Self::Return;
}
