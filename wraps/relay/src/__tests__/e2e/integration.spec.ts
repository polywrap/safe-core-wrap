import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";
import { configure } from "../../../client-config";

jest.setTimeout(60000);

describe("Relayer wrapper", () => {
  const client: PolywrapClient = new PolywrapClient(
    configure(new ClientConfigBuilder()).build()
  );
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "..");
  const wrapperUri = `fs/${wrapperPath}/build`;

  it("calls relay transaction", async () => {
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
    const result = await App.Relayer_Module.relayTransaction(
      { transaction },
      client,
      wrapperUri
    );

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value.taskId).toBeTruthy();
  });
});
