import {
  Cascade,
  Collection,
  Entity,
  OneToMany,
  Property,
  ManyToOne, ManyToMany
} from "mikro-orm";

import { Book, BookTag } from ".";
import { BaseEntity } from "./BaseEntity";

@Entity()
export class Author extends BaseEntity {
  @Property()
  name: string;

  @Property()
  email: string;

  @Property()
  age?: number;

  @Property()
  termsAccepted = false;

  @Property()
  born?: Date;

  @OneToMany(() => Book, (b) => b.author, { cascade: [Cascade.ALL] })
  books = new Collection<Book>(this);

  //@OneToMany(() => Book, 'author', { cascade: [Cascade.ALL] })
  //books = new Collection<Book>(this);

  @ManyToOne()
  favouriteBook?: Book;

  // when none of `owner/inverseBy/mappedBy` is provided, it will be considered owning side
  @ManyToMany()
  tags1 = new Collection<BookTag>(this);

  @ManyToMany(() => BookTag, "books", { owner: true })
  tags2 = new Collection<BookTag>(this);

  @ManyToMany(() => BookTag, "books", { owner: true })
  tags3 = new Collection<BookTag>(this);

  @ManyToMany(() => BookTag, "books", { owner: true })
  tags4 = new Collection<BookTag>(this);

  constructor(name: string, email: string) {
    super();
    this.name = name;
    this.email = email;
  }
}
