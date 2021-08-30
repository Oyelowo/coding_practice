import config from "./utils/config";
import app from "./app";
import http from 'http';

const server = http.createServer(app);

server.listen(config.PORT, () => {
    console.log(`Server running on port ${config.PORT}`)
})