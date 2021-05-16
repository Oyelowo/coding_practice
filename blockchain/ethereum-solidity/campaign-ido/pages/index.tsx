import { Button } from "@chakra-ui/react";
import { GetServerSideProps, InferGetServerSidePropsType } from "next";
import Link from "next/link";
import React from "react";
import Layout from "../components/Layout";
import factory from "../ethereum/factory";

function HomePage({
  data: campaigns,
}: InferGetServerSidePropsType<typeof getServerSideProps>) {
  return (
    <Layout>
      Welcome to Campaign list page! {campaigns[0]}
      <h3>Open campaigns</h3>
      <ul>
        {campaigns?.map((address) => (
          <li>
            address: {address}
            <br />
            description: description
            <br />
            <Link href={`/campaigns/${address}`}>
              <a style={{ color: "blue" }}>view campaign</a>
            </Link>
            <br />
          </li>
        ))}
      </ul>
      <hr />
      <Link href="/campaigns/new">
        <a>
          <Button>Create Campaign</Button>
        </a>
      </Link>
    </Layout>
  );
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const data: string[] = await factory.methods.getDeployedCampaigns().call();

  /*  if (!data) {
    return {
      notFound: true,
    };
  } */

  return {
    props: { data }, // will be passed to the page component as props
  };
};

export default HomePage;
