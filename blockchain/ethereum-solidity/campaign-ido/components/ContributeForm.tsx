import {
  Box,
  Button,
  Input,
  InputGroup,
  InputRightAddon,
  Text,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import React, { FC, FormEvent, useState } from "react";
import Campaign from "../ethereum/campaign";
import web3 from "../ethereum/web3";
import useForm from "../hooks/useForm";
import If from "./If";

const ContributeForm: FC<{ address: string }> = ({ address }) => {
  const { getInputAttributes } = useForm({ contribute: "" });
  const contributeInput = getInputAttributes("contribute");
  const [errorMessage, setErrorMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      setIsLoading(true);
      setErrorMessage("");
      const campaign = Campaign(address);
      const accounts = await web3.eth.getAccounts();
      await campaign.methods.contribute().send({
        from: accounts[0],
        value: web3.utils.toWei(contributeInput.value, "ether"),
      });

      setIsLoading(false);
      router.replace(`/campaigns/${address}`);
    } catch (e) {
      setIsLoading(false);
      setErrorMessage(e.message);
    }
  };

  return (
    <Box maxWidth={500}>
      <form onSubmit={handleSubmit}>
        <label>Amount to Contribute </label>
        <br />
        <InputGroup size="sm">
          <Input {...contributeInput} errorBorderColor="red.300" />
          <InputRightAddon children="ether" />
          <br />
        </InputGroup>
        <Button
          isLoading={isLoading}
          loadingText="Submitting"
          colorScheme="teal"
          variant="outline"
          onClick={handleSubmit}
        >
          Contribute
        </Button>
      </form>
      <If condition={!!errorMessage}>
        <Text color="red.500" noOfLines={2}>
          {errorMessage}
        </Text>
      </If>
    </Box>
  );
};

export default ContributeForm;
