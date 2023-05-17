use rocket::tokio::sync::broadcast::Sender;
use rocket::{get, http::Status, post, serde::json::Json, serde::uuid::Uuid, State};
use std::collections::HashMap;
use std::sync::RwLock;

use super::robot::{TEST_API_KEY};
use crate::db::models::{MovementData, Command};
use crate::db::mongodb::MongoRepo;

#[get("/register")]
pub async fn register_robot() -> String {
    //! Generate an API key for a new robot
    Uuid::new_v4().to_string()

    // FIXME: Save the key in the database.
}

#[get("/data")]
pub async fn get_all_data() {
    //! Optional, might not be required
    //! Get all data for all robots.
}

#[get("/data/position/<robot_id>")]
pub fn get_position(db: &State<MongoRepo>, robot_id: Uuid) -> Result<Json<Vec<MovementData>>, Status> {
    let bson_uuid = bson::Uuid::from_uuid_1(robot_id);

    match db.get_robot_position(bson_uuid) {
        Ok(robot_data) => Ok(Json(robot_data)),
        Err(_) => Err(Status::InternalServerError), // Logging maybe?
    }
}

#[get("/data/history/<robot_id>")]
pub async fn get_history(robot_id: Uuid) {
    //! Get event history for a robot.
}

#[get("/command/hello")]
pub async fn hello_test(
    active_queues: &State<RwLock<HashMap<Uuid, Sender<Command>>>>,
) -> Option<()> {
    //! Test endpoint for testing that the frontend can reach the server.
    //! The endpoint sends a hello command to the robot.
    let _res = active_queues
        .read()
        .unwrap()
        // TODO: Get the robot id given in the request instead
        .get(&TEST_API_KEY)?
        .send(Command::Hello);
    Some(()) //FIXME: This could be better when the implementation is completed.
}

#[get("/command/move/<robot_id>?<drive_speed>&<rotation_speed>")]
pub async fn move_robot(robot_id: Uuid, drive_speed: f32, rotation_speed: f32) {
    //! Move a specified robot forward or backward in a direction.
    //! It is also possible to only rotate the robot or only backward/forward.
    //! Rotation and drive speed can be negative.
}

// TODO: #[get("/command/patrol/<robot_id>/<patrol_id>")]
// pub async fn start_patrol(robot_id: Uuid, patrol_id: usize) {
#[get("/command/patrol/<robot_id>")]
pub async fn start_patrol(
    active_queues: &State<RwLock<HashMap<Uuid, Sender<Command>>>>,
    robot_id: i32,
) -> Option<()> {
    //! Endpoint that will tell the robot to start patrolling a specified path.
    let _res = active_queues
        .read()
        .unwrap()
        // TODO: Get the robot id given in the request instead
        .get(&TEST_API_KEY)?
        .send(Command::Patrol(1));
    Some(()) // FIXME: Handle errors better
}

#[post("/command/patrol/<robot_id>")]
pub async fn add_patrol_route(robot_id: Uuid) {
    //! Endpoint to add patrol routes

    // Send list of coordinates which makes up a path between two points (in body).
}
