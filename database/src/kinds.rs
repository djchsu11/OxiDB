pub(crate) enum MetaCommandKind{
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

#[derive(PartialEq)]
pub(crate) enum ExecutionStatusKind{
    ExitFailure,
    ExitSuccess
}