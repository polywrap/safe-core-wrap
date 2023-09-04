import * as App from "./types/wrap";
import {
  CONNECTION,
  SupportedContracts,
  VERSIONS,
  createTransaction,
  deployTestErc20,
  deployTestSafe,
  fundSafeBalance,
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
  let deploymentInput: Partial<App.Safe_DeploymentInput>;

  const accounts = setupAccounts();
  const client = getClient();
  const safe = new App.Safe(client);
  const ethers = new App.Ethers(client);

  beforeAll(async () => {
    await initInfra();
    contracts = await setUpContracts(ethers);
    deploymentInput = {
      connection: CONNECTION,
      customContractAddresses: {
        safeFactoryContract: contracts.SAFE![safeVersion],
        proxyFactoryContract: contracts.FACTORY![safeVersion],
      },
    };
  });

  afterAll(async () => {
    await stopInfra();
  });

  describe("Should execute a transaction", () => {
    it("with threshold 1", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: [accounts[0].address],
          threshold: 1,
        },
        ...deploymentInput,
      });

      await fundSafeBalance(safeAddress);

      const balanceBefore = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceBefore.ok) fail(balanceBefore.error);

      // @ts-ignore
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

    it("with threshold >1", async () => {
      safeAddress = await deployTestSafe(safe, {
        ...deploymentInput,
        safeAccountConfig: {
          owners: accounts.map((a) => a.address),
          threshold: 2,
        },
      });

      // @ts-ignore
      const transaction = await createTransaction({
        value: "250000000000000000",
      });
      await fundSafeBalance(safeAddress);
      const balanceBefore = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceBefore.ok) fail(balanceBefore.error);

      const signedTxRes = await safe.addSignature(
        { tx: transaction },
        undefined,
        { safeAddress, connection: CONNECTION }
      );

      if (!signedTxRes.ok) fail(signedTxRes.error);

      const txHashRes = await safe.getTransactionHash(
        {
          tx: signedTxRes.value.data,
        },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!txHashRes.ok) fail(txHashRes.error);
      const approveResponse = await safe.approveTransactionHash(
        { hash: txHashRes.value },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!approveResponse.ok) fail(approveResponse.error);

      const executionResult = await safe.executeTransaction(
        { tx: signedTxRes.value },
        getClient({ signer: accounts[2].signer }),
        { safeAddress, connection: CONNECTION }
      );
      expect(executionResult.ok).toBeTruthy();
      const balanceAfter = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceAfter.ok) fail(balanceAfter.error);
      expect(balanceAfter.value).toEqual("750000000000000000");
    });

    it("when is not submitted by owner", async () => {
      safeAddress = await deployTestSafe(safe, {
        ...deploymentInput,
        safeAccountConfig: {
          owners: accounts.map((a) => a.address).slice(0, -1),
          threshold: 2,
        },
      });
      // @ts-ignore
      const transaction = await createTransaction({
        value: "250000000000000000",
      });
      await fundSafeBalance(safeAddress);
      const balanceBefore = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceBefore.ok) fail(balanceBefore.error);
      const signedTxRes = await safe.addSignature(
        { tx: transaction },
        undefined,
        { safeAddress, connection: CONNECTION }
      );

      if (!signedTxRes.ok) fail(signedTxRes.error);

      const txHashRes = await safe.getTransactionHash(
        {
          tx: signedTxRes.value.data,
        },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!txHashRes.ok) fail(txHashRes.error);
      const approveResponse = await safe.approveTransactionHash(
        { hash: txHashRes.value },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!approveResponse.ok) fail(approveResponse.error);

      const executionResult = await safe.executeTransaction(
        { tx: signedTxRes.value },
        getClient({ signer: accounts[2].signer }),
        { safeAddress, connection: CONNECTION }
      );
      expect(executionResult.ok).toBeTruthy();
      const balanceAfter = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceAfter.ok) fail(balanceAfter.error);
      expect(balanceAfter.value).toEqual("750000000000000000");
    });

    it("with options: { gasLimit }", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: [accounts[0].address],
          threshold: 1,
        },
        ...deploymentInput,
      });

      await fundSafeBalance(safeAddress);

      const transaction = await createTransaction();
      const executionResult = await safe.executeTransaction(
        {
          tx: transaction,
          options: { gasLimit: "400000" },
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      if (!executionResult.ok) fail(executionResult.error);
      expect(executionResult.ok).toBeTruthy();
    });

    it("with options: { gasLimit, gasPrice }", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: [accounts[0].address],
          threshold: 1,
        },
        ...deploymentInput,
      });

      await fundSafeBalance(safeAddress);

      const transaction = await createTransaction();
      const options = { gasLimit: "654321", gasPrice: "170000000" };
      const executionResult = await safe.executeTransaction(
        {
          tx: transaction,
          options,
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      if (!executionResult.ok) fail(executionResult.error);
      expect(executionResult.ok).toBeTruthy();
      const tx = await ethers.getTransaction({
        hash: executionResult.value.transactionHash,
      });
      if (!tx.ok) fail(tx.error);
      expect(tx.value.gasPrice).toBe(options.gasPrice);
      expect(tx.value.gasLimit).toBe(options.gasLimit);
    });

    it("with options: { maxFeePerGas, maxPriorityFeePerGas }", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: [accounts[0].address],
          threshold: 1,
        },
        ...deploymentInput,
      });

      await fundSafeBalance(safeAddress);

      const transaction = await createTransaction();
      const options = { maxPriorityFeePerGas: "1", maxFeePerGas: "200000000" };
      const executionResult = await safe.executeTransaction(
        {
          tx: transaction,
          options,
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      if (!executionResult.ok) fail(executionResult.error);
      expect(executionResult.ok).toBeTruthy();
      const tx = await ethers.getTransaction({
        hash: executionResult.value.transactionHash,
      });
      if (!tx.ok) fail(tx.error);

      expect(options.maxFeePerGas).toBe(tx.value.maxFeePerGas);
      expect(options.maxPriorityFeePerGas).toBe(tx.value.maxPriorityFeePerGas);
    });
  });

  describe("Should fail", () => {
    it("if safe does not have enough funds", async () => {
      const transaction = await createTransaction();
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: [accounts[0].address],
          threshold: 1,
        },
        ...deploymentInput,
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
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      executionResult.ok
        ? fail()
        : expect(executionResult.error!.toString()).toContain(
            "Not enough Ether funds"
          );
    });
    it("if user tries to execute a transaction with options: { gas, gasLimit }", async () => {
      const transaction = await createTransaction();
      await fundSafeBalance(safeAddress);
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
          options: { gas: "1000", gasLimit: "1000" },
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      executionResult.ok
        ? fail()
        : expect(executionResult.error!.toString()).toContain(
            "Cannot specify gas and gasLimit together in transaction options"
          );
    });
    it("if there are not enough signatures (1 missing)", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: [accounts[0].address, accounts[1].address],
          threshold: 2,
        },
        ...deploymentInput,
      });
      const transaction = await createTransaction();
      await fundSafeBalance(safeAddress);

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
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      executionResult.ok
        ? fail()
        : expect(executionResult.error!.toString()).toContain(
            `There is 1 signature missing`
          );
    });
    it("if there are not enough signatures (>1 missing)", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: accounts.map((a) => a.address),
          threshold: 3,
        },
        ...deploymentInput,
      });
      const transaction = await createTransaction();
      await fundSafeBalance(safeAddress);

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
        },
        undefined,
        {
          safeAddress,
          connection: CONNECTION,
        }
      );

      executionResult.ok
        ? fail()
        : expect(executionResult.error!.toString()).toContain(
            `There are 2 signatures missing`
          );
    });
  });

  describe("Should execute a multisend transaction", () => {
    it("with threshold > 1", async () => {
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: accounts.map((a) => a.address),
          threshold: 2,
        },
        ...deploymentInput,
      });

      await fundSafeBalance(safeAddress, "5000000000000000000"); // 5 eth

      const multisendTxData: Array<App.Safe_SafeTransactionDataPartial> = [
        { to: accounts[2].address, value: "2000000000000000000", data: "0x" },
        { to: accounts[1].address, value: "1000000000000000000", data: "0x" },
      ];

      const balanceBefore = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceBefore.ok) fail(balanceBefore.error);

      const createMultisendResponse = await safe.createMultiSendTransaction(
        {
          txs: multisendTxData,
          customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
        },
        undefined,
        { safeAddress, connection: CONNECTION }
      );

      if (!createMultisendResponse.ok) fail(createMultisendResponse.error);

      const signedTxRes = await safe.addSignature(
        { tx: createMultisendResponse.value },
        undefined,
        { safeAddress, connection: CONNECTION }
      );
      if (!signedTxRes.ok) fail(signedTxRes.error);
      const txHashRes = await safe.getTransactionHash(
        {
          tx: signedTxRes.value.data,
        },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!txHashRes.ok) fail(txHashRes.error);
      const approveResponse = await safe.approveTransactionHash(
        { hash: txHashRes.value },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!approveResponse.ok) fail(approveResponse.error);

      const executionResult = await safe.executeTransaction(
        { tx: signedTxRes.value },
        getClient({ signer: accounts[2].signer }),
        { safeAddress, connection: CONNECTION }
      );
      expect(executionResult.ok).toBeTruthy();
      const balanceAfter = await ethers.getBalance({
        address: safeAddress,
        blockTag: null,
        connection: CONNECTION,
      });
      if (!balanceAfter.ok) fail(balanceAfter.error);
      expect(balanceAfter.value).toEqual("2000000000000000000");
    });

    it("with contract calls and threshold >1", async () => {
      const erc20Address = await deployTestErc20(ethers);
      safeAddress = await deployTestSafe(safe, {
        safeAccountConfig: {
          owners: accounts.map((a) => a.address),
          threshold: 2,
        },
        ...deploymentInput,
      });

      const mint = await ethers.callContractMethodAndWait({
        address: erc20Address,
        method: "function mint(address account,uint256 amount)",
        args: [safeAddress, "5000000000000000000"],
      });

      if (!mint.ok) fail(mint.error);

      const safeInitialErc20Balance = await ethers.callContractView({
        address: erc20Address,
        method: "function balanceOf(address) returns (uint256)",
        args: [safeAddress],
      });
      if (!safeInitialErc20Balance.ok) fail(safeInitialErc20Balance);

      expect(safeInitialErc20Balance.value).toBe("5000000000000000000");

      const transferAccountOneEncoded = await ethers.encodeFunction({
        method: "function transfer(address, uint256)",
        args: [
          accounts[1].address,
          "2000000000000000000", // 2 ERC20
        ],
      });

      if (!transferAccountOneEncoded.ok) throw transferAccountOneEncoded.error;

      const transferAccountTwoEncoded = await ethers.encodeFunction({
        method: "function transfer(address, uint256)",
        args: [
          accounts[2].address,
          "2000000000000000000", // 2 ERC20
        ],
      });
      if (!transferAccountTwoEncoded.ok) throw transferAccountTwoEncoded.error;

      const transactions: Array<App.Safe_SafeTransactionDataPartial> = [
        {
          to: erc20Address,
          value: "0",
          data: transferAccountOneEncoded.value,
        },
        {
          to: erc20Address,
          value: "0",
          data: transferAccountTwoEncoded.value,
        },
      ];

      const createMultisendResponse = await safe.createMultiSendTransaction(
        {
          txs: transactions,
          customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
        },
        undefined,
        { safeAddress, connection: CONNECTION }
      );

      if (!createMultisendResponse.ok) throw createMultisendResponse.error;

      const signedTxRes = await safe.addSignature(
        { tx: createMultisendResponse.value },
        undefined,
        { safeAddress, connection: CONNECTION }
      );
      if (!signedTxRes.ok) fail(signedTxRes.error);
      const txHashRes = await safe.getTransactionHash(
        {
          tx: signedTxRes.value.data,
        },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!txHashRes.ok) fail(txHashRes.error);
      const approveResponse = await safe.approveTransactionHash(
        { hash: txHashRes.value },
        getClient({ signer: accounts[1].signer }),
        { safeAddress, connection: CONNECTION }
      );
      if (!approveResponse.ok) fail(approveResponse.error);

      const executionResult = await safe.executeTransaction(
        { tx: signedTxRes.value },
        getClient({ signer: accounts[2].signer }),
        { safeAddress, connection: CONNECTION }
      );
      expect(executionResult.ok).toBeTruthy();

      const safeFinalErc20Balance = await ethers.callContractView({
        address: erc20Address,
        method: "function balanceOf(address) returns (uint256)",
        args: [safeAddress],
      });
      if (!safeFinalErc20Balance.ok) fail(safeFinalErc20Balance);

      expect(safeFinalErc20Balance.value).toBe("1000000000000000000");

      const accountOneFinalBanalce = await ethers.callContractView({
        address: erc20Address,
        method: "function balanceOf(address) returns (uint256)",
        args: [accounts[1].address],
      });
      if (!accountOneFinalBanalce.ok) fail(accountOneFinalBanalce);

      expect(accountOneFinalBanalce.value).toBe("2000000000000000000");
      const accountTwoFileBalance = await ethers.callContractView({
        address: erc20Address,
        method: "function balanceOf(address) returns (uint256)",
        args: [accounts[2].address],
      });
      if (!accountTwoFileBalance.ok) fail(accountTwoFileBalance);

      expect(accountTwoFileBalance.value).toBe("2000000000000000000");
    });
  });
});
