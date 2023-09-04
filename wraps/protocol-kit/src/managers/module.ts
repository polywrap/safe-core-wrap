import {
  Args_encodeDisableModuleData,
  Args_encodeEnableModuleData,
  Args_getModules,
  Args_isModuleEnabled,
  Env,
  Ethers_Module,
} from "../wrap";
import {
  validateModuleAddress,
  validateModuleIsNotEnabled,
  validateModuleIsEnabledAndGetPrev,
} from "../utils/validation";

export function getModules(_: Args_getModules, env: Env): string[] {
  const resp = Ethers_Module.callContractView({
    address: env.safeAddress,
    method:
      "function getModulesPaginated(address,uint256) external view returns (address[] memory,address)",
    args: ["0x0000000000000000000000000000000000000001", "15"],
    connection: env.connection,
  }).unwrap();
  // TODO; rewrite to json
  const comma = resp.lastIndexOf(",");
  const arr = resp.substring(0, comma);
  if (arr.includes(",")) {
    return arr.split(",");
  } else {
    return [];
  }
}

export function isModuleEnabled(args: Args_isModuleEnabled, env: Env): boolean {
  const resp = Ethers_Module.callContractView({
    address: env.safeAddress,
    method:
      "function isModuleEnabled(address module) public view returns (bool)",
    args: [args.moduleAddress],
    connection: env.connection,
  }).unwrap();
  if (resp === "true") {
    return true;
  } else {
    return false;
  }
}

export function encodeEnableModuleData(
  args: Args_encodeEnableModuleData,
  env: Env
): string {
  validateModuleAddress(args.moduleAddress);
  validateModuleIsNotEnabled(args.moduleAddress, getModules({}, env));
  const result = Ethers_Module.encodeFunction({
    method: "function enableModule(address module) public",
    args: [args.moduleAddress],
  });
  return result.unwrap();
}

export function encodeDisableModuleData(
  args: Args_encodeDisableModuleData,
  env: Env
): string {
  validateModuleAddress(args.moduleAddress);
  const prevModuleAddress = validateModuleIsEnabledAndGetPrev(
    args.moduleAddress,
    getModules({}, env)
  );
  const result = Ethers_Module.encodeFunction({
    method: "function disableModule(address prevModule, address module) public",
    args: [prevModuleAddress, args.moduleAddress],
  });
  return result.unwrap();
}
