package com.b4n1.web;

import org.junit.jupiter.api.Test;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Smoke tests for the custom exception classes.
 *
 * Verifies {@link BinaryNotFoundException} and {@link NavigationException}
 * message formatting, inheritance, and property access.
 */
class ErrorTest {

    // ── BinaryNotFoundException ───────────────────────────────────────

    @Test
    void binaryNotFoundExceptionShouldExtendRuntimeException() {
        assertInstanceOf(RuntimeException.class, new BinaryNotFoundException(),
                "BinaryNotFoundException should be a RuntimeException");
    }

    @Test
    void binaryNotFoundExceptionDefaultMessageShouldContainInstallInstructions() {
        BinaryNotFoundException ex = new BinaryNotFoundException();
        String msg = ex.getMessage();
        assertNotNull(msg, "Default message must not be null");
        assertTrue(msg.contains("B4n1Web binary not found"),
                "Default message must indicate binary not found");
        assertTrue(msg.contains("install.sh"),
                "Default message must include install instructions URL");
        assertTrue(msg.contains("curl"),
                "Default message must include curl command");
    }

    @Test
    void binaryNotFoundExceptionShouldAcceptCustomMessage() {
        String customMsg = "Custom error message";
        BinaryNotFoundException ex = new BinaryNotFoundException(customMsg);
        assertEquals(customMsg, ex.getMessage());
    }

    @Test
    void binaryNotFoundExceptionDefaultConstructorShouldSetMessage() {
        BinaryNotFoundException ex = new BinaryNotFoundException();
        assertNotNull(ex.getMessage());
        assertFalse(ex.getMessage().isEmpty());
    }

    // ── NavigationException ───────────────────────────────────────────

    @Test
    void navigationExceptionShouldExtendRuntimeException() {
        assertInstanceOf(RuntimeException.class, new NavigationException("http://test.com", "msg"),
                "NavigationException should be a RuntimeException");
    }

    @Test
    void navigationExceptionShouldStoreUrl() {
        NavigationException ex = new NavigationException("https://example.com", "Not found");
        assertEquals("https://example.com", ex.getUrl());
    }

    @Test
    void navigationExceptionMessageConstructorShouldPreserveMessage() {
        NavigationException ex = new NavigationException("https://test.com", "Timeout");
        assertEquals("Timeout", ex.getMessage());
    }

    @Test
    void navigationExceptionCauseConstructorShouldPreserveUrl() {
        Throwable cause = new RuntimeException("root cause");
        NavigationException ex = new NavigationException("https://broken.com", cause);

        assertEquals("https://broken.com", ex.getUrl());
        assertSame(cause, ex.getCause());
        assertNotNull(ex.getMessage(), "Message should not be null even when constructed with cause");
    }

    @Test
    void navigationExceptionShouldHandleNullUrl() {
        NavigationException ex = new NavigationException(null, "No URL provided");
        assertNull(ex.getUrl(), "getUrl() should return null when constructed with null");
    }

    @Test
    void navigationExceptionShouldHandleEmptyUrl() {
        NavigationException ex = new NavigationException("", "Empty URL");
        assertEquals("", ex.getUrl());
    }

    @Test
    void navigationExceptionCauseConstructorShouldChainMessage() {
        Throwable cause = new IOException("connection refused");
        NavigationException ex = new NavigationException("https://fail.com", cause);

        assertSame(cause, ex.getCause());
        String msg = ex.getMessage();
        assertNotNull(msg);
        assertTrue(msg.contains("connection refused") || msg.contains(cause.toString()),
                "Exception message should reference the cause");
    }
}
