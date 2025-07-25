use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketCreate};
use crate::Result;

async fn create_ticket(
    ctx: Ctx,
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(ctx: Ctx, State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_ticket(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    ctx: Ctx,
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}

// #[derive(Clone, Debug, FromRef)]
// struct AppState {
//     mc: ModelController,
// }

pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", delete(delete_ticket))
        .with_state(mc)
}
