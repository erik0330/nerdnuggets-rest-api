// use database::AppService;
// use evm::{DAO_CONTRACTEvents, EVMClient, FUNDING_CONTRACTEvents};
// use std::sync::Arc;
// use utils::{commons::get_proposal_id_from_article_id, env::Env};

// pub async fn run(
//     service: Arc<AppService>,
//     evm_client: Arc<EVMClient>,
//     env: Env,
// ) -> Result<(), anyhow::Error> {
//     let from_block_number = service
//         .util
//         .get_last_block_number()
//         .await?
//         .map(|number| number.parse::<u64>().unwrap())
//         .map(|number| number + 1);

//     // DAO events
//     let (events, to_block_number) = evm_client
//         .get_dao_contract_events(from_block_number, None)
//         .await?;

//     println!("DAO contract events: {:?}", events);
//     for event in &events {
//         match event {
//             DAO_CONTRACTEvents::ProposalCreatedFilter(_ev) => {}
//             DAO_CONTRACTEvents::VotedFilter(_ev) => {}
//             DAO_CONTRACTEvents::ProposalFinalizedFilter(ev) => {
//                 service
//                     .article
//                     .update_dao_result(&ev.proposal_id.as_u64().to_string(), ev.approved)
//                     .await;
//             }
//             _ => {}
//         }
//     }

//     // Funding events
//     let (events, last_block_number) = evm_client
//         .get_funding_contract_events(from_block_number, to_block_number)
//         .await?;

//     println!("Funding contract events: {:?}", events);
//     for event in &events {
//         match event {
//             FUNDING_CONTRACTEvents::ProjectListedFilter(_ev) => {}
//             FUNDING_CONTRACTEvents::DonationReceivedFilter(ev) => {
//                 service
//                     .article
//                     .donate_project(
//                         &ev.project_id.as_u64().to_string(),
//                         &format!("{:?}", ev.sender),
//                         ev.amount.as_u128(),
//                     )
//                     .await;
//             }
//             FUNDING_CONTRACTEvents::DeliveryApprovedFilter(_ev) => {}
//             FUNDING_CONTRACTEvents::MilestoneClosedFilter(ev) => {
//                 service
//                     .article
//                     .close_project_milestone(
//                         &ev.project_id.to_string(),
//                         ev.milestone_id.as_u64() as i16,
//                         ev.success,
//                     )
//                     .await
//                     .ok();
//             }
//             _ => {}
//         }
//     }

//     if let Some(last_block_number) = last_block_number {
//         service
//             .util
//             .upsert_last_block_number(&last_block_number.to_string())
//             .await?;
//     }

//     Ok(())
// }
