MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* Memory for the MAX32625ITKL+ */
  /* Leave the final page open for writing static data */
  FLASH : ORIGIN = 0x00000000, LENGTH = 248K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
