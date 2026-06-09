import os
from pathlib import Path
from unittest.mock import patch
from b4n1web.__main__ import resolve_install_dir

def test_resolve_install_dir_from_env():
    with patch.dict(os.environ, {"B4N1WEB_INSTALL_DIR": "/tmp/custom/bin"}):
        assert resolve_install_dir() == Path("/tmp/custom/bin")

def test_resolve_install_dir_fallback_to_user_local():
    # Mock /usr/local/bin as not writable and home/.local/bin as writable
    with patch.dict(os.environ, {}, clear=True):
        with patch("os.access", side_effect=lambda p, m: p == str(Path.home() / ".local/bin")):
            assert resolve_install_dir() == Path.home() / ".local/bin"

def test_resolve_install_dir_final_fallback():
    # Mock everything as not writable
    with patch.dict(os.environ, {}, clear=True):
        with patch("os.access", return_value=False):
            assert resolve_install_dir() == Path.home() / ".b4n1web" / "bin"
