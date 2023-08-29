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

describe("Owner Manager", () => {
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

  it("Should get owners", async () => {
    const owners = await safe.getOwners({ safeAddress });

    if (!owners.ok) throw owners.error;
    expect(JSON.stringify(owners.value)).toEqual(
      JSON.stringify(accounts.map((a) => a.address.toLowerCase()).slice(0, -1))
    );
  });

  it("Should get threshold", async () => {
    const threshold = await safe.getThreshold({ safeAddress });

    if (!threshold.ok) throw threshold.error;
    expect(threshold.value).toEqual(1);
  });

  it("Should encode add owner with threshold data", async () => {
    const data = await safe.encodeAddOwnerWithThresholdData({
      ownerAddress: accounts[2].address,
    });

    if (!data.ok) throw data.error;
    expect(data.value).toBeTruthy();
  });

  it("Should encode remove owner data", async () => {
    const data = await safe.encodeRemoveOwnerData({
      ownerAddress: accounts[1].address,
    });

    if (!data.ok) throw data.error;
    expect(data.value).toBeTruthy();
  });

  it("Should encode swap owner data", async () => {
    const data = await safe.encodeSwapOwnerData({
      oldOwnerAddress: accounts[1].address,
      newOwnerAddress: accounts[2].address,
    });

    if (!data.ok) throw data.error;
    expect(data.value).toBeTruthy();
  });

  it("Should fail to encode swap owner data because its passing the same address", async () => {
    const data = await safe.encodeSwapOwnerData({
      oldOwnerAddress: accounts[1].address,
      newOwnerAddress: accounts[1].address,
    });

    if (!data.ok) {
      expect(data.error?.message.toString()).toContain(
        "Address provided is already an owner"
      );
    }
  });

  it("Should encode change threshold data", async () => {
    const data = await safe.encodeChangeThresholdData({
      threshold: 2,
    });

    if (!data.ok) throw data.error;
    expect(data.value).toBeTruthy();
  });
});
