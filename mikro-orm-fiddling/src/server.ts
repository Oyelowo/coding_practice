import express from "express";
import { MikroORM, RequestContext } from "mikro-orm";
import { AuthorController } from "./controllers/author.controller";
import { BookController } from "./controllers/book.controller";
import { Author } from "./entities/Author";
import { Book } from "./entities/Book";
import { mikroOrmConfig } from "./mikro-orm.config";

const app = express();
const port = process.env.PORT || 3000;

/* export const DI = {} as {
  orm: MikroORM;
  em: EntityManager;
  authorRepository: EntityRepository<Author>;
  bookRepository: EntityRepository<Book>;
}; */

const orm = await MikroORM.init(mikroOrmConfig);

export const DI = {
  orm: orm,
  em: orm.em,
  authorRepository: orm.em.getRepository(Author),
  bookRepository: orm.em.getRepository(Book),
};

app.use(express.json());
app.use((req, res, next) => RequestContext.create(DI.orm.em, next));
app.get("/", (req, res) =>
  res.json({
    message:
      "Welcome to MikroORM express TS example, try CRUD on /author and /book endpoints!",
  })
);
app.use("/author", AuthorController);
app.use("/book", BookController);
app.use((req, res) => res.status(404).json({ message: "No route found" }));

app.listen(port, () => {
  console.log(
    `MikroORM express TS example started at http://localhost:${port}`
  );
});
