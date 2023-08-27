const deployments = [
  ["1", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["3", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["4", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["10", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["11", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["12", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["25", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["28", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["42", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["5", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["56", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["69", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["82", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["83", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["100", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["106", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["111", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["122", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["123", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["137", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["246", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["288", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["300", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["336", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["338", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["588", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["592", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["595", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["686", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["787", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["1001", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["1008", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1088", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["1284", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1285", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1287", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1984", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["2001", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["2008", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["4002", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["4918", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["4919", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["7341", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["8217", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["9000", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["9001", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["10000", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["10001", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["11437", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["12357", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["42161", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["42170", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["42220", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["43113", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["43114", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["47805", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["71401", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["73799", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["80001", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["200101", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["200202", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["333999", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["421611", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["421613", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1313161554", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1313161555", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["1666600000", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["1666700000", "0x69f4D1788e39c87893C980c06EdF4b7f686e2938"],
  ["11297108099", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
  ["11297108109", "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552"],
];

export default deployments;