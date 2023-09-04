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
  const client = getClient();
  const safe = new App.Safe(client);

  beforeAll(async () => {
    await initInfra();
    contracts = await setUpContracts(new App.Ethers(client));
  });

  afterAll(async () => {
    await stopInfra();
  });

  describe("Deployment with custom contracts addresses", () => {
    it("Should deploy a new safe and predict address with default salt", async () => {
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

    it("Should deploy a new safe and predict address with given salt", async () => {
      const deploymentInput = {
        input: {
          safeAccountConfig: {
            owners: ["0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82"],
            threshold: 1,
          },
          safeDeploymentConfig: {
            saltNonce: "3074",
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
          "Threshold cannot exceed owner count"
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

  describe("Prediction of safe address", () => {
    it("Should fail if the threshold is lower than 0", async () => {
      const deploymentInput = {
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
      };

      const predictSafeResp = await safe.predictSafeAddress(deploymentInput);

      expect(predictSafeResp.ok).toEqual(false);
      if (!predictSafeResp.ok) {
        expect(predictSafeResp.error?.toString()).toMatch(
          "unsigned integer cannot be negative"
        );
      }
    });

    it("Should fail if the threshold is higher than the owners", async () => {
      const deploymentInput = {
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
      };

      const predictSafeResp = await safe.predictSafeAddress(deploymentInput);

      expect(predictSafeResp.ok).toEqual(false);
      if (!predictSafeResp.ok) {
        expect(predictSafeResp.error?.toString()).toMatch(
          "Threshold cannot exceed owner count"
        );
      }
    });

    it("Should fail if the saltNonce is lower than 0", async () => {
      const deploymentInput = {
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
      };

      const predictSafeResp = await safe.predictSafeAddress(deploymentInput);

      expect(predictSafeResp.ok).toEqual(false);
      if (!predictSafeResp.ok) {
        expect(predictSafeResp.error?.toString()).toMatch(
          "saltNonce must be greater than or equal to 0"
        );
      }
    });
  });
});
