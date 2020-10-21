use actix_web::{web,get, HttpResponse,Responder};
use bson::{doc};
use bson::Bson;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug,Clone,Serialize, Deserialize)]
struct Team{
    id: String,
    name: String,
}
#[derive(Debug,Clone,Serialize, Deserialize)]
struct Teams{
    teams:Vec<Team>
}
#[derive(Debug,Serialize, Deserialize)]
struct  Competition {
    id: String,
    name: String
}
#[derive(Debug,Serialize, Deserialize)]
struct Standing{
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
    standing:Standing
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

impl Team {

    pub fn new(id:String,name:String)->Team {
        Team{id,name}
    }
    pub fn get_id(&self)->&String{
        &self.id
   }

   pub fn get_name(&self)->&String{
       &self.name
   }

}

impl Teams {

    pub fn new(teams:Vec<Team>)->Teams {
        Teams{
            teams
        }
    }
}







#[get("/{id}")]
pub async fn get(
    app_data: web::Data<crate::AppState>,
    id: web::Path<String>
  ) -> impl Responder {
    
   
    let mut mycompetition=Competition::new(
        "".to_string(),
        "".to_string()
    );
    let mut myteams:Teams=Teams::new(vec![
        Team::new("".to_string(), "".to_string())
    ]);
    
    let competition= app_data.service_container.competition.get(id.into_inner().as_str());
    let teams=app_data.service_container.team.get();

    match  competition {

        Ok(mut competition)=>{
            while let Some( cur)=competition.next(){
                match  cur {
                    Ok( cur)=>{
                   
                            let id=&cur.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
                            // mycompetition.id=id.to_string();
                            // mycompetition.name=cur.get("name").and_then(Bson::as_str).unwrap().to_string();
                             mycompetition=Competition::new(
                                id.to_string(),
                                cur.get("name").and_then(Bson::as_str).unwrap().to_string()
                            );
                            // competition_id=id;
                            // competition_name=cur.get("name").and_then(Bson::as_str).unwrap().to_string();

                        
                    }
                    Err(e) => {
                        println!("error:{}",e)
        
                    }
        
                };   
    
            }

        }
        Err(e) => {
            println!("error:{}",e)
        }
      
    };

    match  teams {
        Ok(mut cur) =>{
         while  let Some(cur)=cur.next() {
           match cur {
               Ok(cur)=>{
                   let id= &cur.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
                   myteams.teams.push(
                    Team::new(
                        id.to_string(),
                        cur.get("name").and_then(Bson::as_str).unwrap().to_string()
                    ) )
                }
               Err(e) => {println!("error:{}",e)}
           }
         }
       }  
          Err(e) => {println!("error:{}",e)}    
    };

        //out of scope begin if return functions
       let mut standings:Vec<Standing>=Vec::new();
       
    
      //println!("team from scope :{:?}",myteams);
      for team in 1..5{
          let mut wins: i32 =0;
          let mut points: i32=0;
          let mut losts: i32 =0;
          let mut draws: i32 =0;
          let mut goal_scored: i32=0;
          let mut goal_acuired: i32=0;
        //   println!("team :{}",myteams.teams[team]);
          let home_team_result=app_data.service_container.matches.get_team_result(
            mycompetition.get_id(),
            myteams.teams[team].get_id(),
            "home",
            "away");
          let away_team_result=app_data.service_container.matches.get_team_result(
                mycompetition.get_id(),
                myteams.teams[team].get_id(),
                "away",
                "home");
          let home_team_status=app_data.service_container.matches.get_team_status(
                    mycompetition.get_id(),
                    myteams.teams[team].get_id(),
                    "home",
                    "away");
        let away_team_status=app_data.service_container.matches.get_team_status(
                        mycompetition.get_id(),
                        myteams.teams[team].get_id(),
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
                Standing{
                    team_name:myteams.teams[team].get_name().to_string(),
                    wins:wins,
                    draws:draws,
                    losts:losts,
                    goal_scored:goal_scored,
                    goal_acuired:goal_acuired,
                    points:points
                }
            );


         //   println!("team :{}, goal_scored :{},goal_acuired:{} ,wins :{}, draws :{} ,losts :{},points :{}",myteams.teams[team].get_name(),goal_scored,goal_acuired,wins,draws,losts,points);
      }
      let mut positions:Vec<Position>=Vec::new();
       
      let classification:Classification;
      //println!("competition from out of scope:{} ,{}",mycompetition.get_id(),mycompetition.get_name());
      standings.sort_by(|low,max| max.points.cmp(&low.points));
      let mut pos=1;
      for standing in standings{
        positions.push(
            Position{
                position:pos as i32,
                standing:standing
            }
        );
        pos=pos+1;
      }
      classification=Classification{
          competition:Competition{
              id:mycompetition.get_id().to_string(),
              name:mycompetition.get_name().to_string()
          },
          standings:positions
      };
      //println!("classsfication:{:?}",classification);
      let j = serde_json::to_string(&classification);

      HttpResponse::Ok().json(json!(j.ok()))

  }
  