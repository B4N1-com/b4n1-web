"""
B4n1Web CLI Entry Point
"""

import os
import sys
import tempfile
from pathlib import Path


def install_command():
    """Handle the 'python -m b4n1web install' command."""
    from .browser import get_b4n1web_binary, get_b4n1web_version

    existing = get_b4n1web_binary()
    if existing:
        version = get_b4n1web_version()
        print(f"B4n1Web already installed at: {existing}")
        print(f"Version: {version}")
        response = input("Reinstall? (y/N): ").lower().strip()
        if response not in ("y", "yes"):
            print("Installation cancelled.")
            return

    env_install_dir = os.getenv("B4N1WEB_INSTALL_DIR")
    if env_install_dir:
        install_dir = Path(env_install_dir)
        use_sudo = False
    elif os.access("/usr/local/bin", os.W_OK):
        install_dir = Path("/usr/local/bin")
        use_sudo = False
    elif os.access(str(Path.home() / ".local/bin"), os.W_OK):
        install_dir = Path.home() / ".local/bin"
        use_sudo = False
    else:
        install_dir = Path.home() / ".b4n1web" / "bin"
        use_sudo = False

    install_dir.mkdir(parents=True, exist_ok=True)
    binary_path = install_dir / "b4n1web"

    version_url = "https://web.b4n1.com/latest-version"
    print(f"\nChecking latest version...")

    try:
        import requests

        resp = requests.get(version_url, timeout=10)
        version_info = resp.json()
        download_url = version_info.get("url")
        version = version_info.get("version", "unknown")
        print(f"Latest version: {version}")
        print(f"Downloading from: {download_url}")
    except Exception as e:
        print(f"Error getting version info: {e}")
        sys.exit(1)

    print(f"\nDownloading B4n1Web {version}...")
    try:
        resp = requests.get(download_url, timeout=60)
        resp.raise_for_status()

        import tarfile

        with tempfile.NamedTemporaryFile(suffix=".tar.gz", delete=False) as tmp_tar:
            tmp_tar.write(resp.content)
            tmp_tar_path = tmp_tar.name

        try:
            with tarfile.open(tmp_tar_path, "r:gz") as tar:
                for member in tar.getmembers():
                    if member.isfile():
                        binary_data = tar.extractfile(member)
                        if binary_data:
                            with open(binary_path, "wb") as f:
                                f.write(binary_data.read())
                            break

            if use_sudo:
                import subprocess

                subprocess.run(["chmod", "+x", str(binary_path)], check=True)
                if str(binary_path).startswith("/usr/local/bin") or str(
                    binary_path
                ).startswith("/usr/bin"):
                    subprocess.run(
                        ["sudo", "chmod", "+x", str(binary_path)], check=True
                    )
            else:
                os.chmod(binary_path, 0o755)

            installed_version = get_b4n1web_version()
            print(f"B4n1Web {installed_version} installed to: {binary_path}")

            if not use_sudo:
                rc_file = Path.home() / (
                    ".bashrc"
                    if os.environ.get("SHELL", "").endswith("bash")
                    else ".zshrc"
                )
                path_line = f'export PATH="{install_dir}:$PATH"'
                if rc_file.exists():
                    content = rc_file.read_text()
                    if path_line not in content:
                        print(f"\nTo use B4n1Web, add this to your {rc_file.name}:")
                        print(f"  {path_line}")
                else:
                    print(
                        f"\nTo use B4n1Web, add this to your shell rc file (~/.bashrc or ~/.zshrc):"
                    )
                    print(f"  {path_line}")

        finally:
            try:
                os.unlink(tmp_tar_path)
            except:
                pass

    except Exception as e:
        print(f"Error installing B4n1Web: {e}")
        sys.exit(1)


if __name__ == "__main__":
    install_command()
