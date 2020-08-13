import { Book, BookTag, Author } from './entities';

export default {
  entities: [Author, Book, BookTag],
  dbName: "my-db-name",
  type: "mongo", // one of `mongo` | `mysql` | `mariadb` | `postgresql` | `sqlite`
};
