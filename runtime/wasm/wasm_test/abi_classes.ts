enum IndexForAscTypeId {
  STRING = 0,
  ARRAY_BUFFER = 1,
  TYPED_ARRAY = 2,
}

export function id_of_type(type_id_index: IndexForAscTypeId): usize {
  switch (type_id_index) {
    case IndexForAscTypeId.STRING:
      return idof<String>();
    case IndexForAscTypeId.ARRAY_BUFFER:
      return idof<ArrayBuffer>();
    case IndexForAscTypeId.TYPED_ARRAY:
      return idof<Uint8Array>();
    default:
      return 0;
  }
}

export function allocate(n: usize): usize {
  return __alloc(n);
}

// Sequence of 20 `u8`s.
type Address = Uint8Array;

const array_buffer_header_size = 8;

// Clone the address to a new buffer, add 1 to the first and last bytes of the
// address and return the new address.
export function test_address(address: Address): Address {
  let new_address = address.subarray();

  // Add 1 to the first and last byte.
  new_address[0] += 1;
  new_address[address.length - 1] += 1;

  return new_address
}

// Sequence of 32 `u8`s.
type Uint = Uint8Array;

// Clone the Uint to a new buffer, add 1 to the first and last `u8`s and return
// the new Uint.
export function test_uint(address: Uint): Uint {
  let new_address = address.subarray();

  // Add 1 to the first byte.
  new_address[0] += 1;

  return new_address
}

// Return the string repeated twice.
export function repeat_twice(original: string): string {
  return original.repeat(2)
}

// Sequences of `u8`s.
type FixedBytes = Uint8Array;
type Bytes = Uint8Array;

// Concatenate two byte sequences into a new one.
export function concat(bytes1: Bytes, bytes2: FixedBytes): Bytes {
  let concated = new ArrayBuffer(bytes1.byteLength + bytes2.byteLength);
  let concated_offset = changetype<usize>(concated) + array_buffer_header_size;
  let bytes1_start = load<usize>(changetype<usize>(bytes1)) + array_buffer_header_size;
  let bytes2_start = load<usize>(changetype<usize>(bytes2)) + array_buffer_header_size;

  // Move bytes1.
  memory.copy(concated_offset, bytes1_start, bytes1.byteLength);
  concated_offset += bytes1.byteLength

  // Move bytes2.
  memory.copy(concated_offset, bytes2_start, bytes2.byteLength);

  let new_typed_array = new Uint8Array(concated.byteLength);
  store<usize>(changetype<usize>(new_typed_array), changetype<usize>(concated));

  return new_typed_array;
}

// export function test_array(strings: Array<string>): Array<string> {
//   let arr: Array<string> = ["6", "7", "8", "9"];
//   strings.push("5");
//   return strings
// }
enum ValueKind {
    STRING = 0,
    INT = 1,
    BIG_DECIMAL = 2,
    BOOL = 3,
    ARRAY = 4,
    NULL = 5,
    BYTES = 6,
    BIG_INT = 7,
}

// Big enough to fit any pointer or native `this.data`.
type Payload = u64
export class Value {
    kind: ValueKind
    data: Payload
}

export function test_array(strings: Uint8Array): Value {
  // let arr: Array<u8> = [1, 2, 3, 4];
  // let arr: Array<u8> = [6, 7, 8, 9];
  let arr: Uint8Array = new Uint8Array(4);
  arr[0] = 6
  arr[1] = 7
  arr[2] = 8
  arr[3] = 9
  // strings.push(5);// out of bounds :/
  // return strings[4]
  let value = new Value();
  value.kind = ValueKind.INT;
  // value.data = arr[3]
  value.data = strings[3]
  // value.data = strings.length
  // value.data = arr.length
  return value
}

export function byte_array_third_quarter(bytes: Uint8Array): Uint8Array {
  return bytes.subarray(bytes.length * 2/4, bytes.length * 3/4)
}
