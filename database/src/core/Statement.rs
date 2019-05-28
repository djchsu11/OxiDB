pub mod Statement{
    pub enum StatementKind{
        StatementInsert,
        StatementSelect,
        StatementDelete
    }

    trait StatementCreate{
        fn make(StatementKind) -> String;
    }

    impl make for StatementKind::StatementInsert{
        fn make(StatementKind) -> String {
            format!("This is an insert statement!");
        }
    }
}