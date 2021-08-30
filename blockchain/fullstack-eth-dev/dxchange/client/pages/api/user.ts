// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from "next";
import { connectDB } from "../../../server/mongo-config";
import { User, UserModel } from "./model";
/* import Cors from "cors";
import initMiddleware from "../../lib/init-middleware";
import { connectDB } from '../../mongo-config';

// Initialize the cors middleware
const cors = initMiddleware(
  // You can read more about the available options here: https://github.com/expressjs/cors#configuration-options
  Cors({
    // Only allow requests with GET, POST and OPTIONS
    methods: ["GET", "POST", "OPTIONS"],
  })
);
 */

type Data = {
  name: string;
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<User>
) {
  // Run cors
  // await cors(req, res);
  /*  run().catch((err) => console.log(err));

  const db = connection;
  db.once("open", (_) => {
    console.log("Database connected:", "url");
  });

  db.on("error", (err) => {
    console.error("connection error:", err);
  });
 */
  // 4. Connect to MongoDB
  // await connect("mongodb://localhost:27017/test", {
  // 'mongodb://$[username]:$[password]@$[hostlist]/$[database]?authSource=$[authSource]
  // 'mongodb://root:example@27017/$[database]?authSource=$[authSource]
  /*     await connect("mongodb://mongodb:27017/test", {
    useNewUrlParser: true,
    useUnifiedTopology: true,
  } as any);

    const db = connection;
    db.once("open", (_) => {
      console.log("Database connected:", "url");
    });

    db.on("error", (err) => {
      console.error("connection error:", err);
    });
 */

  await connectDB();
  const doc = new UserModel({
    name: "Bill",
    email: "bill@initech.com",
    avatar: "https://i.imgur.com/dM7Thhn.png",
  });
  await doc.save();

  console.log(doc.email); // 'bill@initech.com'
  res
    .status(200)
    .json({ name: doc.name, email: doc.email, avatar: doc.avatar });
}
