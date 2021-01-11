use async_graphql::*;
use serde_json::json;
use std::num::ParseIntError;
use std::default::Default;

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

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self) -> String {
        "todo!()".to_string()
    }
}

#[derive(Default)]
pub struct MovieQuery;

#[Object]
impl MovieQuery {
    async fn movies(&self) -> String {
        "todo!()2".to_string()
    }
}

#[derive(MergedObject, Default)]
pub struct Query(MathQuery, UserQuery, MovieQuery);

#[warn(private_in_public)]
pub type SchemaGraphQL = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> SchemaGraphQL {
    Schema::new(Query::default(), EmptyMutation, EmptySubscription)
}
