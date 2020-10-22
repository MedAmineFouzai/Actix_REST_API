use actix_web::{web,get, HttpResponse,Responder};
use bson::{doc};
use bson::Bson;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize, Deserialize)]
struct TeamState{
    id: String,
    name: String,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
struct Teams{
    teams:Vec<TeamState>
}

#[derive(Debug,Serialize, Deserialize)]
struct  Competition {
    id: String,
    name: String
}
#[derive(Debug,Serialize, Deserialize)]
struct Team{
    team_name:String,
    wins: i32,
    draws : i32,
    losts: i32,
    goal_scored: i32,
    goal_acuired: i32,
    points: i32
}
#[derive(Debug,Serialize, Deserialize)]
struct Position {
    position: i32,
    team:Team
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Classification{
    competition: Competition,
    standings: Vec<Position>
}

impl Competition {

    pub fn new(id:String,name:String)->Competition {
        Competition{id,name}
    }
    
    pub fn get_id(&self)->&String{
         &self.id
    }

    pub fn get_name(&self)->&String{
        &self.name
    }

}

impl TeamState {

    pub fn new(id:String,name:String)->TeamState {
        TeamState{id,name}
    }
    pub fn get_id(&self)->&String{
        &self.id
   }

   pub fn get_name(&self)->&String{
       &self.name
   }

}


impl Teams {

    pub fn new(teams:Vec<TeamState>)->Teams {
        Teams{
            teams
        }
    }
}



#[get("/competition/{id}/standings")]
pub async fn get(
    app_data: web::Data<crate::AppState>,
    id: web::Path<String>
  ) -> impl Responder {
    
   
    let mut competition_state=Competition::new("".to_string(), "".to_string());
    let mut teams_state:Teams=Teams::new(vec![TeamState::new("".to_string(), "".to_string())]);
    let competition= app_data.service_container.competition.get(id.into_inner().as_str());
    let teams=app_data.service_container.team.get();
    match  competition {
        Ok(mut competition)=>{
            while let Some( cur)=competition.next(){
                match  cur {
                    Ok( cur)=>{
                   
                            let id=&cur.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
                            competition_state=Competition::new(
                                id.to_string(),
                                cur.get("name").and_then(Bson::as_str).unwrap().to_string()
                            );

                    }
                    Err(e) => {println!("error:{}",e)}
                };   
            }
        }
        Err(e) => {println!("error:{}",e)}
    };

    match  teams {
        Ok(mut cur) =>{
            while  let Some(cur)=cur.next() {
                match cur {
                    Ok(cur)=>{
                        let id= &cur.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
                        teams_state.teams.push(
                            TeamState::new(
                            id.to_string(),
                          cur.get("name").and_then(Bson::as_str).unwrap().to_string()
                        ))
                    }
               Err(e) => {println!("error:{}",e)}
           }
         }
       }  
          Err(e) => {println!("error:{}",e)}    
    };

        //out of scope begin if return functions
    let mut standings:Vec<Team>=Vec::new();
    for team in 1..5{

          let mut wins: i32 =0;
          let mut points: i32=0;
          let mut losts: i32 =0;
          let mut draws: i32 =0;
          let mut goal_scored: i32=0;
          let mut goal_acuired: i32=0;

         let home_team_result=app_data.service_container.matches.get_team_result(
     competition_state.get_id(),
            teams_state.teams[team].get_id(),
           "home",
            "away");

          let away_team_result=app_data.service_container.matches.get_team_result(
         competition_state.get_id(),
                teams_state.teams[team].get_id(),
               "away",
                "home");

          let home_team_status=app_data.service_container.matches.get_team_status(
        competition_state.get_id(),
               teams_state.teams[team].get_id(),
              "home",
                "away");

        let away_team_status=app_data.service_container.matches.get_team_status(
         competition_state.get_id(),
                teams_state.teams[team].get_id(),
               "away",
                "home");

        match  home_team_result {
            Ok(mut cur) =>{
                while  let Some(cur)=cur.next() {
                    match cur {
                        Ok(cur)=>{
                            goal_scored=goal_scored+cur.get("goal_scored").and_then(Bson::as_i32).unwrap();
                            goal_acuired=goal_acuired+cur.get("goal_acuired").and_then(Bson::as_i32).unwrap();
                            }
                       Err(e) => {println!("error:{}",e)}
                    }
                  }
                }  
                  Err(e) => {println!("error:{}",e)}    
            };

        match  away_team_result {
            Ok(mut cur) =>{
                while  let Some(cur)=cur.next() {
                    match cur {
                        Ok(cur)=>{
                            goal_scored=goal_scored+cur.get("goal_scored").and_then(Bson::as_i32).unwrap();
                            goal_acuired=goal_acuired+cur.get("goal_acuired").and_then(Bson::as_i32).unwrap();
                        }
                       Err(e) => {println!("error:{}",e)}
                   }
                 }
               }  
                  Err(e) => {println!("error:{}",e)}    
            };

        match  home_team_status {
            Ok(mut cur) =>{
                while  let Some(cur)=cur.next() {
                    match cur {
                       Ok(cur)=>{
                            if cur.get("status").and_then(Bson::as_i32)==Some(1){
                                wins=wins+1;
                                points=points+3;
                            }else if cur.get("status").and_then(Bson::as_i32)==Some(0){
                                draws=draws+1;
                                points=points+1;
                            }else if cur.get("status").and_then(Bson::as_i32)==Some(-1){
                                losts=losts+1;
                            }
                        }
                       Err(e) => {println!("error:{}",e)}
                   }
                 }
               }  
                  Err(e) => {println!("error:{}",e)}    
            };

        match  away_team_status {
            Ok(mut cur) =>{
                while  let Some(cur)=cur.next() {
                    match cur {
                        Ok(cur)=>{
                            if cur.get("status").and_then(Bson::as_i32)==Some(1){
                                wins=wins+1;
                                points=points+3;
                            }else if cur.get("status").and_then(Bson::as_i32)==Some(0){
                                draws=draws+1;
                                points=points+1;
                            }else if cur.get("status").and_then(Bson::as_i32)==Some(-1){
                                losts=losts+1;
                            }
                        }
                       Err(e) => {println!("error:{}",e)}
                   }
                 }
               }  
                  Err(e) => {println!("error:{}",e)}    
            };

        standings.push(
                Team{
                    team_name:teams_state.teams[team].get_name().to_string(),
                    wins:wins,
                    draws:draws,
                    losts:losts,
                    goal_scored:goal_scored,
                    goal_acuired:goal_acuired,
                    points:points
                }
            );
      }
      let mut positions:Vec<Position>=Vec::new(); 
      let classification:Classification;
      let mut pos=1;
      standings.sort_by(|low,max| max.points.cmp(&low.points));
      for standing in standings{
        positions.push(
            Position{
                position:pos as i32,
                team:standing
            }
        );
        pos=pos+1;
      }

      classification=Classification{
          competition:Competition{
              id:competition_state.get_id().to_string(),
              name:competition_state.get_name().to_string()
          },
          standings:positions
      };

      HttpResponse::Ok().json(classification)

  }
  