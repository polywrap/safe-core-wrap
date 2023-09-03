import * as App from "../wrap";
import { SALT_NONCE, getClient } from "../utils";

export const deploySafe = async () => {
  const client = getClient();
  const safe = new App.Safe(client);
  const ethers = new App.Ethers(client);
  // Get signer address
  const signerAddress = await ethers.getSignerAddress({});

  if (!signerAddress.ok) throw signerAddress.error;
  console.log(`Signer address: ${signerAddress.value}`);

  const deploymentInput = {
    safeAccountConfig: {
      owners: [signerAddress.value],
      threshold: 1,
    },
    safeDeploymentConfig: {
      saltNonce: SALT_NONCE,
    },
  };

  const expectedSafeAddress = await safe.predictSafeAddress({
    input: deploymentInput,
  });
  if (!expectedSafeAddress.ok) throw expectedSafeAddress.error;
  console.log(`Expected safe address: ${expectedSafeAddress.value}`);

  console.log("Deploying safe...");

  const deploySafe = await safe.predictSafeAddress({
    input: deploymentInput,
  });
  if (!deploySafe.ok) throw deploySafe.error;

  console.log(`Safe deployed to address: ${deploySafe.value}`);
};
