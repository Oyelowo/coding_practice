import CampaignFactory from "./build/CampaignFactory.json";
import web3 from "./web3";

const instance = new web3.eth.Contract(
  JSON.parse(CampaignFactory.interface),
  "0xcd323DC26114eDDD42D0771D36CAC3288932c26D"
);

export default instance;
