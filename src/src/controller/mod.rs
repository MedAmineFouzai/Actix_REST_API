use actix_web::{web,get, HttpResponse,HttpRequest,Responder};
use bson::{doc, oid::ObjectId, ordered::OrderedDocument};
use bson::Bson;
use serde::Deserialize;
use std::fmt;


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





#[get("/{id}")]
pub async fn get(
    app_data: web::Data<crate::AppState>,
    id: HttpRequest
  ) -> impl Responder {
    //let team_result = web::block(move || app_data.service_container.team.get()).await;
    let result= web::block(move ||
   {
    let competition= app_data.service_container.competition.get("5f8b4839d42642b5f38d78db");
    let teams=app_data.service_container.team.get();

    let Some(myfinal)=match competition {
        Ok(mut competition) => {
            let competition=competition.next();
            match teams{
                Ok(mut teams)=>{
                 for team in teams{
                    let mut wins: i32 =0;
                    let mut points: i32=0;
                    let mut losts: i32 =0;
                    let mut draws: i32 =0;
                    let mut goal_scored: i32=0;
                    let mut goal_acuired: i32=0;
                    let team=team.unwrap();
                    let team_id= team.get("_id").and_then(Bson::as_object_id).unwrap().to_string();
                    let team_name= team.get("name").and_then(Bson::as_str).unwrap();
                    println!("team_name :{},team_id:{}",team_name,team_id);

                 }

                }

                Err(e) =>{
                  println!("error :{}",e);
                }
           
            }//team match arm 
            
            
            competition
            
        }

        Err(e) => {
          println!("error :{:?}",e);
        }
    };//competition match arm 

    Ok("hello".to_string())


    }
).await;
    
    
     


    match result {
      Ok(mut result) => {
        let doc =result;
        println!("doc :{:?}",doc);
        
        HttpResponse::Ok().json(doc)
      
      }
      
      Err(e) => {
        println!("Error while getting, {:?}", e);
        HttpResponse::InternalServerError().finish()
      }
    }
  }
  