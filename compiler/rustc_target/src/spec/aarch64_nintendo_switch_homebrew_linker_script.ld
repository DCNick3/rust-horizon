OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

PHDRS
{
  text PT_LOAD FLAGS(5);
  rodata PT_LOAD FLAGS(4);
  data PT_LOAD FLAGS(6);
  bss PT_LOAD FLAGS(6);
  tls PT_TLS;
  dynamic PT_DYNAMIC;
}

SECTIONS
{
  . = 0;

  .text : ALIGN(0x1000) {
    HIDDEN(__text_start = .);
    KEEP(*(.text.rrt0))
    /* KEEP(*(.text.jmp)) the proposed linker script was different..  */

    . = 0x80;

    *(.text .text.*)
    *(.plt .plt.*)
  }

  /* Read-only sections */

  . = ALIGN(0x1000);

  .module_name : { *(.module_name) } :rodata

  .rodata : {
    *(.rodata .rodata.*)
  } :rodata

  /* Init values for TLS */
  .tdata ALIGN(8) :
  {
    . = ALIGN(8);
    HIDDEN(__tls_image_start = .);
    /* initialized */
    *(.tdata .tdata.* .gnu.linkonce.td.*)
    /* uninitialized */
    *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon)
    . = ALIGN(8);
    HIDDEN(__tls_image_end = .);
  } :rodata :tls /* we need to emit those into both into rodata and into tls. rodata is for runtime (to copy stuff into thread-local region), tls is for debuggers(?) and to make linker happy (fails because it emits TLS symbols w/o TLS section) */

  .hash : { *(.hash) } :rodata
  .dynsym : { *(.dynsym .dynsym.*) }
  .dynstr : { *(.dynstr .dynstr.*) }
  .rela.dyn : { *(.rela.dyn) }

  .eh_frame : {
    HIDDEN(__eh_frame_start = .);
    *(.eh_frame .eh_frame.*)
    HIDDEN(__eh_frame_end = .);
  }

  .eh_frame_hdr : {
    HIDDEN(__eh_frame_hdr_start = .);
    *(.eh_frame_hdr .eh_frame_hdr.*)
    HIDDEN(__eh_frame_hdr_end = .);
  }

  .dynamic : {
    HIDDEN(__dynamic_start = .);
    *(.dynamic)
  }

  /* Read-write sections */

   . = ALIGN(0x1000);

  .data : {
    *(.data .data.*)
    *(.got .got.*)
    *(.got.plt .got.plt.*)
  } :data

  /* BSS section */
  . = ALIGN(0x1000);

  .bss : {
    HIDDEN(__bss_start = .);

    *(.bss .bss.*)
    *(COMMON)

    /* Reserve space for the TLS segment of the main thread */
    . = ALIGN(8);
    HIDDEN(__main_thread_tls_start = .);
    . += __tls_image_end - __tls_image_start;
    HIDDEN(__main_thread_tls_end = .);

    . = ALIGN(8);

    HIDDEN(__bss_end = .);
  } :bss

}
