import { Document, model, Schema } from "mongoose";
import { convertSchemaToJSON } from "../../utils/mongoose-utils";
import mongooseUniqueValidator = require("mongoose-unique-validator");

/* export interface IUser {
  author: string;
  title: string;
  url: string;
  upvotes: number;
  claps: number;
}
 */
// 1. Create an interface representing a document in MongoDB.
export interface IUser {
  name: string;
  email: string;
  avatar?: string;
}

export interface IUserDocument extends IUser, Document {}

// 2. Create a Schema corresponding to the document interface.
const UserSchema = new Schema({
  name: { type: String, required: true },
  email: { type: String, required: true },
  avatar: String,
  upvotes: Number,
});

// 3. Create a Model.
export const UserModel = model<IUserDocument>("User", UserSchema);

UserSchema.plugin(mongooseUniqueValidator, {
  message: "Error, {VALUE} is already taken",
});

convertSchemaToJSON(UserSchema);

/* 
//type UserDocument = User & mongoose.Document;
export interface IUserDocument extends IUser, mongoose.Document {}

const UserSchema = new Schema({
  author: {
    type: String,
    required: true,
    minlength: 5
  },
  title: {
    type: String,
    required: true
  },
  url: {
    type: String,
    required: true
  },
  upvotes: Number,
  claps: Number
});

UserSchema.plugin(mongooseUniqueValidator, { message: 'Error, {VALUE} is already taken' });

convertSchemaToJSON(UserSchema);

export const User = mongoose.model<IUserDocument>('User', UserSchema);
 */
