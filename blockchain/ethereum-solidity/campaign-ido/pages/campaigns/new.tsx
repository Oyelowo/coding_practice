import {
  Button,
  Input,
  InputGroup,
  InputRightAddon,
  Spinner,
  Text,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import React, { FormEvent, useState } from "react";
import If from "../../components/If";
import Layout from "../../components/Layout";
import factory from "../../ethereum/factory";
import web3 from "../../ethereum/web3";
import useForm from "../../hooks/useForm";

export default function CampaignNew() {
  const { getInputAttributes } = useForm({ minimumContribution: "" });
  const minimumContributionInput = getInputAttributes("minimumContribution");
  const [errorMessage, setErrorMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      setIsLoading(true);
      const accounts = await web3.eth.getAccounts();
      await factory.methods
        .createCampaign(minimumContributionInput.value)
        .send({
          from: accounts[0],
        });
      setIsLoading(false);
      router.push("/");
    } catch (e) {
      setIsLoading(false);
      setErrorMessage(e.message);
    }
  };
  return (
    <Layout>
      <h3>New campaign</h3>
      <form onSubmit={handleSubmit}>
        <label>Minimum Contribution </label>
        <br />
        <InputGroup size="sm">
          <Input
            {...minimumContributionInput}
            isInvalid={!!errorMessage}
            errorBorderColor="red.300"
          />
          <InputRightAddon children="wei" />
          <br />
        </InputGroup>
        <If condition={!!errorMessage}>
          <Text color="red.500" noOfLines={2}>
            {errorMessage}
          </Text>
        </If>
        <Button
          isLoading={isLoading}
          loadingText="Submitting"
          colorScheme="teal"
          variant="outline"
          type="submit"
          onClick={handleSubmit}
        >
          Create
        </Button>
      </form>
      <If condition={isLoading}>
        <Spinner
          thickness="4px"
          speed="0.65s"
          emptyColor="gray.200"
          color="blue.500"
          size="xl"
        />
      </If>
    </Layout>
  );
}
