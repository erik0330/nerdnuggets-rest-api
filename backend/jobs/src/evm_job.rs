use database::AppService;
use evm::{DAO_CONTRACTEvents, EVMClient};
use std::sync::Arc;
use utils::env::Env;

pub async fn run(
    service: Arc<AppService>,
    evm_client: Arc<EVMClient>,
    env: Env,
) -> Result<(), anyhow::Error> {
    // check if there is completed dao
    if let Ok(completed_daos) = service
        .project
        .get_completed_daos(env.dao_duration.clone())
        .await
    {
        for dao in completed_daos {
            if let Ok(_) = evm_client.approve_project(dao.proposal_id as u64).await {
                println!("approve dao: {}", dao.proposal_id);
            }
        }
    }

    let from_block_number = service
        .util
        .get_last_block_number()
        .await?
        .map(|number| number.parse::<u64>().unwrap())
        .map(|number| number + 1);

    // DAO events
    let (events, to_block_number) = evm_client
        .get_dao_contract_events(from_block_number, None)
        .await?;

    println!("DAO contract events: {:?}", events);
    for event in &events {
        match event {
            DAO_CONTRACTEvents::VotedFilter(ev) => {
                service
                    .project
                    .submit_dao_vote(
                        ev.project_id.as_u64() as i64,
                        &format!("{:?}", ev.voter),
                        ev.support,
                        ev.weight.as_u128(),
                    )
                    .await
                    .ok();
            }
            DAO_CONTRACTEvents::ProjectApprovedFilter(ev) => {
                service
                    .project
                    .finished_dao(ev.project_id.as_u64() as i64, ev.proceeded_to_funding)
                    .await
                    .ok();
            }
            _ => {}
        }
    }

    if let Some(last_block_number) = to_block_number {
        service
            .util
            .upsert_last_block_number(&last_block_number.to_string())
            .await?;
    }

    Ok(())
}
