use crate::{
    error::AppError,
    features::contact_info::{
        data_transfer_objects::UpdateContactInfoRequest, model::ContactInfoRow,
    },
};

use super::repository::DynamicContactInfoRepository;

#[derive(Clone)]
pub struct ContactInfoService {
    repository: DynamicContactInfoRepository,
}

impl ContactInfoService {
    pub fn new(repository: DynamicContactInfoRepository) -> Self {
        Self { repository }
    }

    pub async fn get(&self) -> Result<ContactInfoRow, AppError> {
        self.repository
            .get()
            .await?
            .ok_or(AppError::NotFound("Contact info not found"))
    }

    pub async fn update(
        &self,
        request: UpdateContactInfoRequest,
    ) -> Result<ContactInfoRow, AppError> {
        // Fetch current (if any) to support partial update. If missing, start with blanks.
        let current = self.repository.get().await?;
        let (mut address, mut phone, mut email) = if let Some(row) = current {
            (row.address, row.phone, row.email)
        } else {
            (String::new(), String::new(), String::new())
        };

        if let Some(v) = request.address {
            address = v;
        }
        if let Some(v) = request.phone {
            phone = v;
        }
        if let Some(v) = request.email {
            email = v;
        }

        // All fields must be non-empty due to NOT NULL constraints.
        if address.is_empty() || phone.is_empty() || email.is_empty() {
            return Err(AppError::BadRequest(
                "address, phone, and email must all be provided at least once",
            ));
        }

        self.repository.set(&address, &phone, &email).await?;
        // Return fresh row (also gives updated_at)
        self.get().await
    }
}
