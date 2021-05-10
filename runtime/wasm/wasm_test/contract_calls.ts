enum IndexForAscTypeId {
  STRING = 0,
}

export function id_of_type(type_id_index: IndexForAscTypeId): usize {
  switch (type_id_index) {
    case IndexForAscTypeId.STRING:
      return idof<String>();
    default:
      return 0;
  }
}

export function allocate(n: usize): usize {
  return __alloc(n);
}

type Address = Uint8Array;

export declare namespace ethereum {
  function call(call: Address): Array<Address> | null
}

export function callContract(address: Address): void {
  ethereum.call(address)
}
