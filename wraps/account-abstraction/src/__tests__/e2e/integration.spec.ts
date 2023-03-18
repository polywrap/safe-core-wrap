import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";
import { configure } from "../../../client-config";
import { EthersUtils_Module } from "../types/wrap";

jest.setTimeout(60000);

describe("Account abstraction wrapper", () => {
  const builder = new ClientConfigBuilder();
  const configuredBuilder = configure(builder);
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "..");
  let wrapperFsUri: string = `fs/${wrapperPath}/build`;
  const wrapperUri = "wrap://wrapper/account-abstraction"
  configuredBuilder
    .addEnv(wrapperUri, {
      connection: {
        networkNameOrChainId: "goerli"
      }
    })
    .addEnv(
      "wrap://ens/account-abstraction.wraps.eth:relayer-adapter@0.0.1", {
        relayerApiKey: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_"
      }
    )
    .addRedirect(wrapperUri, wrapperFsUri)
  const client = new PolywrapClient(configuredBuilder.build());

  it("calls relay transaction", async () => {
    const encodedFunction = await EthersUtils_Module.encodeFunction({
      method: "function store(uint256 num) public",
      args: ["99"],
    }, client, "wrap://ens/wraps.eth:ethereum-utils@0.0.1")

    if (!encodedFunction.ok) throw encodedFunction.error;
    const metaTransactionData = {
      to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e",
      value: "0",
      data: encodedFunction.value,
      operation: "1",
    };

    const metaTransactionOptions = {
      gasLimit: "250000",
    };

    

    const result = await App.AccountAbstraction_Module.relayTransaction(
      {
        transaction: metaTransactionData,
        options: metaTransactionOptions,
      },
      client,
      wrapperUri
    );
    console.log(result)
    expect(result.ok).toBeTruthy();
    if (!result.ok) fail(result.error);
    expect(result.value).toBeTruthy();
  });
});
