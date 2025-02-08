/* In Ariel OS on Cortex-M, the main (ISR) stack is explicitly taken at the beginning of the RAM.
 * That means, all memory from `__sheap` to the end of RAM should be available for the heap.*/
PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
PROVIDE(__eheap = _ram_end);
