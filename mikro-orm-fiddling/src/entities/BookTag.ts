import { ObjectID } from "mongodb";
import {
  Collection,
  Entity,
  ManyToMany,
  PrimaryKey,
  Property,
  SerializedPrimaryKey,
} from "mikro-orm";

import { BaseEntity } from './BaseEntity';
import { Book } from "./Book";

@Entity()
export class BookTag extends BaseEntity{

  @Property()
  name: string;

  @ManyToMany(() => Book, (b) => b.tags)
  books: Collection<Book> = new Collection<Book>(this);

  constructor(name: string) {
    super()
    this.name = name;
  }
}
