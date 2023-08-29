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

describe("Create transaction ", () => {
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

  it("Should create a single transaction with gasPrice=0", async () => {
    const transactionData = {
      to: accounts[0].address,
      value: "500000000000000000", // 0.5 ETH
      data: "0x",
      operation: 0,
      baseGas: "111",
      gasPrice: "0",
      gasToken: "0x333",
      refundReceiver: "0x444",
      nonce: 555,
      safeTxGas: "666",
    };

    const transaction = await safe.createTransaction({
      tx: transactionData,
    });

    if (!transaction.ok) throw transaction.error;

    expect(transaction.value.data).toStrictEqual(transactionData);
  });

  it("Should create a single transaction with gasPrice>0", async () => {
    const transactionData = {
      to: accounts[0].address,
      value: "500000000000000000", // 0.5 ETH
      data: "0x",
      operation: 0,
      baseGas: "111",
      gasPrice: "0",
      gasToken: "0x333",
      refundReceiver: "0x444",
      nonce: 555,
      safeTxGas: "666",
    };

    const transaction = await safe.createTransaction({
      tx: transactionData,
    });
    if (!transaction.ok) throw transaction.error;

    expect(transaction.value.data).toStrictEqual(transactionData);
  });

  it("Should create a single transaction when passing a transaction array with length=1", async () => {
    const transactionData = {
      to: accounts[0].address,
      value: "500000000000000000", // 0.5 ETH
      data: "0x",
    };

    const transaction = await safe.createMultiSendTransaction({
      txs: [transactionData],
      customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
    });
    if (!transaction.ok) throw transaction.error;

    expect(transaction.value.data.to).toEqual(transactionData.to);
    expect(transaction.value.data.value).toEqual(transactionData.value);
    expect(transaction.value.data.data).toEqual(transactionData.data);
  });

  it("Should create a single transaction when passing a transaction array with length=1 and options", async () => {
    const transactionData = {
      to: accounts[0].address,
      value: "500000000000000000", // 0.5 ETH
      data: "0x",
    };

    const options = {
      baseGas: "111",
      gasPrice: "222",
      gasToken: "0x333",
      refundReceiver: "0x444",
      nonce: 2938,
      safeTxGas: "666",
    };
    const transaction = await safe.createMultiSendTransaction({
      txs: [transactionData],
      options,
      customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
    });
    if (!transaction.ok) throw transaction.error;

    expect(transaction.value.data.to).toEqual(transactionData.to);
    expect(transaction.value.data.value).toEqual(transactionData.value);
    expect(transaction.value.data.data).toEqual(transactionData.data);
  });

  it("Should fail when creating a MultiSend transaction passing a transaction array with length=0", async () => {
    const multiSendResult = await safe.createMultiSendTransaction({
      txs: [],
      customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
    });

    expect(multiSendResult.ok).toBe(false);
  });

  it("Should create a MultiSend transaction", async () => {
    const safeTransactionData = {
      to: accounts[0].address,
      value: "500000000000000000", // 0.5 ETH
      data: "0x00",
    };

    const safeTxArray = [safeTransactionData, safeTransactionData];

    const multisendTransaction = await safe.createMultiSendTransaction({
      txs: safeTxArray,
      customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
    });

    if (!multisendTransaction.ok) fail(multisendTransaction.error);
    const multisendTransactionData = multisendTransaction.value.data;

    expect(multisendTransactionData.to).toEqual(
      contracts.MULTISEND![safeVersion]
    );
    expect(multisendTransactionData.data).not.toEqual(safeTransactionData.data);
  });

  it("Should create a MultiSend transaction with options", async () => {
    const [account1] = setupAccounts();

    const safeTransactionData = {
      to: account1.address,
      value: "500000000000000000", // 0.5 ETH
      data: "0x00",
    };

    const safeTxArray = [safeTransactionData, safeTransactionData];

    const options = {
      baseGas: "111",
      gasPrice: "222",
      gasToken: "0x333",
      refundReceiver: "0x444",
      nonce: 3074,
      safeTxGas: "666",
    };

    const multiSendTransaction = await safe.createMultiSendTransaction({
      txs: safeTxArray,
      options,
      customMultiSendContractAddress: contracts.MULTISEND![safeVersion],
    });

    if (!multiSendTransaction.ok) fail(multiSendTransaction.error);
    const multiSendTransactionData = multiSendTransaction.value.data;

    expect(multiSendTransactionData.to).toEqual(
      contracts.MULTISEND![safeVersion]
    );
    expect(multiSendTransactionData.data).not.toEqual(safeTransactionData.data);
    expect(multiSendTransactionData.value).not.toEqual(
      safeTransactionData.value
    );

    expect(multiSendTransactionData.baseGas).toEqual(options.baseGas);
    expect(multiSendTransactionData.gasPrice).toEqual(options.gasPrice);
    expect(multiSendTransactionData.gasToken).toEqual(options.gasToken);
    expect(multiSendTransactionData.refundReceiver).toEqual(
      options.refundReceiver
    );
    expect(multiSendTransactionData.nonce).toEqual(options.nonce);
    expect(multiSendTransactionData.safeTxGas).toEqual(options.safeTxGas);
  });
});
