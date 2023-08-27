import * as App from "./types/wrap";
import {
  CONNECTION,
  SupportedContracts,
  VERSIONS,
  getClient,
  initInfra,
  setUpContracts,
  stopInfra,
} from "./utils";

jest.setTimeout(500000);

const safeVersion = process.env.SAFE_VERSION as VERSIONS;

describe("Safe Factory", () => {
  let contracts: SupportedContracts<string>;
  let safe: App.Safe;

  beforeAll(async () => {
    await initInfra();
    const client = await getClient();
    contracts = await setUpContracts(client);
    safe = new App.Safe(client);
  });

  describe("Deployment with custom contracts addresses", () => {
    it("Should deploy a new safe without giving salt", async () => {
      const deploymentResponse = await safe.deploySafe({
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: 1,
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      });

      if (!deploymentResponse.ok) {
        fail(deploymentResponse.error);
      }

      expect(deploymentResponse.ok).toBeTruthy();
      expect(deploymentResponse.value).toMatch("0x");
    });

    it("Should deploy a new safe with given salt", async () => {
      const deploymentResponse = await safe.deploySafe({
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: 1,
          },
          safeDeploymentConfig: {
            saltNonce: Date.now().toString(),
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      });

      if (!deploymentResponse.ok) {
        fail(deploymentResponse.error);
      }

      expect(deploymentResponse.ok).toBeTruthy();
      expect(deploymentResponse.value).toMatch("0x");
    });

    it("Should fail if there are no owners", async () => {
      const deploySafeResponse = await safe.deploySafe({
        input: {
          safeAccountConfig: {
            owners: [],
            threshold: 1,
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      });

      expect(deploySafeResponse.ok).toEqual(false);
      if (!deploySafeResponse.ok) {
        expect(deploySafeResponse.error?.toString()).toMatch(
          "Owner list must have at least one owner"
        );
      }
    });
    it("Should fail if the threshold is lower than 0", async () => {
      const deploySafeResponse = await safe.deploySafe({
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: -1,
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      });

      expect(deploySafeResponse.ok).toEqual(false);
      if (!deploySafeResponse.ok) {
        expect(deploySafeResponse.error?.toString()).toMatch(
          "unsigned integer cannot be negative"
        );
      }
    });

    it("Should fail if the threshold is higher than the owners length", async () => {
      const deploySafeResponse = await safe.deploySafe({
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: 2,
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      });

      expect(deploySafeResponse.ok).toEqual(false);
      if (!deploySafeResponse.ok) {
        expect(deploySafeResponse.error?.toString()).toMatch(
          "Threshold must be lower than or equal to owners length"
        );
      }
    });

    it("Should fail if the saltNonce is lower than 0", async () => {
      const deploySafeResponse = await safe.deploySafe({
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: 1,
          },
          safeDeploymentConfig: {
            saltNonce: "-2",
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      });

      expect(deploySafeResponse.ok).toEqual(false);
      if (!deploySafeResponse.ok) {
        expect(deploySafeResponse.error?.toString()).toMatch(
          "saltNonce must be greater than or equal to 0"
        );
      }
    });
  });

  describe.only("Prediction of safe address", () => {
    it("Should predict a new safe", async () => {
      const deploymentInput = {
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: 1,
          },
          connection: CONNECTION,
          customContractAddresses: {
            safeFactoryContract: contracts.SAFE![safeVersion],
            proxyFactoryContract: contracts.FACTORY![safeVersion],
          },
        },
      };

      const predictSafeResp = await safe.predictSafeAddress(deploymentInput);
      if (!predictSafeResp.ok) throw predictSafeResp.error;
      expect(predictSafeResp.value).not.toBeNull();

      const deploySafeResp = await safe.deploySafe(deploymentInput);
      if (!deploySafeResp.ok) throw deploySafeResp.error;
      expect(deploySafeResp.value).not.toBeNull();

      expect(predictSafeResp.value).toEqual(deploySafeResp.value);
    });
  });

  afterAll(async () => {
    await stopInfra();
  });
});
