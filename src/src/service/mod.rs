use bson::{doc, oid::ObjectId};
use mongodb::{error::Error, Collection,Cursor};
#[derive(Clone)]
pub struct TeamService {
    collection:Collection,
}
#[derive(Clone)]
pub struct CompetitionService {
    collection:Collection,
}


#[derive(Clone)]
pub struct MatchesService {
    collection:Collection,
}



impl TeamService {

    pub fn new(collection: Collection)->TeamService {
        TeamService{ collection }
    }

    pub fn get(&self)->Result<Cursor,Error>{
        self.collection.find(doc!{},None)
    }

}

impl CompetitionService {

    pub fn new(collection: Collection)->CompetitionService {
        CompetitionService{ collection }
    }

    pub fn get(&self,id:&str)->Result<Cursor,Error>{
        self.collection.find(doc!{
            "_id":ObjectId::with_string(id).unwrap()
        },None)

}

}

impl MatchesService {

    pub fn new(collection: Collection)->MatchesService {
        MatchesService{ collection }
    }

    pub fn get_team_result(&self,
        competition_id:&str,
        team_id:&str,
        gameplay:&str,
        counter:&str
    )->Result<Cursor,Error>{

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
        self.collection.aggregate(vec![
            match_,
            group_], None)
    }



    pub fn get_team_status(&self,
        competition_id:&str,
        team_id:&str,
        gameplay:&str,
        counter:&str
    )->Result<Cursor,Error>{

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
        self.collection.aggregate(vec![
            match_,
            project_], None)
    }




}