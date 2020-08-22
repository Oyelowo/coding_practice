import {
  Cascade,
  Collection,
  Entity,
  ManyToMany,
  ManyToOne,
  Property,
} from "mikro-orm";
import { Author } from "./Author";
import { BaseEntity } from "./BaseEntity";
import { BookTag } from "./BookTag";
import { Publisher } from "./Publisher";

@Entity()
export class Book extends BaseEntity {
  @Property()
  title: string;

  @ManyToOne()
  author: Author;

  @ManyToOne({ cascade: [Cascade.PERSIST, Cascade.REMOVE] })
  publisher?: Publisher;

  @ManyToMany()
  tags = new Collection<BookTag>(this);

  @Property()
  metaObject?: object;

  @Property()
  metaArray?: any[];

  @Property()
  metaArrayOfStrings?: string[];

  constructor(title: string, author: Author) {
    super();
    this.title = title;
    this.author = author;
  }
}
