import { BaseEntity, } from "mikro-orm";
import { Author } from "./entities/Author";
import { Book } from "./entities/Book";
import { BookTag } from "./entities/BookTag";
import { Publisher } from "./entities/Publisher";
export const mikroOrmConfig = {
    dbName: "my-db-name",
    type: "mongo",
    entities: [Author, Book, BookTag, Publisher, BaseEntity],
    entitiesTs: ["app/entities"],
    logger: console.log.bind(console),
    debug: true,
};
