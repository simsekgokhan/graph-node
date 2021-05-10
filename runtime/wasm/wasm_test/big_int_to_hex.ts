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
    function bigIntToHex(n: Uint8Array): String
}

export function big_int_to_hex(n: Uint8Array): String {
    return typeConversion.bigIntToHex(n)
}
