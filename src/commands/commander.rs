pub trait Commander {
    fn generate_settled_event() -> anyhow::Result<()>;
    fn generate_failed_event() -> anyhow::Result<()>;
}

struct TrueLayerCommandsImpl {

}

impl Commander for TrueLayerCommandsImpl {
    fn generate_settled_event() -> anyhow::Result<()> {
        todo!()
    }

    fn generate_failed_event() -> anyhow::Result<()> {
        todo!()
    }
}