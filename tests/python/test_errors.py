"""Unit tests for BinaryNotFoundError exception."""

import pytest
from b4n1web.errors import BinaryNotFoundError


class TestBinaryNotFoundError:
    def test_error_message(self):
        error = BinaryNotFoundError()
        message = str(error)
        assert "B4n1Web binary not found" in message
        assert "curl -sL https://web.b4n1.com/install" in message

    def test_is_runtime_error(self):
        error = BinaryNotFoundError()
        assert isinstance(error, RuntimeError)

    def test_can_be_raised(self):
        with pytest.raises(BinaryNotFoundError):
            raise BinaryNotFoundError()

    def test_can_be_caught_as_runtime_error(self):
        with pytest.raises(RuntimeError):
            raise BinaryNotFoundError()
