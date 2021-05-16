import {
  Button,
  Table,
  TableCaption,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react";
import { InferGetServerSidePropsType } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import React from "react";
import If from "../../../components/If";
import Layout from "../../../components/Layout";
import Campaign from "../../../ethereum/campaign";
import web3 from "../../../ethereum/web3";

const RequestIndex = ({
  requests,
  requestCount,
  approversCount,
}: InferGetServerSidePropsType<typeof getServerSideProps>) => {
  const router = useRouter();

  const handleApproveRequest = async (id: string) => {
    const campaign = Campaign(router.query.address);

    const accounts = await web3.eth.getAccounts();
    await campaign.methods.approveRequest(id).send({
      from: accounts[0],
    });
  };

  const handleFinalizeRequest = async (id: string) => {
    const campaign = Campaign(router.query.address);

    const accounts = await web3.eth.getAccounts();
    await campaign.methods.finalizeRequest(id).send({
      from: accounts[0],
    });
  };

  //

  return (
    <Layout>
      <h3>Requests</h3>
      <Link href={`/campaigns/${router.query.address}/requests/new`}>
        <a>
          <Button>Add request</Button>
        </a>
      </Link>

      <Table variant="simple">
        <TableCaption>Kickstarter info</TableCaption>
        <Thead>
          <Tr>
            <Th>ID</Th>
            <Th>Description</Th>
            <Th isNumeric>Amount</Th>
            <Th>Recipient</Th>
            <Th>Approval Count</Th>
            <Th>Approve</Th>
            <Th>Finalize</Th>
          </Tr>
        </Thead>
        <Tbody>
          {requests?.map((request, i) => {
            // Hightlight that it is ready to be finalized. Give highlighted color to the row.
            const readyToFinalize = request.approvalCount > approversCount / 2;
            return (
              <Tr
                key={i}
                disabled={request.complete}
                positive={readyToFinalize && !request.complete}
              >
                <Td>{i}</Td>
                <Td>{request.description}</Td>
                <Td isNumeric>{web3.utils.fromWei(request.value, "ether")}</Td>
                <Td>{request.recipient}</Td>
                <Td>
                  {request.approvalCount}/{approversCount}
                </Td>
                <Td>
                  <If condition={!request.complete}>
                    <Button
                      color="teal"
                      onClick={() => handleApproveRequest(i)}
                    >
                      Approve
                    </Button>
                  </If>
                </Td>
                <If condition={!request.complete}>
                  <Td>
                    <Button
                      color="teal"
                      onClick={() => handleFinalizeRequest(i)}
                    >
                      Finalize
                    </Button>
                  </Td>
                </If>
              </Tr>
            );
          })}
        </Tbody>
      </Table>

      <div>Found {requestCount} requests</div>
    </Layout>
  );
};

export default RequestIndex;

export const getInitialProps = async (context) => {
  const campaign = Campaign(context.query.address);
  const requestCount = await campaign.methods.getRequestsCount().call();
  const approversCount = await campaign.methods.approversCount().call();

  const requests = await Promise.all(
    Array(Number(requestCount))
      .fill(null)
      .map((el, i) => campaign.methods.requests(i).call())
  );

  /*  if (!data) {
    return {
      notFound: true,
    };
  } */
  console.log({requests});

  return {
    props: {
      requests,
      address: context.query.address,
      requestCount,
      approversCount,
    },
  };
};
