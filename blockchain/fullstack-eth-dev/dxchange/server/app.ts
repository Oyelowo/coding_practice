import express from 'express';
import cors from 'cors';
import mongoose from 'mongoose';
import bodyParser = require('body-parser');
import morgan = require('morgan');
import config from './utils/config';
import logger from './utils/logger';
import middlewares from './utils/middlewares';
import userRouter from './features/user/controller';
const app = express();

logger.info('connecting to', config.MONGO_URI);

mongoose
  .connect(config.MONGO_URI as string, { useNewUrlParser: true })
  .then(() => {
    logger.info('Connected to MongoDb');
  })
  .then((error) => {
    logger.error('something went wrong while connecting to MongoDb', error);
  });

app.use(cors());
app.use(express.static('build'));
app.use(bodyParser.json());
app.use(middlewares.requestLogger)
//app.use(morgan('dev')); 

app.use('/api/v1/users', userRouter);

app.use(middlewares.unknownEndpoint)
app.use(middlewares.errorHandler);


export default app;
