import * as App from "./types/wrap";
import {
  CONNECTION,
  SupportedContracts,
  VERSIONS,
  deployTestSafe,
  getClient,
  initInfra,
  setUpContracts,
  setupAccounts,
  stopInfra,
} from "./utils";
jest.setTimeout(500000);

const safeVersion = process.env.SAFE_VERSION as VERSIONS;

describe("Transactions execution", () => {
  let contracts: SupportedContracts<string>;
  let safeAddress: string;

  const accounts = setupAccounts();
  let client = getClient();
  let safe = new App.Safe(client);
  let ethers = new App.Ethers(client);

  beforeAll(async () => {
    await initInfra();
    contracts = await setUpContracts(ethers);
    safeAddress = await deployTestSafe(safe, {
      safeAccountConfig: {
        owners: [accounts[0].address],
        threshold: 1,
      },
      connection: CONNECTION,
      customContractAddresses: {
        safeFactoryContract: contracts.SAFE![safeVersion],
        proxyFactoryContract: contracts.FACTORY![safeVersion],
      },
    });

    // Update client's environment with new safe address
    client = getClient({
      env: {
        safeAddress,
        connection: CONNECTION,
      },
    });
    safe = new App.Safe(client);
  });

  afterAll(async () => {
    await stopInfra();
  });

  describe("Sign transaction hash method", () => {
    it("Should sign a transaction hash with the current signer", async () => {
      const transactionData = {
        to: accounts[0].address,
        value: "500000000000000000", // 0.5 ETH,
        data: "0x",
      };
      const transaction = await safe.createTransaction({
        tx: transactionData,
      });

      if (!transaction.ok) throw transaction.error;

      const transactionHash = await safe.getTransactionHash({
        tx: transaction.value.data,
      });

      if (!transactionHash.ok) throw transactionHash.error;
      const signature = await safe.signTransactionHash({
        hash: transactionHash.value,
      });

      if (!signature.ok) throw signature.error;
      expect(signature.value).toBeTruthy();
      expect(signature.value.data.length).toEqual(132);
    });
  });

  describe("Sign transaction method", () => {
    it("Should add the signature of the current signer", async () => {
      const transactionData = {
        to: accounts[0].address,
        value: "500000000000000000", // 0.5 ETH,
        data: "0x",
      };
      const transaction = await safe.createTransaction({
        tx: transactionData,
      });
      if (!transaction.ok) fail(transaction.error);

      expect(transaction.value).toBeTruthy();
      expect(transaction.value.signatures?.size).toEqual(0);

      const signedTransaction = await safe.addSignature({
        tx: transaction.value,
      });

      if (!signedTransaction.ok) throw signedTransaction.error;
      expect(signedTransaction.value.signatures?.size).toEqual(1);
    });
    it("Should ignore duplicated signatures", async () => {
      const transactionData = {
        to: accounts[0].address,
        value: "500000000000000000", // 0.5 ETH,
        data: "0x",
      };
      const transaction = await safe.createTransaction({
        tx: transactionData,
      });
      if (!transaction.ok) fail(transaction.error);

      expect(transaction.value).toBeTruthy();
      expect(transaction.value.signatures?.size).toEqual(0);

      const signedTransaction = await safe.addSignature({
        tx: transaction.value,
      });

      if (!signedTransaction.ok) throw signedTransaction.error;
      expect(signedTransaction.value.signatures?.size).toEqual(1);

      // Try to sign again
      const secondSignedTransaction = await safe.addSignature({
        tx: signedTransaction.value,
      });

      if (!secondSignedTransaction.ok) throw secondSignedTransaction.error;
      expect(secondSignedTransaction.value.signatures?.size).toEqual(1);
    });
    it("Should faild if sugnature is added by an account that is not an owner", async () => {
      const transactionData = {
        to: accounts[0].address,
        value: "500000000000000000", // 0.5 ETH,
        data: "0x",
      };
      const transaction = await safe.createTransaction(
        {
          tx: transactionData,
        },
        getClient({ signer: accounts[1].signer })
      );
      if (!transaction.ok) fail(transaction.error);

      expect(transaction.value).toBeTruthy();
      expect(transaction.value.signatures?.size).toEqual(0);

      const signedTransaction = await safe.addSignature(
        {
          tx: transaction.value,
        },
        getClient({ signer: accounts[1].signer }),
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      if (signedTransaction.ok) throw "Sign transaction should fail";
      expect(signedTransaction.error!.message).toContain(
        "Transactions can only be signed by Safe owners"
      );
    });
  });
});
