import * as BufferLayout from '@solana/buffer-layout';

// Define the layout for an array of numbers
const NUMBERS_LAYOUT = BufferLayout.seq(
  BufferLayout.u8(), // Define each element as an unsigned 8-bit integer
  6 // Length of the array
);

export async function createNumbersInstruction(numbers: Uint8Array): Promise<Buffer> {
  // Convert Uint8Array to number[]
  const numberArray = Array.from(numbers);

  // Create a buffer with the appropriate size
  const buffer = Buffer.alloc(NUMBERS_LAYOUT.span);
  
  // Encode the numbers array into the buffer using the layout
  NUMBERS_LAYOUT.encode(numberArray, buffer);
  
  // Log the serialized buffer (for debugging purposes)
  console.log('Serialized Buffer:', buffer);

  return buffer;
}