import { PolywrapClient } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";

jest.setTimeout(60000);

describe("Relayer wrapper", () => {
  const client: PolywrapClient = new PolywrapClient();
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "..");
  const wrapperUri = `fs/${wrapperPath}/build`;

  it("calls relay transaction", async () => {
    const expected: string = "foo";

    const options = {
      gasLimit: "0",
      gasToken: "0x",
      isSponsored: false,
    }

    const transaction = {
      target: "0x",
      encodedTransaction: "0x",
      chainId: 5,
      options
    };
    const result = await App.Relayer_Module.relayTransaction(
      { transaction },
      client,
      wrapperUri
    );

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value.taskId).toEqual(expected);
  });
});
