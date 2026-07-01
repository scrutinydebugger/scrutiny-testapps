import argparse
import socket
import sys
import logging
from pathlib import Path
import os
import binascii

from elftools.elf.elffile import ELFFile
logging.basicConfig(level=logging.INFO, format='[%(levelname)s] %(message)s')
logger = logging.getLogger("memdump")


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(
        description="Dump ELF sections from QEMU memory via the monitor socket."
    )
    p.add_argument("elf", type=Path, help="Path to the ELF file")
    p.add_argument("sections", nargs="+", help="Section names to dump (e.g. .data .bss)")
    p.add_argument(
        "--socket",
        default=None,
        required=True,
        metavar="SOCK",
        help="QEMU monitor Unix socket path (overrides QEMU_MONITOR_SOCK env var)",
    )
    p.add_argument(
        "--outdir",
        type=Path,
        default=Path("memdump"),
        metavar="DIR",
        help="Directory where dumps are written (default: ./memdump)",
    )
    return p.parse_args()


def find_sections(elf_path: Path, names: list[str]) -> dict[str, tuple[int, int]]:
    """Return {name: (vma, size)} for each requested section found in the ELF."""
    found = {}
    with elf_path.open("rb") as f:
        elf = ELFFile(f)
        for name in names:
            sec = elf.get_section_by_name(name)
            if sec is None:
                logger.warning(f"{name}: not found in ELF, skipping")
                continue
            vma = sec["sh_addr"]
            size = sec["sh_size"]
            if size == 0:
                logger.warning(f"{name}: size is 0, skipping")
                continue
            found[name] = (vma, size)
    return found


QEMU_PROMPT = b"(qemu) "


def _read_until_prompt(s: socket.socket, timeout: float) -> bytes:
    """Read from the socket until the QEMU prompt appears or timeout expires."""
    s.settimeout(timeout)
    buf = b""
    try:
        while QEMU_PROMPT not in buf:
            chunk = s.recv(4096)
            if not chunk:
                break
            buf += chunk
    except TimeoutError:
        pass
    return buf


def monitor_cmd(sock_path: str, cmd: str, timeout: float = 10.0) -> None:
    """Connect to the QEMU monitor socket, drain the banner, send cmd, then close."""
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(sock_path)
    try:
        _read_until_prompt(s, timeout)          # drain banner + initial prompt
        s.sendall((cmd + "\n").encode())
        _read_until_prompt(s, timeout)          # wait for command completion
    finally:
        s.close()

def bin_to_hex(addr:int, infile:Path, outfile:Path) -> None:
    with open(infile, 'rb') as f:
        data = f.read()

    with open(outfile, 'a') as f:
        while len(data) > 0:
            line = data[:16]
            data = data[len(line):]
            f.write(f"0x{addr:08X}: {binascii.hexlify(line).decode('utf8')}\n")
            addr+=len(line)

def _delete_or_die(file:Path):
    if os.path.isfile(file):
        logger.debug(f"{file} already exist. Deleting")
        try:
            os.unlink(file)
        except OSError:
            raise RuntimeError(f"Cannot delete {file}")


def main() -> None:
    args = parse_args()

    if not args.elf.is_file():
        logger.error(f"error: ELF not found: {args.elf}")
        sys.exit(1)

    logger.info(f"ELF:    {args.elf}")
    logger.info(f"Socket: {args.socket}")

    sections = find_sections(args.elf, args.sections)
    if not sections:
        logger.warning('No sections to dump.')
    else:
        args.outdir.mkdir(parents=True, exist_ok=True)
        outfile_memdump = args.outdir / f"{Path(args.elf).stem}.memdump"
        _delete_or_die(outfile_memdump)

        for name, (vma, size) in sections.items():
            outfile_bin = args.outdir / f"{name.lstrip('.')}.bin"
            _delete_or_die(outfile_bin)
            logger.info(f"{name}  addr=0x{vma:08x}  size=0x{size:x} ({size} B) -> {outfile_bin}")
            
            monitor_cmd(args.socket, f"memsave 0x{vma:x} 0x{size:x} {outfile_bin}")
            bin_to_hex(vma, outfile_bin, outfile_memdump)
            try:
                os.remove(outfile_bin)
            except OSError as e:
                logger.warning(f"Cannot delete {outfile_bin}. {e}")

    logger.info(f"Done")


if __name__ == "__main__":
    main()
