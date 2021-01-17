use async_graphql::*;
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::{collections::HashMap, default::Default};
use std::{env, num::ParseIntError};

#[derive(Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
    // #[graphql(skip)]
    //  pub members: Vec<Member>,
}

#[Object]
impl Team {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn members(&self) -> Result<Vec<Member>> {
        let pool = connect_db().await?;
        let members = sqlx::query_as!(
            Member,
            "SELECT m.id, m.name, m.knockouts, m.team_id FROM teams as t JOIN members as m ON m.team_id = t.id WHERE t.id=$1", self.id
        )
        .fetch_all(&pool)
        .await?;
        println!("membersOnTeamObject{:?}", members);
        Ok(members)
    }

}

// no traits are needed
#[derive(Debug)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

#[Object]
impl Member {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> String {
        self.name.to_owned()
    }
    async fn knockouts(&self) -> i32 {
        self.knockouts
    }
    async fn team_id(&self) -> i32 {
        self.team_id
    }

    async fn team(&self) -> Result<Team> {
        let pool = connect_db().await?;
        let team = sqlx::query_as!(
            Team,
            "SELECT t.id, t.name FROM teams as t JOIN members as m ON m.team_id = t.id WHERE m.id=$1", self.id
        )
        .fetch_one(&pool)
        .await?;
        println!("teamOnMemberObject{:?}", team);
        Ok(team)
    }
}

#[derive(Default)]
pub struct MemberQuery;

#[Object]
impl MemberQuery {
    async fn member(&self, id: i32) -> Result<Option<Member>> {
        let pool = connect_db().await?;
        // let members = sqlx::query_as!( "SELECT M.id, M.name, knockouts, team_id FROM MEMBERS AS M JOIN TEAMS AS T ON M.team_id = T.id")
        let members = sqlx::query_as!(
            Member,
            "SELECT id, name, knockouts, team_id FROM MEMBERS AS M WHERE id=$1",
            id
        )
        .fetch_optional(&pool)
        .await?;
        println!("memberQuery{:?}", members);
        Ok(members)
    }

    async fn members(&self) -> Result<Option<Vec<Member>>> {
        let pool = connect_db().await?;
        // let members = sqlx::query_as!( "SELECT M.id, M.name, knockouts, team_id FROM MEMBERS AS M JOIN TEAMS AS T ON M.team_id = T.id")
        let members = sqlx::query_as!(Member, "SELECT id, name, knockouts, team_id FROM MEMBERS")
            .fetch_all(&pool)
            .await?;
        println!("memberSQuery{:?}", members);
        Ok(Some(members))
    }
}

#[derive(MergedObject, Default)]
pub struct Query(MemberQuery);

#[warn(private_in_public)]
pub type SchemaGraphQL = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> SchemaGraphQL {
    Schema::new(Query::default(), EmptyMutation, EmptySubscription)
}

pub async fn connect_db() -> Result<Pool<Postgres>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DB URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
