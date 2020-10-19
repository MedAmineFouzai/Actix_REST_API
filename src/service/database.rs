use mongodb::{bson::{Document, doc, oid::ObjectId}};
use std::error::Error;
use tokio::{self, stream::StreamExt};

const URI:&str="mongodb://localhost:27017";
const DATABASE:&str="Tournaments";
const COMPETITIONS:&str="Competitions";
const  TEAMS:&str="Teams";
const MATCHES:&str="Matches";

#[tokio::main]
pub async fn get_competition(id:&str) -> Result<Document, Box<dyn Error>> {
   let client = mongodb::Client::with_uri_str(URI).await?;
   let db = client.database(DATABASE);
   let collection=db.collection(COMPETITIONS);
   let mut cursor = collection.find(doc!{
       "_id":ObjectId::with_string(id).unwrap()
   }, None).await?;
   let  mut doc:Document=Document::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                
                  doc= document;
                  break;
                 }
                
            Err(e) => return Err(e.into())
            
        }
    };

Ok(doc)
   }


#[tokio::main]
pub async fn get_teams() -> Result<Vec<Document>, Box<dyn Error>> {
           
           let client = mongodb::Client::with_uri_str(URI).await?;
           let db = client.database(DATABASE);
           let collection=db.collection(TEAMS);
           let mut cursor = collection.find(None, None).await?;
           let  mut docs:Vec<Document>=[].to_vec();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                    
                        docs.push(
                            document
                        )
                         }
                        
                    Err(e) => return Err(e.into())
                    
                }
            };
        
        Ok(docs)
   }
   






pub async fn get_team_result(
    competition_id:&str,
    team_id:&str,
    gameplay:&str,
    counter:&str)-> Result<Document, Box<dyn Error>> {

        let client = mongodb::Client::with_uri_str(URI).await?;
        let db = client.database(DATABASE);
        let collection=db.collection(MATCHES);
        let match_ =doc!{
            "$match": {
                "competition":ObjectId::with_string(competition_id).unwrap()
            },
            "$match":{
                format!("teams.{}",gameplay) :ObjectId::with_string(team_id).unwrap()
            },
            };
        let group_=doc!{
            "$group":{
                "_id":ObjectId::with_string(team_id).unwrap() ,
                "goal_scored": {
                    "$sum": format!("$scores.{}",gameplay)
                },
                "goal_acuired":{
                    "$sum": format!("$scores.{}",counter)
                }
            }
        };
        let mut cursor =  collection.aggregate(vec![
            match_,
            group_],  None).await?;
        let  mut doc:Document=Document::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    doc=document;
                    break;
                    }
               Err(e) => return Err(e.into())
           }
        };
   
   Ok(doc)
      }
    
pub async fn team_status(
    competition_id:&str,
    team_id:&str,
    gameplay:&str,
    counter:&str) -> Result<Vec<Document>, Box<dyn Error>> {
        
        let client = mongodb::Client::with_uri_str(URI).await?;
        let db = client.database(DATABASE);
        let collection=db.collection(MATCHES);
        let match_ =doc!{
        "$match": {
         "competition":ObjectId::with_string(competition_id).unwrap()
        },
        "$match":{
          format!("teams.{}",gameplay) :ObjectId::with_string(team_id).unwrap()
        },
       
    };
    let project_=doc! {
        "$project":{
            "status":{
                "$cmp":[ format!("$scores.{}",gameplay),format!("$scores.{}",counter)]
            }
        }
    };
        let mut cursor =  collection.aggregate(vec![
            match_,
            project_
            ],  None).await?;
        
        let  mut docs:Vec<Document>=[].to_vec();
         while let Some(result) = cursor.next().await {
             match result {
                 Ok(document) => {
                 
                     docs.push(
                         document
                     )
                      }
                     
                 Err(e) => return Err(e.into())
                 
             }
         };
     
     Ok(docs)
}

      

