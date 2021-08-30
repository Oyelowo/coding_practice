import { connect, connection } from "mongoose";

export const connectDB = async () => {
  await connect("mongodb://mongodb:27017/test", {
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
};
