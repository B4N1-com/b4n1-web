package com.b4n1.web;

/**
 * Exception thrown when B4n1Web binary is not found.
 */
public class BinaryNotFoundException extends RuntimeException {
    public BinaryNotFoundException() {
        super("B4n1Web binary not found. Please install it first:\n  curl -sL https://web.b4n1.com/install | bash");
    }

    public BinaryNotFoundException(String message) {
        super(message);
    }
}
