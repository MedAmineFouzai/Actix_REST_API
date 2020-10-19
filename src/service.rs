use std::fmt;
use std::error::Error;
use mongodb::bson::{Bson};
mod database;

#[derive(Debug)]
struct Team{
    id: String,
    name: String,
    wins: i32,
    draws: i32,
    losts: i32,
    goal_scored: i32,
    goal_acuired:i32,
    points: i32,
}
#[derive(Debug)]
struct  Competition {
    id: String,
    name: String
}
#[derive(Debug)]
struct Standing{
    position: i32,
    team:Team
}

#[derive(Debug)]
pub struct Classification{
    competition: Competition,
    standings: Vec<Standing>
}


impl  fmt::Display for Classification {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({:?},{:?})",self.competition,self.standings)
    }

}



impl  fmt::Display for Competition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})",self.id,self.name)
    }

}


impl fmt::Display for Standing {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})",self.position,self.team)
    }
}

impl fmt::Display  for Team{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{},{},{},{},{},{},{})",self.id,self.name,self.wins,self.draws,self.losts,self.goal_scored,self.goal_acuired,self.points)
    }

}

pub async fn classifications(query_id:&str)->  Result<Classification,Box<dyn Error>>{

let mut arrteams:Vec<Team>=Vec::new();
let mut  standings:Vec<Standing>=Vec::new();
let competition_id:&str;
let competition_name:&str;
let mut team_id:&str;
let mut team_name:&str;

let competition=database::get_competition(query_id);//the only parameter passed by get request to be handeled
let teams=database::get_teams();

match competition {
    Ok(competition)=>{
        
        let _id=&competition.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
        competition_id=_id;
        competition_name=competition.get("name").and_then(Bson::as_str).unwrap();
        println!("competation_id:{}",competition_id);
        println!("competition_name:{}",competition_name);
        match teams{
            Ok(teams)=>{
                for team in teams{
                    let mut wins: i32 =0;
                    let mut points: i32=0;
                    let mut losts: i32 =0;
                    let mut draws: i32 =0;
                    let mut goal_scored: i32=0;
                    let mut goal_acuired: i32=0;
                    
                    let _id= &team.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
                    team_id=_id;
                    team_name=team.get("name").and_then(Bson::as_str).unwrap();

                    let team_as_home_data=database::get_team_result(
                        competition_id,
                            _id,
                            "home",
                            "away"
                            ).await;
                    let team_as_away_data=database::get_team_result(

                        competition_id,
                        team_id,
                        "away",
                        "home"
                    ).await;


                    let team_as_home_status=database::team_status(
                        competition_id,
                        team_id,
                        "home",
                        "away"
                    ).await;


                    let team_as_away_status=database::team_status(
                        competition_id,
                        team_id,
                        "away",
                        "home"
                    ).await;


                        match team_as_home_data {
                            Ok(doc)=>{
                                goal_scored=goal_scored+ doc.get("goal_scored").and_then(Bson::as_i32).unwrap();
                                goal_acuired=goal_acuired+doc.get("goal_acuired").and_then(Bson::as_i32).unwrap();
                            
                            }
                            Err(e)=>{
                                println!("error:{}",e)
                            }
                        }
                        match team_as_away_data {
                            Ok(doc)=>{
                                
                                goal_scored=goal_scored+ doc.get("goal_scored").and_then(Bson::as_i32).unwrap();
                                goal_acuired=goal_acuired+doc.get("goal_acuired").and_then(Bson::as_i32).unwrap();
                            
                            }
                            Err(e)=>{
                                println!("error:{}",e)
                            }
                        }
                    
                        match team_as_home_status {
                            Ok(documents)=>{
                                for document in documents {
                                    if document.get("status").and_then(Bson::as_i32)==Some(1){
                                        wins=wins+1;
                                        points=points+3;
                                    }else if document.get("status").and_then(Bson::as_i32)==Some(0){
                                        draws=draws+1;
                                        points=points+1;
                                    }else if document.get("status").and_then(Bson::as_i32)==Some(-1){
                                        losts=losts+1;
                                    }
                                }
                            }
                            Err(e)=>{
                                println!("error :{}",e);
                            }
                        }

                        match team_as_away_status {
                            Ok(documents)=>{
                                for document in documents {
                                    if document.get("status").and_then(Bson::as_i32)==Some(1){
                                        wins=wins+1;
                                        points=points+3;
                                    }else if document.get("status").and_then(Bson::as_i32)==Some(0){
                                        draws=draws+1;
                                        points=points+1;
                                    }else if document.get("status").and_then(Bson::as_i32)==Some(-1){
                                        losts=losts+1;
                                    }
                                }
                            }
                            Err(e)=>{
                                println!("error :{}",e);
                            }
                        }
                        
                        arrteams.push(
                        
                            Team{
                                id:team_id.to_owned(),
                                name:team_name.to_owned(),
                                wins:wins,
                                draws:draws,
                                losts:losts,
                                goal_scored:goal_scored,
                                goal_acuired:goal_acuired,
                                points:points
                                }
                        
                        
                       
                    ); 


                }
            }
            Err(e)=>{
                println!("error :{}",e);
            }
        }
        
    
        arrteams.sort_by(|low,max| max.points.cmp(&low.points));
        let mut pos=1;
        for team in arrteams{
            standings.push(
                Standing{
                    position:pos,
                    team:team
                }
            );
            pos=pos+1;
        }
        let classified=Classification{
            competition:Competition{
                id:competition_id.to_owned(),
                name:competition_name.to_owned()
            },
            standings:standings
        };    
           
    Ok(classified)
    }   
    Err(e)=>Err(e.into())

}


}