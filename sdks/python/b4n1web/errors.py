"""
B4n1Web SDK Exceptions
"""


class BinaryNotFoundError(RuntimeError):
    """Raised when B4n1Web binary is not found."""

    def __init__(self):
        super().__init__(
            "B4n1Web binary not found. Please install it first:\n"
            "  curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash"
        )
