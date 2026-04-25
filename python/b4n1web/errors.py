"""
B4n1Web SDK Exceptions
"""


class BinaryNotFoundError(RuntimeError):
    """Raised when B4n1Web binary is not found."""

    def __init__(self):
        super().__init__(
            "B4n1Web binary not found. Please install it first:\n"
            "  curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz"
        )
