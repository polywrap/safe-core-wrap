import * as App from "../wrap";
import { createTransaction, getClient, SALT_NONCE } from "../utils";
import { BigNumber } from "ethers";

export const sponsoredTransaction = async () => {
  console.log("Execute meta-transaction via Gelato Relay paid by 1Balance");

  const client = getClient();
  const accountAbstraction = new App.AccountAbstraction(client);
  const ethers = new App.Ethers(client);
  const { gasLimit, transaction } = await createTransaction(ethers);

  const gasLimitWithBuffer = BigNumber.from(gasLimit).add(250_000).toString();

  const address = await accountAbstraction.getSafeAddress({
    config: {
      saltNonce: SALT_NONCE,
    },
  });
  if (!address.ok) throw address.error;

  console.log("Predicted safe address: ", address);

  const metaTransactionOptions = {
    gasLimit: gasLimitWithBuffer,
    isSponsored: true,
  };

  const response = await accountAbstraction.relayTransaction({
    transaction,
    options: metaTransactionOptions,
    config: {
      saltNonce: SALT_NONCE,
    },
  });

  if (!response.ok) throw response.error;

  console.log("Transaction has been relayed...");
  console.log(
    `Task URL: https://relay.gelato.digital/tasks/status/${response.value}`
  );
};
