import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import * as App from "./wrap";
import path from "path";
import { config } from "dotenv";

config();

jest.setTimeout(60000);

describe("Relayer Kit Wrap", () => {
  const wrapUri = "wrapscan.io/polywrap/relay-kit@0.1.0";
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..");
  const client: PolywrapClient = new PolywrapClient(
    new PolywrapClientConfigBuilder()
      .addDefaults()
      .setRedirect(wrapUri, `fs/${wrapperPath}/build`)
      .build()
  );

  it("calls relay transaction", async () => {
    if (!process.env.RELAYER_API_KEY) {
      throw Error("Relayer API Key not defined in .env");
    }

    const feeToken = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

    const options = {
      gasLimit: "0",
      gasToken: feeToken,
      isSponsored: false,
    };

    const transaction = {
      target: "0xA045eb75e78f4988d42c3cd201365bDD5D76D406",
      encodedTransaction:
        "0xae53dcae000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa9604500000000000000000000000000000000000000000000000000038d7ea4c68000",
      chainId: 5,
      options,
    };

    const relayer = new App.Relayer();
    const result = await relayer.relayTransaction(
      { transaction },
      client,
      {
        relayerApiKey: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_",
      },
      wrapUri
    );

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value.taskId).toBeTruthy();
  });
});
