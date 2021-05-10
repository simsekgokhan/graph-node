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

declare namespace typeConversion {
    function bytesToBase58(n: Uint8Array): string
}

export function bytes_to_base58(n: Uint8Array): string {
    return typeConversion.bytesToBase58(n)
}
