use crate::error;

pub trait Command {
    type Return;
    fn run(&self) -> error::Result<Self::Return>;
}
