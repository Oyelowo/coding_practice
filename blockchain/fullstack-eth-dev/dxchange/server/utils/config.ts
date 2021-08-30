import {config} from 'dotenv';
config()

let MONGO_URI = process.env.MONGO_URI;
const PORT = process.env.PORT || 3001;

if (process.env.NODE_ENV === 'test') {
    MONGO_URI = process.env.TEST_MONGO_URI
  }

export default {
    MONGO_URI,
    PORT
}