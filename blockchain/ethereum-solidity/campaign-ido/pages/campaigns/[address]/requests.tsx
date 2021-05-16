import {
  Button,
  Table,
  TableCaption,
  Tbody,
  Td,
  Tfoot,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react";
import { GetServerSideProps, InferGetServerSidePropsType } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import React from "react";
import Layout from "../../../components/Layout";
import Campaign from "../../../ethereum/campaign";
import web3 from "../../../ethereum/web3";

const RequestIndex = ({
  requests,
  requestCount,
  approversCount,
}: InferGetServerSidePropsType<typeof getServerSideProps>) => {
  const router = useRouter();
  return (
    <Layout>
      <h3>Requests</h3>
      <Link href={`/campaigns/${router.query.address}/requests/new`}>
        <a>
          <Button>Add request</Button>
        </a>
      </Link>

      <Table variant="simple">
        <TableCaption>Imperial to metric conversion factors</TableCaption>
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
          {requests.map((request, i) => (
            <Tr key={i}>
              <Td>{i}</Td>
              <Td>{request.description}</Td>
              <Td isNumeric>{web3.utils.fromWei(request.value, "ether")}</Td>
              <Td isNumeric>{request.recipient}</Td>
              <Td isNumeric>
                {request.approvalCounts}/{approversCount}
              </Td>
              <Td isNumeric>{request.recipient}</Td>
            </Tr>
          ))}
        </Tbody>
        <Tfoot>
          <Tr>
            <Th>To convert</Th>
            <Th>into</Th>
            <Th isNumeric>multiply by</Th>
          </Tr>
        </Tfoot>
      </Table>
    </Layout>
  );
};

export default RequestIndex;

export const getServerSideProps: GetServerSideProps = async (context) => {
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
  console.log(requests);

  return {
    props: {
      requests,
      address: context.query.address,
      requestCount,
      approversCount,
    },
  };
};
