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

declare namespace dataSource {
    function create(name: string, params: Array<string>): void
}

export function dataSourceCreate(name: string, params: Array<string>): void {
    dataSource.create(name, params)
}
