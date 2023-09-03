import * as App from "../wrap";
import { getClient } from "../utils";
import { BigNumber } from "ethers";
const pattern = "0x5afe";
const start = Date.now();
const addressesFound: { predictedSafeAddress: string; saltNonce: number }[] =
  [];

export async function generateSafeAddresses() {
  printFirstMessage();
  const client = getClient();
  const safe = new App.Safe(client);

  let saltNonce = 0;
  let iterations = 0;

  const safeAccountConfig = {
    owners: ["0x0Ce3cC862b26FC643aA8A73D2D30d47EF791941e"],
    threshold: 1,
  };

  while (true) {
    iterations++;

    const deploymentInput = {
      safeAccountConfig,
      safeDeploymentConfig: {
        saltNonce: BigNumber.from(saltNonce).toHexString(),
      },
    };

    const predictedSafeAddress = await safe.predictSafeAddress({
      input: deploymentInput,
    });

    if (!predictedSafeAddress.ok) throw predictedSafeAddress.error;

    printPerfomanceMessage(iterations, saltNonce);

    const isValidAddress = predictedSafeAddress.value.startsWith(pattern);

    if (isValidAddress) {
      printValidAddress(predictedSafeAddress.value, saltNonce);
    }

    saltNonce++;
  }
}

// util functions

// used to print the initial message
function printFirstMessage(): void {
  console.clear();

  console.log(`Searching for a Safe address starting with "${pattern}..." `);

  console.log(" ");
  console.log("Addresses found:  0");
  console.log("------------------------------------------------------------");
}

// used to print the perfomance message
function printPerfomanceMessage(addresses: number, saltNonce: number): void {
  const seconds = (Date.now() - start) / 1000;
  const addressesPerSecond = (addresses / seconds).toFixed(8);

  process.stdout.write(
    `\rSpeed: ${addressesPerSecond} addresses/second   ------  Current saltNonce: ${saltNonce}`
  );
}

// used to print the found addresses
function printValidAddress(validAddress: string, saltNonce: number): void {
  addressesFound.push({
    predictedSafeAddress: validAddress,
    saltNonce,
  });

  console.clear();

  console.log(`Searching for a Safe address starting with "${pattern}..." `);

  console.log(" ");
  console.log("Addresses found:  ", addressesFound.length);
  console.log("------------------------------------------------------------");
  addressesFound.forEach(({ predictedSafeAddress, saltNonce }) => {
    console.log("Safe Address: ", predictedSafeAddress);
    console.log("saltNonce: ", saltNonce);
    console.log("------------------------------------------------------------");
  });
  console.log(" ");
}
