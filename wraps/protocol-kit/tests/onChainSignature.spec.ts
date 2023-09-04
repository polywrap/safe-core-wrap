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
        owners: accounts.map((a) => a.address).slice(0, -1),
        threshold: 2,
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

  describe("Approve transaction hash method", () => {
    it("should fail if a transaction hash is approved by an account that is not an owner", async () => {
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

      const signature = await safe.approveTransactionHash(
        {
          hash: transactionHash.value,
        },
        // Pass client with different signer when approving hash
        getClient({ signer: accounts[2].signer }),
        {
          safeAddress,
          connection: CONNECTION,
        }
      );
      if (signature.ok) throw "Approve transaction hash should have failed";
      expect(signature.error?.toString()).toContain(
        "Transaction hashes can only be approved by Safe owners"
      );
    });

    it("should approve the transaction hash", async () => {
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

      const signatureTx = await safe.approveTransactionHash({
        hash: transactionHash.value,
      });
      if (!signatureTx.ok) throw signatureTx.error;
      expect(signatureTx.value.transactionHash).toBeTruthy();
      expect(signatureTx.value.logs.length).toBeGreaterThan(0);
      expect(signatureTx.value.to.toLowerCase()).toEqual(
        safeAddress.toLowerCase()
      );

      const approvedHashes = await safe.approvedHashes({
        address: safeAddress,
        hash: transactionHash.value,
        ownerAddress: accounts[0].address,
      });
      if (!approvedHashes.ok) throw approvedHashes.error;

      expect(approvedHashes.value).toEqual("1");
    });

    it("should ignore duplicated signature", async () => {
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

      const signatureTx = await safe.approveTransactionHash({
        hash: transactionHash.value,
      });
      if (!signatureTx.ok) throw signatureTx.error;
      expect(signatureTx.value.transactionHash).toBeTruthy();
      expect(signatureTx.value.logs.length).toBeGreaterThan(0);
      expect(signatureTx.value.to.toLowerCase()).toEqual(
        safeAddress.toLowerCase()
      );

      const approvedHashes = await safe.approvedHashes({
        address: safeAddress,
        hash: transactionHash.value,
        ownerAddress: accounts[0].address,
      });
      if (!approvedHashes.ok) throw approvedHashes.error;

      expect(approvedHashes.value).toEqual("1");
      const secondApproval = await safe.approveTransactionHash({
        hash: transactionHash.value,
      });
      if (!secondApproval.ok) throw secondApproval.error;

      const refetchApprovedHashes = await safe.approvedHashes({
        address: safeAddress,
        hash: transactionHash.value,
        ownerAddress: accounts[0].address,
      });
      if (!refetchApprovedHashes.ok) throw refetchApprovedHashes.error;

      expect(refetchApprovedHashes.value).toEqual("1");
    });
  });

  describe("Get owners who approved tx method", () => {
    it("Should return the list of owners who approved a transaction hash", async () => {
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

      const hash = transactionHash.value;
      const signatureTx = await safe.approveTransactionHash({
        hash,
      });
      if (!signatureTx.ok) throw signatureTx.error;

      const secondSignatureTx = await safe.approveTransactionHash(
        {
          hash,
        },
        getClient({ signer: accounts[1].signer }),
        {
          safeAddress,
          connection: CONNECTION,
        }
      );
      if (!secondSignatureTx.ok) throw secondSignatureTx.error;

      const approvers = await safe.getOwnersWhoApprovedTx({ hash });
      if (!approvers.ok) throw approvers.error;

      expect(approvers.value.length).toEqual(2);
      expect(JSON.stringify(approvers.value)).toBe(
        JSON.stringify(
          accounts.map((a) => a.address.toLowerCase()).slice(0, -1)
        )
      );
    });
  });
});
