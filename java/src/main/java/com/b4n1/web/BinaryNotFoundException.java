package com.b4n1.web;

/**
 * Exception thrown when B4n1Web binary is not found.
 */
public class BinaryNotFoundException extends RuntimeException {
    public BinaryNotFoundException() {
        super("B4n1Web binary not found. Please install it first:\n  curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.7.0-flat.tar.gz | tar -xz");
    }

    public BinaryNotFoundException(String message) {
        super(message);
    }
}
