import { PolywrapClient } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";

jest.setTimeout(60000);

describe("Account abstraction wrapper", () => {
  const client = new PolywrapClient();
  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..", "..", "..");
  let wrapperUri: string = `fs/${wrapperPath}/build`;

  it("calls relay transaction", async () => {
    const expected: string = "lmao";

    const metaTransactionData = {
      to: "",
      value: "0",
      data: "",
      operation: 0,
    };

    const metaTransactionOptions = {
      gasLimit: "0",
      gasToken: "",
      isSponsored: false,
    };

    const result = await App.AccountAbstraction_Module.relayTransaction(
      {
        transaction: metaTransactionData,
        options: metaTransactionOptions,
      },
      client,
      wrapperUri
    );

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value).toEqual(expected);
  });
});
