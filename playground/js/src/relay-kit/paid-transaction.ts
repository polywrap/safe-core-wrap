import * as App from "../wrap";
import { createTransaction, getClient, SALT_NONCE } from "../utils";
import { BigNumber } from "ethers";

export const paidTransaction = async () => {
  const client = getClient();
  const accountAbstraction = new App.AccountAbstraction(client);
  const ethers = new App.Ethers(client);
  const relay = new App.Relayer(client);

  const { gasLimit, transaction } = await createTransaction(ethers);

  const address = await accountAbstraction.getSafeAddress({
    config: {
      saltNonce: SALT_NONCE,
    },
  });
  if (!address.ok) throw address.error;

  console.log("Predicted safe address: ", address);

  const safeBalance = await ethers.getBalance({
    address: address.value,
  });
  if (!safeBalance.ok) throw safeBalance.error;

  const estimation = await relay.getEstimateFee({
    chainId: 5,
    gasLimit,
  });

  if (!estimation.ok) throw estimation.error;

  const estimationInWei = BigNumber.from(estimation.value).mul(10e8).toString();
  if (BigNumber.from(safeBalance.value).lt(estimationInWei)) {
    const valueInEth = await ethers.toEth({
      wei: estimationInWei,
    });

    if (!valueInEth.ok) throw valueInEth.error;
    console.log(`Funding the Safe with ${valueInEth.value} ETH`);
    const sendTx = await ethers.sendTransactionAndWait({
      tx: {
        value: estimationInWei,
        to: address.value,
        data: "0x",
      },
    });
    if (!sendTx.ok) throw sendTx.error;
    console.log("Safe funded");
  }

  const gasLimitWithBuffer = BigNumber.from(estimation.value)
    .add(250_000)
    .toString();
  const metaTransactionOptions = {
    gasLimit: gasLimitWithBuffer,
  };

  const response = await accountAbstraction.relayTransaction({
    transaction: transaction,
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
