use database::AppService;
use evm::{DAO_CONTRACTEvents, EVMClient, FUNDING_CONTRACTEvents};
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
            match evm_client.approve_project(dao.proposal_id as u64).await {
                Ok(_) => println!("approve dao: {}", dao.proposal_id),
                Err(e) => println!("approve dao error: {}", e.to_string()),
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

    // println!("DAO contract events: {:?}", events);
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

    // Funding events
    let (events, last_block_number) = evm_client
        .get_funding_contract_events(from_block_number, to_block_number)
        .await?;

    // println!("Funding contract events: {:?}", events);
    for event in &events {
        match event {
            FUNDING_CONTRACTEvents::DonatedFilter(ev) => {
                service
                    .project
                    .donate_milestone(
                        ev.project_id.as_u64() as i64,
                        ev.milestone_index.as_u32() as i16,
                        &format!("{:?}", ev.donor),
                        ev.amount.as_u128(),
                    )
                    .await
                    .ok();
            }
            FUNDING_CONTRACTEvents::PredictionPlacedFilter(ev) => {
                if let Err(e) = service
                    .prediction_placement
                    .create_prediction_placement(
                        &format!("{:?}", ev.user),
                        ev.project_id.as_u64() as i64,
                        ev.milestone_index.as_u64() as i64,
                        ev.predicts_success,
                        ev.nerd_amount.as_u128(),
                        to_block_number.unwrap_or_default() as i64,
                    )
                    .await
                {
                    println!("Failed to store prediction placement: {}", e);
                } else {
                    println!(
                        "Stored prediction placement for user {:?}, project {}, milestone {}",
                        ev.user, ev.project_id, ev.milestone_index
                    );
                }
            }
            // FUNDING_CONTRACTEvents::MilestoneFinalizedFilter(ev) => {
            //     let project_id = ev.project_id.as_u64() as i64;
            //     let milestone_index = ev.milestone_index.as_u32() as i16;
            //     let success = ev.success;

            //     // Get the project by proposal ID
            //     if let Ok(Some(project)) =
            //         service.project.get_project_by_proposal_id(project_id).await
            //     {
            //         let milestones = service
            //             .project
            //             .get_milestones(&project.id.to_string())
            //             .await;

            //         if let Ok(milestones) = milestones {
            //             // Find the current milestone by number (milestone_index is 0-based, but our system uses 1-based)
            //             let current_milestone_number = milestone_index + 1;
            //             if let Some(current_milestone) = milestones
            //                 .iter()
            //                 .find(|m| m.number == current_milestone_number)
            //             {
            //                 // Update current milestone status based on success
            //                 let new_status = if success { 2 } else { 3 }; // 2=success, 3=giveup

            //                 if let Err(e) = service
            //                     .project
            //                     .update_milestone_status(
            //                         &current_milestone.id.to_string(),
            //                         new_status,
            //                     )
            //                     .await
            //                 {
            //                     println!(
            //                         "Failed to update milestone {} status: {}",
            //                         current_milestone.id, e
            //                     );
            //                 } else {
            //                     println!(
            //                         "Updated milestone {} status to {}",
            //                         current_milestone.id, new_status
            //                     );
            //                 }

            //                 // If milestone was successful, update user's claimed funding amount
            //                 if success {
            //                     if let Err(e) = service
            //                         .project
            //                         .donate_project(project.id, current_milestone.funding_amount)
            //                         .await
            //                     {
            //                         println!("Failed to update project funding amount: {}", e);
            //                     } else {
            //                         println!(
            //                             "Updated project {} funding amount by {}",
            //                             project.id, current_milestone.funding_amount
            //                         );
            //                     }
            //                 }

            //                 // Find and start the next milestone if it exists
            //                 let next_milestone_number = current_milestone_number + 1;
            //                 if let Some(next_milestone) = milestones
            //                     .iter()
            //                     .find(|m| m.number == next_milestone_number)
            //                 {
            //                     // Update next milestone status to "in process" (1)
            //                     if let Err(e) = service
            //                         .project
            //                         .update_milestone_status(&next_milestone.id.to_string(), 1)
            //                         .await
            //                     {
            //                         println!(
            //                             "Failed to update next milestone {} status: {}",
            //                             next_milestone.id, e
            //                         );
            //                     } else {
            //                         println!(
            //                             "Started next milestone {} (in process)",
            //                             next_milestone.id
            //                         );
            //                     }
            //                 } else {
            //                     println!("No next milestone found for project {}", project.id);
            //                 }
            //             } else {
            //                 println!(
            //                     "Milestone with number {} not found for project {}",
            //                     current_milestone_number, project.id
            //                 );
            //             }
            //         } else {
            //             println!("Failed to get milestones for project {}", project.id);
            //         }
            //     } else {
            //         println!("Project with proposal ID {} not found", project_id);
            //     }
            // }
            _ => {}
        }
    }

    // // Prediction contract events
    // let (events, prediction_last_block_number) = evm_client
    //     .get_prediction_contract_events(from_block_number, to_block_number)
    //     .await?;

    // // println!("Prediction contract events: {:?}", events);
    // for event in &events {
    //     match event {
    //         PREDICTION_CONTRACTEvents::PredictionPlacedFilter(ev) => {
    //             // Store prediction placement in database
    // if let Err(e) = service
    //     .prediction_placement
    //     .create_prediction_placement(
    //         &format!("{:?}", ev.user),
    //         ev.project_id.as_u64() as i64,
    //         ev.milestone_index.as_u64() as i64,
    //         ev.predicts_success,
    //         ev.nerd_amount.as_u64() as i64,
    //         to_block_number.unwrap_or_default() as i64, // Use the current block number
    //         "unknown", // Transaction hash not available in event data
    //     )
    //     .await
    // {
    //     println!("Failed to store prediction placement: {}", e);
    // } else {
    //     println!(
    //         "Stored prediction placement for user {:?}, project {}, milestone {}",
    //         ev.user, ev.project_id, ev.milestone_index
    //     );
    // }
    //         }
    //         _ => {}
    //     }
    // }

    // // Use the latest block number from all contract events
    // let final_block_number = last_block_number
    //     .or(last_block_number)
    //     .or(to_block_number);

    if let Some(last_block_number) = last_block_number {
        service
            .util
            .upsert_last_block_number(&last_block_number.to_string())
            .await?;
    }

    Ok(())
}
