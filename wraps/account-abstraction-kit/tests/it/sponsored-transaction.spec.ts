import * as App from "../types/wrap";
import { BigNumber } from "ethers";
import { CONNECTION, SALT_NONCE, getClient } from "./utils";

jest.setTimeout(60000);

describe("Sponsored transaction AA wrapper", () => {
  const client = getClient();

  const ethers = new App.Ethers(client);
  const accountAbstraction = new App.AccountAbstraction(client);
  const relay = new App.Relayer(client);

  it("calls relay transaction", async () => {
    const encodedFunction = await ethers.encodeFunction({
      method: "function store(uint256 num) public",
      args: ["4"],
    });

    if (!encodedFunction.ok) throw encodedFunction.error;
    const metaTransactionData = {
      to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e",
      value: "0",
      data: encodedFunction.value,
    };

    const gasLimit = await ethers.estimateTransactionGas({
      tx: {
        to: metaTransactionData.to,
        value: metaTransactionData.value,
        data: metaTransactionData.data,
      },
    });
    if (!gasLimit.ok) throw gasLimit.error;

    const gaslimitWithBuffer = BigNumber.from(gasLimit.value)
      .add(300_000)
      .toString();
    const estimation = await relay.getEstimateFee({
      chainId: 5,
      gasLimit: gaslimitWithBuffer,
    });
    if (!estimation.ok) throw estimation.error;

    const safeAddress = await accountAbstraction.getSafeAddress({
      config: {
        saltNonce: SALT_NONCE,
      },
    });
    if (!safeAddress.ok) throw safeAddress.error;
    console.log("Predicted safe address: ", safeAddress.value);
    const safeBalance = await ethers.getBalance({
      address: safeAddress.value,
      connection: CONNECTION,
    });
    if (!safeBalance.ok) throw safeBalance.error;

    const estimationInEth = await ethers.toEth({
      wei: estimation.value,
    });
    if (!estimationInEth.ok) throw estimationInEth.error;
    console.log(`Fee estimation: ${estimationInEth.value} ETH`);

    const metaTransactionOptions = {
      gasLimit: gaslimitWithBuffer,
      isSponsored: true,
    };

    console.log("Relaying sponsored transaction...");
    const result = await accountAbstraction.relayTransaction({
      transaction: metaTransactionData,
      options: metaTransactionOptions,
      config: {
        saltNonce: SALT_NONCE,
      },
    });
    expect(result.ok).toBeTruthy();
    if (!result.ok) fail(result.error);
    console.log("Transaction has been relayed...");
    console.log(
      `Task URL: https://relay.gelato.digital/tasks/status/${result.value}`
    );
    expect(result.value).toBeTruthy();
  });
});
