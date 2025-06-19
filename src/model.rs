//! Simplistic Model Layer
//! (With mock-store layer)
//!

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{ctx::Ctx, ApiError, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ticket {
    pub id: u64,
    // Creator User Id
    pub c_id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TicketCreate {
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, // Only for local prototype
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self { tickets_store: Arc::default() })
    }

    // CRUD Implementation

    /// Create Implementation
    pub async fn create_ticket(&self, ctx: Ctx, ticket: TicketCreate) -> Result<Ticket> {
        let mut store = self
            .tickets_store
            .lock()
            .map_err(|_| ApiError::InternalServerError("Problem accessing store.".to_string()))?;
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            c_id: ctx.user_id(),
            title: ticket.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    /// Get implementation
    pub async fn list_ticket(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().map_err(|_| {
            ApiError::InternalServerError("Error accessing ticket store.".to_string())
        })?;
        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    /// Delete implementation
    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().map_err(|_| {
            ApiError::InternalServerError("Error accessing ticket store".to_string())
        })?;
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(ApiError::DeleteFailedIdNotFound { id: id.to_string() })
    }
}
