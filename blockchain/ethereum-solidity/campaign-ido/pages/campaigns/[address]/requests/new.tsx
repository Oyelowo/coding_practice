import {
  Button,
  Input,
  InputGroup,
  InputRightAddon,
  Text,
} from "@chakra-ui/react";
import Link from "next/link";
import { useRouter } from "next/router";
import React, { FormEvent, useState } from "react";
import If from "../../../../components/If";
import Layout from "../../../../components/Layout";
import Campaign from "../../../../ethereum/campaign";
import web3 from "../../../../ethereum/web3";
import useForm from "../../../../hooks/useForm";

const RequestNew = () => {
  const { getInputAttributes } = useForm({
    description: "",
    recipient: "",
    etherValue: "",
  });
  const [errorMessage, setErrorMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const descriptionInput = getInputAttributes("description");
  const recipientInput = getInputAttributes("recipient");
  const etherValueInput = getInputAttributes("etherValue");
  const router = useRouter();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    try {
      setIsLoading(true);
      setErrorMessage("");
      const campaign = Campaign(router.query.address);
      const accounts = await web3.eth.getAccounts();
      await campaign.methods
        .createRequest(
          descriptionInput.value,
          web3.utils.toWei(etherValueInput.value, "ether"),
          recipientInput.value
        )
        .send({
          from: accounts[0],
        });

      setIsLoading(false);
      router.push(`/campaigns/${router.query.address}/requests`);
    } catch (e) {
      setIsLoading(false);
      setErrorMessage(e.message);
    }
  };
  return (
    <Layout>
      <Link href={`/campaigns/${router.query.address}/requests`}>
        <a>
          <Button>Back</Button>
        </a>
      </Link>
      <h3>Create a Request</h3>

      <form onSubmit={handleSubmit}>
        <label>Description</label>
        <br />
        <InputGroup size="sm">
          <Input
            {...descriptionInput}
            // isInvalid={!!errorMessage}
            errorBorderColor="red.300"
          />

          <br />
        </InputGroup>

        <label>Value in Ether</label>
        <br />

        <InputGroup size="sm">
          <Input
            {...etherValueInput}
            // isInvalid={!!errorMessage}
            errorBorderColor="red.300"
          />
          <InputRightAddon children="Ether" />
          <br />
        </InputGroup>

        <label>Recipient</label>
        <br />
        <InputGroup size="sm">
          <Input
            {...recipientInput}
            // isInvalid={!!errorMessage}
            errorBorderColor="red.300"
          />

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
    </Layout>
  );
};

export default RequestNew;
