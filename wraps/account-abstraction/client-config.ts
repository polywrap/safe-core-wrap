import { IWrapPackage } from "@polywrap/client-js";
import {
  ethereumProviderPlugin,
  Connection,
  Connections,
} from "@cbrazon/ethereum-provider-js";
import { providers } from "@polywrap/test-env-js";
import { IClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { dateTimePlugin } from "@polywrap/datetime-plugin-js";
import { config } from "dotenv";
import { Wallet } from "ethers";

config();

export function configure(builder: IClientConfigBuilder): IClientConfigBuilder {
  if (!process.env.PRIVATE_KEY) {
    throw new Error(
      "You must define a private key in the .env file. See .example.env"
    );
  }

  if (!process.env.RPC_URL) {
    throw new Error(
      "You must define a RPC URL in the .env file. See .example.env"
    );
  }

  return (
    builder
      .addDefaults()
      .addPackages({
        "wrap://ens/wraps.eth:ethereum-provider@1.1.0": ethereumProviderPlugin({
          connections: new Connections({
            networks: {
              goerli: new Connection({
                provider: process.env.RPC_URL as string,
                signer: new Wallet(process.env.PRIVATE_KEY as string),
              }),
              testnet: new Connection({
                provider: providers.ethereum
              })
            }
          }),
        }),
        "wrap://ens/datetime.polywrap.eth": dateTimePlugin({}) as IWrapPackage,
      })
      .addInterfaceImplementation(
        "wrap://ens/wraps.eth:ethereum-provider@1.1.0",
        "wrap://ens/wraps.eth:ethereum-provider@1.1.0"
      )
      // @TODO(cbrzn): Remove this once the ENS text record content hash has been updated
      .addRedirect(
        "ens/wraps.eth:ethereum@1.1.0",
        "wrap://ipfs/QmbnAG8iCdVMPQK8tQ5qqFwLKjaLF8BUuuLYiozj7mLF8Y"
      )
      .addRedirect(
        "wrap://ens/safe.wraps.eth:contracts@0.0.1",
        "wrap://fs/../../../safe-contracts-wrapper/packages/safe-contracts-wrapper/build"
      )
      .addRedirect(
        "wrap://ens/safe.wraps.eth:factory@0.0.1",
        "wrap://fs/../../../safe-contracts-wrapper/packages/safe-factory-wrapper/build"
      )
      .addRedirect(
        "wrap://ens/safe.wraps.eth:manager@0.0.1",
        "wrap://fs/../../../safe-contracts-wrapper/packages/safe-managers-wrapper/build"
      )
  );
}
