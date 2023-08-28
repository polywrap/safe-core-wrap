import * as App from "./types/wrap";
import {
  CONNECTION,
  SupportedContracts,
  VERSIONS,
  createTransaction,
  fundSafeBalance,
  getClient,
  initInfra,
  setUpContracts,
  stopInfra,
} from "./utils";

jest.setTimeout(500000);

const safeVersion = process.env.SAFE_VERSION as VERSIONS;

describe("Transactions execution", () => {
  let contracts: SupportedContracts<string>;
  let safe: App.Safe;
  let ethers: App.Ethers;
  let safeAddress: string;

  beforeAll(async () => {
    await initInfra();
    const client = getClient();
    contracts = await setUpContracts(client);
    safe = new App.Safe(client);
    ethers = new App.Ethers(client);

    const deploySafeResponse = await safe.deploySafe({
      input: {
        safeAccountConfig: {
          owners: ["0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1"],
          threshold: 1,
        },
        connection: CONNECTION,
        customContractAddresses: {
          safeFactoryContract: contracts.SAFE![safeVersion],
          proxyFactoryContract: contracts.FACTORY![safeVersion],
        },
      },
    });

    if (!deploySafeResponse.ok) {
      throw "Error deploying test safe: " + deploySafeResponse.error;
    }
    safeAddress = deploySafeResponse.value;
  });

  afterAll(async () => {
    await stopInfra();
  });

  it.only("Should execute a transaction with threshold 1", async () => {
    await fundSafeBalance(safeAddress);

    const balanceBefore = await ethers.getBalance({
      address: safeAddress,
      blockTag: null,
      connection: CONNECTION,
    });
    if (!balanceBefore.ok) fail(balanceBefore.error);

    const transaction = await createTransaction({
      value: "250000000000000000",
    });

    const signedTxRes = await safe.addSignature(
      { tx: transaction },
      undefined,
      { safeAddress, connection: CONNECTION }
    );
    if (!signedTxRes.ok) fail(signedTxRes.error);

    const signedTx = signedTxRes.value;
    const executionResult = await safe.executeTransaction(
      {
        tx: signedTx,
        options: { gasLimit: "400000" },
      },
      undefined,
      {
        safeAddress,
        connection: CONNECTION,
      }
    );

    if (!executionResult.ok) fail(executionResult.error);

    const balanceAfter = await ethers.getBalance({
      address: safeAddress,
      blockTag: null,
      connection: CONNECTION,
    });
    if (!balanceAfter.ok) fail(balanceAfter.error);

    expect(balanceAfter.value).toEqual("750000000000000000");
  });
});
