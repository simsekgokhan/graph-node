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

// Test that non-terminating handlers are killed by timeout.
export function loop(): void {
    while (true) {}
}

export function rabbit_hole(): void {
    rabbit_hole()
}
