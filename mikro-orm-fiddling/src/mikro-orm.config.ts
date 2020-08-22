import {
  BaseEntity,
  Configuration,
  Connection,
  IDatabaseDriver,
  MikroORM,
  Options,
} from "mikro-orm";
import { Author } from "./entities/Author";
import { Book } from "./entities/Book";
import { BookTag } from "./entities/BookTag";
import { Publisher } from "./entities/Publisher";

type MConfigAlt =
  | Options<IDatabaseDriver<Connection>>
  | Configuration<IDatabaseDriver<Connection>>
  | undefined;
type MConfig = Parameters<typeof MikroORM.init>[0];

export const mikroOrmConfig: MConfig = {
  dbName: "my-db-name",
  type: "mongo", // one of `mongo` | `mysql` | `mariadb` | `postgresql` | `sqlite`
  entities: [Author, Book, BookTag, Publisher, BaseEntity],
  entitiesTs: ["app/entities"],
  logger: console.log.bind(console),
  debug: true,
};
