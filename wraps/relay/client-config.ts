import { IClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { config } from "dotenv";

config();

export function configure(builder: IClientConfigBuilder): IClientConfigBuilder {
  if (!process.env.RELAYER_API_KEY) {
    throw new Error(
      "You must define a relayer api key in the .env file. See .example.env"
    );
  }

  return (
    builder
      .addDefaults()
      .addRedirect(
        "wrap://ens/gelato.wraps.eth:relayer@0.0.1",
        "wrap://fs/../../../../polywrap/gelato-relay-polywrap/build"
      )
  );
}
