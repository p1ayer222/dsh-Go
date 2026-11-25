#!/usr/bin/env python3
"""Run pymobiledevice3 on WSL2 against the Windows Apple Mobile Device Service.

WSL cannot complete usbmuxd ``SavePairRecord`` (mux result 183). Pairing still
writes ``~/.local/share/pymobiledevice3/<udid>.plist`` and the device trusts
the host; only the Windows daemon copy is skipped.
"""
from __future__ import annotations

import plistlib
import re

from pymobiledevice3 import usbmux
from pymobiledevice3.exceptions import MuxException
from pymobiledevice3.lockdown import PlistUsbmuxLockdownClient
from pymobiledevice3.remote.tunnel_service import PEER_ALT_IRK_KEY, RemotePairingProtocol


def _patch_remote_pairing_plist() -> None:
    _save = RemotePairingProtocol.save_pair_record

    def save_pair_record(self) -> None:  # type: ignore[no-untyped-def]
        _save(self)
        data = plistlib.loads(self.pair_record_path.read_bytes())
        if "identifier" not in data:
            data["identifier"] = self.identifier
            if data.get(PEER_ALT_IRK_KEY) and "alt_irk" not in data:
                data["alt_irk"] = data[PEER_ALT_IRK_KEY]
            self.pair_record_path.write_bytes(plistlib.dumps(data))

    RemotePairingProtocol.save_pair_record = save_pair_record  # type: ignore[method-assign]


def _patch_save_pair_record() -> None:
    base_save = PlistUsbmuxLockdownClient.__bases__[0].save_pair_record

    async def save_pair_record(self) -> None:  # type: ignore[no-untyped-def]
        await base_save(self)
        assert self.pair_record is not None and self.identifier is not None
        record_data = plistlib.dumps(self.pair_record)
        async with await usbmux.create_mux(usbmux_address=self.usbmux_address) as client:
            from pymobiledevice3.usbmux import PlistMuxConnection

            if isinstance(client, PlistMuxConnection) and self.service.mux_device is not None:
                try:
                    await client.save_pair_record(
                        self.identifier, self.service.mux_device.devid, record_data
                    )
                except MuxException as error:
                    if not re.search(r"Number['\"]?:\s*183", str(error)):
                        raise

    PlistUsbmuxLockdownClient.save_pair_record = save_pair_record  # type: ignore[method-assign]


def main() -> None:
    _patch_save_pair_record()
    _patch_remote_pairing_plist()
    from pymobiledevice3.__main__ import main as pmd3_main

    raise SystemExit(pmd3_main())


if __name__ == "__main__":
    main()