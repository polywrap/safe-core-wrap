import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";
import { configure } from "../../../client-config";
import { EthersUtils_Module } from "../types/wrap";
import { BigNumber } from "ethers";

jest.setTimeout(60000);

const connection = {
  networkNameOrChainId: "goerli",
};

describe("Paid transaction AA wrapper", () => {
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "..");
  const accountAbstractionWrapperFsUri = `fs/${wrapperPath}/build`;

  const accountAbstractionWrapperUri = "wrap://wrapper/account-abstraction";
  const etherUtilsWrapperUri = "wrap://ens/wraps.eth:ethereum-utils@0.0.1";
  const etherCoreWrapperUri = "wrap://ens/wraps.eth:ethereum@2.0.0";
  const relayerAdapterWrapperUri =
    "wrap://ens/account-abstraction.wraps.eth:relayer-adapter@0.0.1";

  const builder = new ClientConfigBuilder();
  const configuredBuilder = configure(builder);
  configuredBuilder
    .addEnv(accountAbstractionWrapperUri, {
      connection,
    })
    .addEnv(relayerAdapterWrapperUri, {
      relayerApiKey: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_",
    })
    .addRedirect(accountAbstractionWrapperUri, accountAbstractionWrapperFsUri);

  const client = new PolywrapClient(configuredBuilder.build());

  it("calls relay transaction", async () => {
    const encodedFunction = await EthersUtils_Module.encodeFunction(
      {
        method: "function store(uint256 num) public",
        args: ["10"],
      },
      client,
      etherUtilsWrapperUri
    );

    if (!encodedFunction.ok) throw encodedFunction.error;
    const metaTransactionData = {
      to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e",
      value: "0",
      data: encodedFunction.value,
      operation: "0",
    };

    const estimation = await App.Relayer_Module.getEstimateFee(
      {
        chainId: 5,
        gasLimit: "250000",
      },
      client,
      relayerAdapterWrapperUri
    );
    if (!estimation.ok) throw estimation.error;

    const safeAddress = await App.AccountAbstraction_Module.getSafeAddress(
      {
      config: {
        saltNonce: "0x99"
      }},
      client,
      accountAbstractionWrapperUri
    );
    if (!safeAddress.ok) throw safeAddress.error;
    console.log("Safe address: ", safeAddress.value);
    const safeBalance = await App.EtherCore_Module.getBalance(
      {
        address: safeAddress.value,
        connection,
      },
      client,
      etherCoreWrapperUri
    );
    if (!safeBalance.ok) throw safeBalance.error;

    const safeBalanceInEth = await App.EtherCore_Module.toEth({
      wei: safeBalance.value
    }, client, etherCoreWrapperUri);
    if (!safeBalanceInEth.ok) throw safeBalanceInEth.error;

    console.log(`Safe balance: ${safeBalanceInEth.value} ETH`);
    const estimationInEth = await App.EtherCore_Module.toEth({
      wei: estimation.value
    }, client, etherCoreWrapperUri);
    if (!estimationInEth.ok) throw estimationInEth.error;
    console.log(`Fee estimation: ${estimationInEth.value} ETH`);

    if (BigNumber.from(safeBalance.value).lt(estimation.value)) {
      const valueInEth = await App.EtherCore_Module.toEth({
        wei: estimation.value
      }, client, etherCoreWrapperUri)
      if (!valueInEth.ok) throw valueInEth.error;
      console.log(
        `Funding the Safe with ${valueInEth.value} ETH`
      )
      const sendTx = await App.EtherCore_Module.sendTransactionAndWait(
        {
          tx: {
            value: estimation.value,
            to: safeAddress.value,
            data: "0x",
          },
          connection,
        },
        client,
        etherCoreWrapperUri
      );
      if (!sendTx.ok) throw sendTx.error;
      console.log("Safe funded");
    }
    const metaTransactionOptions = {
      gasLimit: "250000",
    };

    console.log("Relaying paid transaction...")
    const result = await App.AccountAbstraction_Module.relayTransaction(
      {
        transaction: metaTransactionData,
        options: metaTransactionOptions,
        config: {
          saltNonce: "0x99"
        }
      },
      client,
      accountAbstractionWrapperUri
    );
    
    if (!result.ok) fail(result.error);
    expect(result.ok).toBeTruthy();
    console.log("Transaction has been relayed...")
    console.log(`Task URL: https://relay.gelato.digital/tasks/status/${result.value}`)
    expect(result.value).toBeTruthy();
  });
});
