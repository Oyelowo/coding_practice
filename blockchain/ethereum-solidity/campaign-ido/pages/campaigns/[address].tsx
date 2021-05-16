import { Box, Button, Grid, Heading, Text } from "@chakra-ui/react";
import { GetServerSideProps, InferGetServerSidePropsType } from "next";
import Link from "next/link";
import React from "react";
import ContributeForm from "../../components/ContributeForm";
import Layout from "../../components/Layout";
import Campaign from "../../ethereum/campaign";
import web3 from "../../ethereum/web3";

const Show = ({
  manager,
  minimumContribution,
  balance,
  requestCount,
  approversCount,
  address,
}: InferGetServerSidePropsType<typeof getServerSideProps>) => {
  console.log({ manager });
  const items = [
    {
      header: manager,
      meta: "Address of Manager",
      description: "The manager created this campaign and can create request",
    },
    {
      header: minimumContribution,
      meta: "Minimum contribution (Wei)",
      description: "You must contribute at least this to be an approver",
    },
    {
      header: requestCount,
      meta: "Number of requests",
      description:
        "A request tries to request money from the contract. Requests must be approved by the manager",
    },
    {
      header: approversCount,
      meta: "Number of approvers",
      description: "Number of people who have already donated to this project",
    },
    {
      header: web3.utils.fromWei(balance, "ether"),
      meta: "Campaign Balance (ether)",
      description: "How much money the campaign has left to spend",
    },
  ];
  return (
    <Layout>
      <Grid gap={10} templateColumns="2fr 1fr">
        <Grid
          spacing={8}
          templateColumns="repeat(2, 1fr)"
          gap={4}
          maxWidth="70vw"
        >
          {items.map(({ header, meta, description }) => (
            <Box p={5} shadow="md" borderWidth="1px" flex="1">
              <Heading fontSize="xl">{header}</Heading>
              <Text mt={4} size="s">
                {meta}
              </Text>
              <Text mt={4}>{description}</Text>
            </Box>
          ))}
        </Grid>

        <ContributeForm address={address} />
      </Grid>
      <Link href={`/campaigns/${address}/requests`}>
        <a>
          <Button>View Requests</Button>
        </a>
      </Link>
    </Layout>
  );
};

export default Show;

export const getServerSideProps: GetServerSideProps = async (context) => {
  const campaign = Campaign(context.query.address);
  const summary = await campaign.methods.getSummary().call();

  /*  if (!data) {
    return {
      notFound: true,
    };
  } */

  return {
    props: {
      minimumContribution: summary[0],
      balance: summary[1],
      requestCount: summary[2],
      approversCount: summary[3],
      manager: summary[4],
      address: context.query.address,
    }, // will be passed to the page component as props
  };
};
