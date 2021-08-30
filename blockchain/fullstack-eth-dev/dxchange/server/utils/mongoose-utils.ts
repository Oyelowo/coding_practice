import { Schema } from "mongoose";

export const convertSchemaToJSON = (schema: Schema) => {
    schema.set('toJSON',{
        transform
        : (document, returnedObject)=> {
            returnedObject.id = returnedObject._id.toString()
            delete returnedObject._id
            delete returnedObject.__v
        }
    })
}