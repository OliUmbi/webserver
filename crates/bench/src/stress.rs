use anyhow::Result;
use goose::prelude::*;
use tokio::runtime::Builder;

pub fn run(base: &str) -> Result<()> {
    let rt = Builder::new_multi_thread().enable_all().build()?;

    rt.block_on(async {
        GooseAttack::initialize()?
            .register_scenario(scenario!("basic").register_transaction(transaction!(index)))
            .set_default(GooseDefault::Host, format!("http://{}", base).as_str())?
            .set_default(GooseDefault::RunTime, 10)?
            .set_default(GooseDefault::Users, 100)?
            .set_default(GooseDefault::StartupTime, 3)?
            .set_default(GooseDefault::Quiet, 1)?
            .execute()
            .await
    })?;

    Ok(())
}

async fn index(user: &mut GooseUser) -> TransactionResult {
    user.get("/index.html").await?;
    Ok(())
}
