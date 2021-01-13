use async_graphql::*;
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::default::Default;
use std::{env, num::ParseIntError};

#[derive(SimpleObject)]
struct MyObject {
    a: i32,
    b: i32,

    #[graphql(skip)]
    c: i32,
}

struct MyObject2 {
    value: i32,
}

#[Object]
impl MyObject2 {
    async fn value(&self) -> String {
        self.value.to_string()
    }

    async fn value_from_db(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of object")] id: i64,
    ) -> Result<String> {
        // let conn = ctx.data::<DbPool>()?.take();
        Ok("conn.query_something(id)?.name".to_string())
    }
}

#[derive(Default)]
pub struct MathQuery;

#[Object]
impl MathQuery {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /*     async fn parse_with_extensions(&self, input: String) -> Result<i32> {
        Ok("234a"
            .parse()
            .map_err(|err: ParseIntError|
                err.extend_with(|_, _|
                    json!({"code": 400})
                )
            )?
        )
    } */
}

// no traits are needed
#[derive(Debug, SimpleObject)]
struct Member {
    id: i32,
    name: String,
    knockouts: i32,
    team_id: i32,
}

#[derive(Default)]
pub struct MembersQuery;

#[Object]
impl MembersQuery {
    async fn members(&self) -> Result<Option<Vec<Member>>> {
        let pool = connect_db().await?;
        // let members = sqlx::query_as!( "SELECT M.id, M.name, knockouts, team_id FROM MEMBERS AS M JOIN TEAMS AS T ON M.team_id = T.id")
        let members = sqlx::query_as!(
            Member,
            "SELECT M.id, M.name, knockouts, team_id FROM MEMBERS AS M"
        )
        .fetch_all(&pool)
        .await?;
        println!("vv{:?}", members);
        Ok(Some(members))
    }
}

#[derive(Debug, SimpleObject)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(Default)]
pub struct TeamQuery;

#[Object]
impl TeamQuery {
    async fn users(&self) -> Result<Option<Vec<Team>>> {
        let pool = connect_db().await?;
        let teams: Vec<Team> = sqlx::query_as!(Team, "SELECT * FROM TEAMS")
            .fetch_all(&pool) // -> Vec<{ country: String, count: i64 }>
            .await?;
        println!("vv{:?}", teams);
        Ok(Some(teams))
    }
}

#[derive(MergedObject, Default)]
pub struct Query(MathQuery, MembersQuery, TeamQuery);

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
