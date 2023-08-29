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

describe("Module Manager", () => {
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

  it("Should get modules", async () => {
    const modules = await safe.getModules({});
    if (!modules.ok) throw modules.error;
    expect(modules.value).toEqual([]);
  });

  it("Should check if module is enabled", async () => {
    const modules = await safe.isModuleEnabled({
      moduleAddress: accounts[2].address,
    });
    if (!modules.ok) throw modules.error;
    expect(modules.value).toBeFalsy();
  });

  // TODO: Update this with a real deployed module
  const moduleAddress = accounts[2].address;
  it("Should encode enable module data", async () => {
    const enableData = await safe.encodeEnableModuleData({ moduleAddress });

    if (!enableData.ok) throw enableData.error;
    expect(enableData.value).not.toBeNull();
  });

  it("Should encode disable module data", async () => {
    const resp = await safe.encodeDisableModuleData({ moduleAddress });
    if (!resp.ok) {
      expect(resp.error?.toString()).toMatch(
        "Module provided is not enabled yet"
      );
    }
  });
});
