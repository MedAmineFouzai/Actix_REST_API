use actix_web::{App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use service::{TeamService,CompetitionService,MatchesService};
mod controller;
mod service;

pub struct ServiceContainer {
    competition:CompetitionService,
    team: TeamService,
    matches:MatchesService
  
}

impl ServiceContainer {
  pub fn new(competition:CompetitionService,team: TeamService,matches:MatchesService) -> Self {
    ServiceContainer {competition,team ,matches}
  }
}

pub struct AppState{
    service_container:ServiceContainer,

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  const DB:&str="Tournaments";
  let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
  let client = Client::with_options(client_options).unwrap();
  let db = client.database(DB);

  let teams_collection = db.collection("Teams");
  let competitions_collection=db.collection("Competitions");
  let matches_collection=db.collection("Matches");

  HttpServer::new(move || {
    let service_container = ServiceContainer::new(
        CompetitionService::new(competitions_collection.clone()),
        TeamService::new(teams_collection.clone()),
        MatchesService::new(matches_collection.clone())
    );

    App::new()
      .data(AppState { service_container })
      .service(controller::get)
    })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
