/* import { prop, getModelForClass, Ref } from "@typegoose/typegoose";
import * as mongoose from "mongoose";

class Job {
  @prop()
  public title?: string;

  @prop()
  public position?: string;
}

class Car {
  @prop()
  public model?: string;
}

class User {
  @prop()
  public name?: string;

  @prop({ required: true })
  public age!: number;

  @prop()
  public job?: Job;

  @prop({ ref: Car })
  public car?: Ref<Car>;
}

const UserModel = getModelForClass(User); // UserModel is a regular Mongoose Model with correct types

(async () => {
  await mongoose.connect("mongodb://localhost:27017/", {
    useNewUrlParser: true,
    useUnifiedTopology: true,
    dbName: "test",
  });

  const userF = await UserModel.create({ age:2, job:{title: '3', position: '434'}, car: {model: ''}}); // an "as" assertion, to have types for all properties

  const user = await UserModel.findById(userF.id).exec();


  console.log(user); // prints { _id: 59218f686409d670a97e53e0, name: 'JohnDoe', __v: 0 }
})();


 */
export const f = ''