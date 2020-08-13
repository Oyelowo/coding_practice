import { MikroORM } from 'mikro-orm';
import express from 'express';
const orm = await MikroORM.init({
    entities: [],
    dbName: "my-db-name",
    type: "mongo",
    //clientUrl: "...", // defaults to 'mongodb://localhost:27017' for mongodb driver
    baseDir: __dirname,
});
console.log(orm.em);
const app = express();
app.use((req, res, next) => {
    //RequestContext.create(orm.em, next);
});
