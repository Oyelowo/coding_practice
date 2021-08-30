import express from "express";
import { UserModel } from "./model";

const userRouter = express.Router();

userRouter.get("/user", async (req, res, next) => {
  try {
   /*  const blogs = await UserModel.find({});
    res.json(blogs); */

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
  } catch (error) {
    next(error);
  }
});

/* 
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
*/

/* userRouter.post("/", async (req, res, next) => {
  try {
    const blog = await new Blog(req.body).save();
    res.json(blog);
  } catch (error) {
    next(error);
  }
}); */

export default userRouter;
