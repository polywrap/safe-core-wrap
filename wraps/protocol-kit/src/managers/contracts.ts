import { Args_getVersion, Ethers_Module } from "../wrap";

export function getVersion(args: Args_getVersion): string {
  const version = Ethers_Module.callContractView({
    address: args.address,
    method: "function VERSION() public view returns (string)",
    args: [],
    connection: args.connection,
  }).unwrap();

  return version;
}
