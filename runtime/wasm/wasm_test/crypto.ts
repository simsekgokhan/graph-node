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

declare namespace crypto {
    function keccak256(input: Uint8Array): Uint8Array
}

export function hash(input: Uint8Array): Uint8Array {
    return crypto.keccak256(input)
}
