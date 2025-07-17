use std::sync::Arc;

use crate::{DatabasePool, NotificationRepository};
use serde_json::json;
use types::{
    error::{ApiError, DbError},
    models::{CreateNotification, Notification, NotificationResponse, NotificationType},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct NotificationService {
    repository: NotificationRepository,
}

impl NotificationService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            repository: NotificationRepository::new(db_conn),
        }
    }

    pub async fn create_notification(
        &self,
        notification: CreateNotification,
    ) -> Result<Notification, ApiError> {
        let notification = self
            .repository
            .create_notification(notification.clone())
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        Ok(notification)
    }

    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<NotificationResponse>, ApiError> {
        let notifications = self
            .repository
            .get_user_notifications(user_id, limit, offset)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;
        Ok(notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect())
    }

    pub async fn get_unread_notifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NotificationResponse>, ApiError> {
        let notifications = self
            .repository
            .get_unread_notifications(user_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;
        Ok(notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect())
    }

    pub async fn mark_notification_as_read(
        &self,
        notification_id: i64,
        user_id: Uuid,
    ) -> Result<(), ApiError> {
        Ok(self
            .repository
            .mark_notification_as_read(notification_id, user_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }

    pub async fn mark_all_notifications_as_read(&self, user_id: Uuid) -> Result<(), ApiError> {
        Ok(self
            .repository
            .mark_all_notifications_as_read(user_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }

    pub async fn delete_notification(
        &self,
        notification_id: i64,
        user_id: Uuid,
    ) -> Result<(), ApiError> {
        Ok(self
            .repository
            .delete_notification(notification_id, user_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }

    pub async fn get_notification_count(&self, user_id: Uuid) -> Result<i64, ApiError> {
        Ok(self
            .repository
            .get_notification_count(user_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }

    pub async fn get_unread_count(&self, user_id: Uuid) -> Result<i64, ApiError> {
        Ok(self
            .repository
            .get_unread_count(user_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }

    // Helper methods for creating specific types of notifications
    pub async fn notify_editor_invitation(
        &self,
        user_id: Uuid,
        project_name: &str,
        admin_name: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id,
            notification_type: NotificationType::InviteEditor,
            title: "Editor Invitation".to_string(),
            message: format!(
                "{} has invited you to be an editor for project '{}'",
                admin_name, project_name
            ),
            data: Some(json!({
                "project_name": project_name,
                "admin_name": admin_name,
                "action": "invite_editor"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_editor_declined(
        &self,
        admin_id: Uuid,
        user_name: &str,
        project_name: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id: admin_id,
            notification_type: NotificationType::DeclineEditor,
            title: "Editor Invitation Declined".to_string(),
            message: format!(
                "{} has declined the editor invitation for project '{}'",
                user_name, project_name
            ),
            data: Some(json!({
                "user_name": user_name,
                "project_name": project_name,
                "action": "decline_editor"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_editor_accepted(
        &self,
        admin_id: Uuid,
        user_name: &str,
        project_name: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id: admin_id,
            notification_type: NotificationType::AcceptEditor,
            title: "Editor Invitation Accepted".to_string(),
            message: format!(
                "{} has accepted the editor invitation for project '{}'",
                user_name, project_name
            ),
            data: Some(json!({
                "user_name": user_name,
                "project_name": project_name,
                "action": "accept_editor"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_new_bounty(
        &self,
        user_ids: Vec<Uuid>,
        bounty_title: &str,
        creator_name: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::NewBounty,
                title: "New Bounty Available".to_string(),
                message: format!(
                    "{} has created a new bounty: '{}'",
                    creator_name, bounty_title
                ),
                data: Some(json!({
                    "bounty_title": bounty_title,
                    "creator_name": creator_name,
                    "action": "new_bounty"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_new_dao(
        &self,
        user_ids: Vec<Uuid>,
        dao_name: &str,
        creator_name: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::NewDAO,
                title: "New DAO Created".to_string(),
                message: format!("{} has created a new DAO: '{}'", creator_name, dao_name),
                data: Some(json!({
                    "dao_name": dao_name,
                    "creator_name": creator_name,
                    "action": "new_dao"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_new_prediction(
        &self,
        user_ids: Vec<Uuid>,
        prediction_title: &str,
        creator_name: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::NewPrediction,
                title: "New Prediction Market".to_string(),
                message: format!(
                    "{} has created a new prediction: '{}'",
                    creator_name, prediction_title
                ),
                data: Some(json!({
                    "prediction_title": prediction_title,
                    "creator_name": creator_name,
                    "action": "new_prediction"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_bid_approved(
        &self,
        user_id: Uuid,
        bounty_title: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id,
            notification_type: NotificationType::ApprovedBid,
            title: "Bid Approved".to_string(),
            message: format!("Your bid for '{}' has been approved!", bounty_title),
            data: Some(json!({
                "bounty_title": bounty_title,
                "action": "bid_approved"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_bid_rejected(
        &self,
        user_id: Uuid,
        bounty_title: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id,
            notification_type: NotificationType::RejectedBid,
            title: "Bid Rejected".to_string(),
            message: format!("Your bid for '{}' has been rejected", bounty_title),
            data: Some(json!({
                "bounty_title": bounty_title,
                "action": "bid_rejected"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_bid_reviewed(
        &self,
        user_id: Uuid,
        bounty_title: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id,
            notification_type: NotificationType::BidReviewed,
            title: "Bid Under Review".to_string(),
            message: format!("Your bid for '{}' is currently under review", bounty_title),
            data: Some(json!({
                "bounty_title": bounty_title,
                "action": "bid_reviewed"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_new_project(
        &self,
        user_ids: Vec<Uuid>,
        project_name: &str,
        creator_name: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::NewProject,
                title: "New Project Created".to_string(),
                message: format!(
                    "{} has created a new project: '{}'",
                    creator_name, project_name
                ),
                data: Some(json!({
                    "project_name": project_name,
                    "creator_name": creator_name,
                    "action": "new_project"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_project_milestone(
        &self,
        user_ids: Vec<Uuid>,
        project_name: &str,
        milestone_name: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::ProjectMilestone,
                title: "Project Milestone Update".to_string(),
                message: format!(
                    "Milestone '{}' has been completed for project '{}'",
                    milestone_name, project_name
                ),
                data: Some(json!({
                    "project_name": project_name,
                    "milestone_name": milestone_name,
                    "action": "project_milestone"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_project_comment(
        &self,
        user_id: Uuid,
        project_name: &str,
        commenter_name: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id,
            notification_type: NotificationType::ProjectComment,
            title: "New Project Comment".to_string(),
            message: format!("{} commented on project '{}'", commenter_name, project_name),
            data: Some(json!({
                "project_name": project_name,
                "commenter_name": commenter_name,
                "action": "project_comment"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_bounty_comment(
        &self,
        user_id: Uuid,
        bounty_title: &str,
        commenter_name: &str,
    ) -> Result<(), ApiError> {
        let notification = CreateNotification {
            user_id,
            notification_type: NotificationType::BountyComment,
            title: "New Bounty Comment".to_string(),
            message: format!("{} commented on bounty '{}'", commenter_name, bounty_title),
            data: Some(json!({
                "bounty_title": bounty_title,
                "commenter_name": commenter_name,
                "action": "bounty_comment"
            })),
        };
        self.create_notification(notification).await?;
        Ok(())
    }

    pub async fn notify_dao_vote(
        &self,
        user_ids: Vec<Uuid>,
        dao_name: &str,
        proposal_title: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::DAOVote,
                title: "DAO Vote Required".to_string(),
                message: format!(
                    "A new proposal '{}' requires your vote in DAO '{}'",
                    proposal_title, dao_name
                ),
                data: Some(json!({
                    "dao_name": dao_name,
                    "proposal_title": proposal_title,
                    "action": "dao_vote"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_funding_update(
        &self,
        user_ids: Vec<Uuid>,
        project_name: &str,
        funding_amount: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::FundingUpdate,
                title: "Funding Update".to_string(),
                message: format!(
                    "Project '{}' has received {} in funding",
                    project_name, funding_amount
                ),
                data: Some(json!({
                    "project_name": project_name,
                    "funding_amount": funding_amount,
                    "action": "funding_update"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_prediction_result(
        &self,
        user_ids: Vec<Uuid>,
        prediction_title: &str,
        result: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::PredictionResult,
                title: "Prediction Result".to_string(),
                message: format!("Prediction '{}' has resolved: {}", prediction_title, result),
                data: Some(json!({
                    "prediction_title": prediction_title,
                    "result": result,
                    "action": "prediction_result"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    pub async fn notify_system_message(
        &self,
        user_ids: Vec<Uuid>,
        title: &str,
        message: &str,
    ) -> Result<(), ApiError> {
        for user_id in user_ids {
            let notification = CreateNotification {
                user_id,
                notification_type: NotificationType::SystemMessage,
                title: title.to_string(),
                message: message.to_string(),
                data: Some(json!({
                    "action": "system_message"
                })),
            };
            self.create_notification(notification).await?;
        }
        Ok(())
    }

    // Methods for the WebSocket reader
    pub async fn get_latest_notification_index(&self) -> Result<i64, ApiError> {
        Ok(self
            .repository
            .get_latest_notification_index()
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }

    pub async fn get_notifications_from_index(
        &self,
        from_index: i64,
        limit: i64,
    ) -> Result<Vec<Notification>, ApiError> {
        Ok(self
            .repository
            .get_notifications_from_index(from_index, limit)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?)
    }
}
