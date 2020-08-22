import { Collection, Entity, Enum, OneToMany, Property } from "mikro-orm";
import { BaseEntity } from "./BaseEntity";
import { Book } from "./Book";

@Entity()
export class Publisher extends BaseEntity {
  @Property()
  name: string;

  @Enum()
  type: PublisherType;

  @OneToMany(() => Book, (b) => b.publisher)
  books = new Collection<Book>(this);

  constructor(name: string, type = PublisherType.LOCAL) {
    super()
    this.name = name;
    this.type = type;
  }
}

export enum PublisherType {
  LOCAL = "local",
  GLOBAL = "global",
}
